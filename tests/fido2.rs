mod common;

use cbor_smol::cbor_deserialize;
use ctap_types::ctap2::make_credential::{AttestedCredentialData, AuthenticatorData};
use ctap_types::ctap2::{AuthenticatorDataFlags, AuthenticatorOptions};
use ctap_types::sizes::{COSE_KEY_LENGTH, MAX_CREDENTIAL_ID_LENGTH};
use ctap_types::{Bytes, String};
use fido_authenticator::Authenticator;
use serde::Deserialize;
use serde_cbor;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, ErrorKind, Read, Seek};

// deserialization should be implemented in ctap-types
fn deserialize_authenticator_data(data: &[u8]) -> Result<AuthenticatorData, std::io::Error> {
    let mut cursor = Cursor::new(data);
    let mut rp_id_hash = [0u8; 32];
    let flags;
    let sign_count;

    cursor.read_exact(&mut rp_id_hash)?;
    flags = cursor.read_u8()?;
    sign_count = cursor.read_u32::<BigEndian>()?;

    let flags = AuthenticatorDataFlags::from_bits(flags)
        .ok_or_else(|| std::io::Error::from(ErrorKind::InvalidData))?;

    let attested_credential_data =
        if flags.contains(AuthenticatorDataFlags::ATTESTED_CREDENTIAL_DATA) {
            let mut aaguid = [0u8; 16];
            let credential_id_len;

            cursor.read_exact(&mut aaguid)?;
            credential_id_len = cursor.read_u16::<BigEndian>()? as usize;
            if credential_id_len > MAX_CREDENTIAL_ID_LENGTH {
                return Err(std::io::Error::from(ErrorKind::InvalidData));
            }
            let mut credential_id: Vec<u8> = vec![0; credential_id_len];
            cursor.read_exact(&mut credential_id)?;

            // serde_cbor will fail if remaining data (extensions) exists after credential_public_key.
            // Use serde to figure out credential_public_key's size.
            let mut data: Vec<u8> = vec![];
            let position = cursor.stream_position().unwrap();
            let _ = cursor.read_to_end(&mut data)?;
            let mut deserializer = serde_cbor::de::Deserializer::from_slice(&data);
            let _value = serde_cbor::value::Value::deserialize(&mut deserializer)
                .map_err(|_| std::io::Error::from(ErrorKind::InvalidData))?;
            let credential_public_key_len = deserializer.byte_offset() as usize;
            if credential_public_key_len > COSE_KEY_LENGTH {
                return Err(std::io::Error::from(ErrorKind::InvalidData));
            }
            cursor.seek(std::io::SeekFrom::Start(position)).unwrap();

            let mut credential_public_key: Vec<u8> = vec![0; credential_public_key_len];
            cursor.read_exact(&mut credential_public_key)?;

            Some(AttestedCredentialData {
                aaguid: Bytes::from_slice(&aaguid).unwrap(),
                credential_id: Bytes::from_slice(&credential_id).unwrap(),
                credential_public_key: Bytes::from_slice(&credential_public_key).unwrap(),
            })
        } else {
            None
        };

    let extensions = if flags.contains(AuthenticatorDataFlags::EXTENSION_DATA) {
        let mut extensions_buf = Vec::with_capacity(128);
        let _extensions_buf_len = cursor.read_to_end(&mut extensions_buf)?;
        let extensions = cbor_deserialize(&extensions_buf)
            .map_err(|_| std::io::Error::from(ErrorKind::UnexpectedEof))?;
        Some(extensions)
    } else {
        None
    };

    Ok(AuthenticatorData {
        rp_id_hash: Bytes::from_slice(&rp_id_hash).unwrap(),
        flags,
        sign_count,
        attested_credential_data,
        extensions: extensions,
    })
}

