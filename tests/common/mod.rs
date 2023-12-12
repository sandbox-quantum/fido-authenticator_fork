use ctap_types::{Bytes, String};

pub fn random_bytes<const N: usize>() -> Bytes<N> {
    use rand::{
        distributions::{Distribution, Uniform},
        rngs::OsRng,
        RngCore,
    };
    let mut bytes = Bytes::default();

    let between = Uniform::from(0..(N + 1));
    let n = between.sample(&mut OsRng);

    bytes.resize_default(n).unwrap();

    OsRng.fill_bytes(&mut bytes);
    bytes
}

#[allow(dead_code)]
fn maybe_random_bytes<const N: usize>() -> Option<Bytes<N>> {
    use rand::{rngs::OsRng, RngCore};
    if OsRng.next_u32() & 1 != 0 {
        Some(random_bytes())
    } else {
        None
    }
}

pub fn random_string<const N: usize>() -> String<N> {
    use rand::{
        distributions::{Alphanumeric, Distribution, Uniform},
        rngs::OsRng,
        Rng,
    };
    use std::str::FromStr;

    let between = Uniform::from(0..(N + 1));
    let n = between.sample(&mut OsRng);

    let std_string: std::string::String = OsRng
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect();
    String::from_str(&std_string).unwrap()
}

pub fn maybe_random_string<const N: usize>() -> Option<String<N>> {
    use rand::{rngs::OsRng, RngCore};
    if OsRng.next_u32() & 1 != 0 {
        Some(random_string())
    } else {
        None
    }
}
