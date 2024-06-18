use rand::Rng;
use std::{cmp, fmt};

pub const ALPHABETS: [&'static str; 2] = ["bcdfglmnprstvz", "aeiou"];

#[derive(Debug)]
pub enum BananaError {
    InvalidBanana,
    InvalidAlphabet,
}

impl fmt::Display for BananaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BananaError::InvalidBanana => write!(f, "Invalid banana number"),
            BananaError::InvalidAlphabet => write!(f, "Invalid alphabet"),
        }
    }
}

#[derive(Default, Debug)]
pub struct EncodeParams {
    pub alphabet_shift: Option<usize>,
    pub alphabet_end: Option<usize>,
    pub min_length: Option<usize>,
    pub alphabets: Option<Vec<String>>,
}

fn get_alphabets(alphabets: Option<&[String]>) -> Vec<Vec<char>> {
    alphabets
        .as_deref()
        .map(|a| a.iter().map(|s| &**s).collect::<Vec<&str>>())
        .unwrap_or(ALPHABETS.to_vec())
        .iter()
        .map(|alphabet| alphabet.chars().collect())
        .collect()
}

pub fn encode(num: u64, params: &EncodeParams) -> Result<String, BananaError> {
    let alphabets = get_alphabets(params.alphabets.as_deref());

    let n_alphabets = alphabets.len();
    let alphabet_shift = params.alphabet_shift.unwrap_or(0) % n_alphabets;
    let alphabet_end = params.alphabet_end.unwrap_or(0) % n_alphabets;
    let min_length = cmp::max(1, params.min_length.unwrap_or(1));

    let mut v = num;
    let mut word = Vec::new();

    let mut idx = (n_alphabets - 1 + alphabet_shift + alphabet_end) % n_alphabets;
    while v != 0
        || (idx != (n_alphabets - 1 + alphabet_shift) % n_alphabets)
        || word.len() < min_length
    {
        let alphabet_len = alphabets[idx].len() as u64;
        if alphabet_len == 0 {
            return Err(BananaError::InvalidAlphabet);
        }

        let r: usize = usize::try_from(v % alphabet_len).unwrap();
        v = v / alphabet_len;

        word.push(alphabets[idx][r]);
        idx = (idx + n_alphabets - 1) % n_alphabets;
    }

    word.reverse();
    Ok(word.iter().collect())
}

pub fn random(params: &EncodeParams) -> Result<String, BananaError> {
    let alphabets = get_alphabets(params.alphabets.as_deref());

    let n_alphabets = alphabets.len();

    let alphabet_shift = params.alphabet_shift.unwrap_or(0) % n_alphabets;
    let alphabet_end = params.alphabet_end.unwrap_or(0);
    let min_length = cmp::max(1, params.min_length.unwrap_or(1));

    if min_length == 0 {
        return Ok("".to_string());
    }

    let mut word = Vec::new();
    let mut rng = rand::thread_rng();

    let mut idx = (n_alphabets - 1 + alphabet_shift + alphabet_end) % n_alphabets;

    while idx != (n_alphabets - 1 + alphabet_shift) % n_alphabets || word.len() < min_length {
        let alphabet_len = alphabets[idx].len();
        if alphabet_len == 0 {
            return Err(BananaError::InvalidAlphabet);
        }
        let r = rng.gen_range(0..alphabet_len);
        word.push(alphabets[idx][r]);
        idx = (idx + n_alphabets - 1) % n_alphabets;
    }

    word.reverse();
    Ok(word.iter().collect())
}

#[derive(Default, Debug)]
pub struct DecodeParams {
    pub alphabet_shift: Option<usize>,
    pub alphabet_end: Option<usize>,
    pub alphabets: Option<Vec<String>>,
}

pub fn decode(word: &str, params: &DecodeParams) -> Result<u64, BananaError> {
    let alphabets = get_alphabets(params.alphabets.as_deref());

    let n_alphabets = alphabets.len();

    let alphabet_shift = params.alphabet_shift.unwrap_or(0) % n_alphabets;
    let alphabet_end = params.alphabet_end.unwrap_or(0) % n_alphabets;

    if (word.chars().count() - alphabet_end) % n_alphabets != 0 {
        return Err(BananaError::InvalidBanana);
    }

    let mut v: u64 = 0;
    for (i, c) in word.chars().enumerate() {
        let idx = (n_alphabets + i + alphabet_shift) % n_alphabets;
        let alphabet = &alphabets[idx];
        match alphabet.iter().position(|&x| x == c) {
            Some(pos) => v = v * alphabet.len() as u64 + pos as u64,
            None => return Err(BananaError::InvalidBanana),
        };
    }

    Ok(v)
}