fn register_and_auth(alg: i32) {
    use ctap_types::authenticator::Ctap2Authenticator;
    use ctap_types::ctap2::{get_assertion, make_credential};
    use ctap_types::webauthn::{
        FilteredPublicKeyCredentialParameters, KnownPublicKeyCredentialParameters,
        PublicKeyCredentialDescriptorRef, PublicKeyCredentialRpEntity,
        PublicKeyCredentialUserEntity, COUNT_KNOWN_ALGS,
    };
    use fido_authenticator::{Config, Silent};
    use std::str::FromStr;

    let rp_name = "Duo Security";
    let user_name = "lee@webauthn.guide";
    let rp_id = "duosecurity.com";
    let user_id = [0u8; 16];

    let rp = PublicKeyCredentialRpEntity {
        id: String::from_str(&rp_id).unwrap(),
        name: Some(rp_name.into()),
        icon: None,
    };
    let user = PublicKeyCredentialUserEntity {
        id: Bytes::from_slice(&user_id).unwrap(),
        icon: common::maybe_random_string(),
        name: Some(user_name.into()),
        display_name: common::maybe_random_string(),
    };

    let client_data_hash = rand::random::<[u8; 32]>();
    let chall_bytes: &serde_bytes::Bytes = serde_bytes::Bytes::new(&client_data_hash);

    trussed::virt::with_ram_client("test", |client| {
        let mut app = Authenticator::new(
            client,
            Silent {},
            Config {
                max_msg_size: 7069, //usbd_ctaphid::constants::MESSAGE_SIZE,
                // max_creds_in_list: ctap_types::sizes::MAX_CREDENTIAL_COUNT_IN_LIST,
                // max_cred_id_length: ctap_types::sizes::MAX_CREDENTIAL_ID_LENGTH,
                skip_up_timeout: None,
                max_resident_credential_count: None,
            },
        );

        let mut algs = heapless::Vec::<KnownPublicKeyCredentialParameters, COUNT_KNOWN_ALGS>::new();
        algs.extend((0..COUNT_KNOWN_ALGS).map(|_| KnownPublicKeyCredentialParameters { alg: alg }));

        let request = make_credential::Request {
            client_data_hash: &chall_bytes,
            rp: rp,
            user: user,
            pub_key_cred_params: FilteredPublicKeyCredentialParameters(algs),
            exclude_list: None,
            extensions: None,
            options: Some(AuthenticatorOptions {
                rk: Some(true),
                up: None,
                uv: None,
            }),
            pin_auth: None,
            pin_protocol: None,
        };
        let attestation_object = app.make_credential(&request).unwrap();
        println!("Register result: {:?}", &attestation_object);
        let auth_data: AuthenticatorData =
            deserialize_authenticator_data(&attestation_object.auth_data).unwrap();

        let attested_credential_data = auth_data.attested_credential_data.unwrap();
        let credential_id = attested_credential_data.credential_id.into_vec();
        let id = PublicKeyCredentialDescriptorRef {
            id: serde_bytes::Bytes::new(&credential_id),
            key_type: "usb",
        };

        let mut allow_list: heapless::Vec<PublicKeyCredentialDescriptorRef, 10> =
            heapless::Vec::new();
        allow_list.push(id).unwrap();

        let request = get_assertion::Request {
            rp_id: rp_id.into(),
            client_data_hash: Bytes::from_slice(&client_data_hash).unwrap(),
            allow_list: Some(allow_list),
            extensions: None,
            options: None,
            pin_auth: None,
            pin_protocol: None,
        };
        let object = app.get_assertion(&request).unwrap();
        println!("Sign result: {:?}", &object);
    });
}

#[test]
fn register_and_auth_all_algs() -> () {
    use ctap_types::webauthn::{KNOWN_ALGS, KYBER768};
    // KYBER768 algorithm isn't supported for now
    KNOWN_ALGS
        .iter()
        .filter(|alg| *alg != &KYBER768)
        .for_each(|alg| register_and_auth(*alg));
}