pub fn is_valid(word: &str, params: &DecodeParams) -> bool {
    let alphabets = get_alphabets(params.alphabets.as_deref());

    let n_alphabets = alphabets.len();

    let alphabet_shift = params.alphabet_shift.unwrap_or(0) % n_alphabets;
    let alphabet_end = params.alphabet_end.unwrap_or(0) % n_alphabets;

    if (word.chars().count() - alphabet_end) % n_alphabets != 0 {
        return false;
    }

    for (i, c) in word.chars().enumerate() {
        let idx = (n_alphabets + i + alphabet_shift) % n_alphabets;
        let alphabet = &alphabets[idx];
        if alphabet.iter().position(|&x| x == c).is_none() {
            return false;
        }
    }

    return true;
}

#[macro_export]
macro_rules! encode {
    ($a: expr) => {
        encode($a, &EncodeParams::default())
    };
}

#[macro_export]
macro_rules! decode {
    ($a: expr) => {
        decode($a, &DecodeParams::default())
    };
}

#[macro_export]
macro_rules! random {
    () => {
        random(&EncodeParams::default())
    };
}

#[macro_export]
macro_rules! is_valid {
    ($a: expr) => {
        is_valid($a, &DecodeParams::default())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        for test in [
            (1, "be"),
            (1000, "duga"),
            (100000000000, "tumuzadofopa"),
            (u64::MAX, "cenicovutinazamofevafa"),
        ] {
            assert_eq!(encode!(test.0).unwrap(), test.1);
            assert_eq!(decode!(test.1).unwrap(), test.0);
            assert_eq!(is_valid!(&encode!(test.0).unwrap()), true);
        }

        assert_eq!(is_valid!("123"), false);
        assert!(random!().unwrap().len() > 0)
    }

    #[test]
    fn test_alphabet_shift() {
        for test in [
            (1, "be", 0),
            (1, "ac", 1),
            (1000, "ebem", 3),
            (1, "ac", usize::MAX),
        ] {
            assert_eq!(
                encode(
                    test.0,
                    &EncodeParams {
                        alphabet_shift: Some(test.2),
                        ..Default::default()
                    },
                )
                .unwrap(),
                test.1
            );

            assert_eq!(
                decode(
                    test.1,
                    &DecodeParams {
                        alphabet_shift: Some(test.2),
                        ..Default::default()
                    },
                )
                .unwrap(),
                test.0
            );
        }
    }

    #[test]
    fn test_alphabet_end() {
        for test in [
            (1, "be", 0),
            (1, "c", 1),
            (1000, "bebem", 3),
            (1, "c", usize::MAX),
        ] {
            assert_eq!(
                encode(
                    test.0,
                    &EncodeParams {
                        alphabet_end: Some(test.2),
                        ..Default::default()
                    },
                )
                .unwrap(),
                test.1
            );

            assert_eq!(
                decode(
                    test.1,
                    &DecodeParams {
                        alphabet_end: Some(test.2),
                        ..Default::default()
                    },
                )
                .unwrap(),
                test.0
            );
        }
    }

    #[test]
    fn test_min_length() {
        for test in [
            (1, "be", 0),
            (1, "be", 1),
            (1, "bababababe", 10),
            (1000, "bababaduga", 10),
        ] {
            assert_eq!(
                encode(
                    test.0,
                    &EncodeParams {
                        min_length: Some(test.2),
                        ..Default::default()
                    },
                )
                .unwrap(),
                test.1
            );

            assert_eq!(decode!(test.1).unwrap(), test.0);
        }
    }

    #[test]
    fn test_alphabets() {
        for test in [
            (1, "aw", vec!["abc".to_string(), "qwe".to_string()]),
            (8, "ce", vec!["abc".to_string(), "qwe".to_string()]),
            (9, "awaq", vec!["abc".to_string(), "qwe".to_string()]),
            (
                9,
                "bq1",
                vec!["abc".to_string(), "qwe".to_string(), "123".to_string()],
            ),
            (
                27,
                "aq2aq1",
                vec!["abc".to_string(), "qwe".to_string(), "123".to_string()],
            ),
            (
                0,
                "🐼🐶🐱",
                vec![
                    "🐼🐵🦍".to_string(),
                    "🐶🐺🦊".to_string(),
                    "🐱🦁🐯".to_string(),
                ],
            ),
            (
                27,
                "🐼🐶🦁🐼🐶🐱",
                vec![
                    "🐼🐵🦍".to_string(),
                    "🐶🐺🦊".to_string(),
                    "🐱🦁🐯".to_string(),
                ],
            ),
        ] {
            assert_eq!(
                encode(
                    test.0,
                    &EncodeParams {
                        alphabets: Some(test.2.clone()),
                        ..Default::default()
                    },
                )
                .unwrap(),
                test.1
            );

            assert_eq!(
                decode(
                    test.1,
                    &DecodeParams {
                        alphabets: Some(test.2.clone()),
                        ..Default::default()
                    }
                )
                .unwrap(),
                test.0
            );
        }
    }
}
