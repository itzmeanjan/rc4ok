#![cfg(test)]

use crate::RC4ok;
use rand::{thread_rng, RngCore};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
fn test_known_answer_tests() {
    const FILE: &str = "./rc4ok.kat";

    let fd = File::open(FILE).unwrap();
    let mut lines = BufReader::new(fd).lines();

    while let Some(line) = lines.next() {
        let key = hex::decode(line.unwrap().split(" = ").collect::<Vec<&str>>()[1]).unwrap();
        let prbytes = hex::decode(
            lines
                .next()
                .unwrap()
                .unwrap()
                .split(" = ")
                .collect::<Vec<&str>>()[1],
        )
        .unwrap();

        let mut computed_prbytes = vec![0u8; prbytes.len()];

        let mut rng = RC4ok::init(&key);
        rng.generate(&mut computed_prbytes);

        assert_eq!(prbytes, computed_prbytes);

        lines.next(); // empty line, at end of each test case
    }
}

#[test]
#[should_panic(expected = "Key must be non-empty !")]
fn test_failure_with_empty_key() {
    let mut rng = RC4ok::init(&[]);
    rng.generate(&mut []);
}

#[test]
fn test_success_with_non_empty_key() {
    const MIN_KEY_LEN: usize = 1;
    const MAX_KEY_LEN: usize = 256;

    let mut rng = thread_rng();

    let mut klen = MIN_KEY_LEN;
    while klen <= MAX_KEY_LEN {
        let mut key = vec![0u8; klen];
        rng.fill_bytes(&mut key);

        let mut prng = RC4ok::init(&key);
        prng.generate(&mut []);

        klen += 1;
    }
}

#[test]
fn test_success_with_state_reset() {
    const MIN_KEY_LEN: usize = 1;
    const MAX_KEY_LEN: usize = 256;

    let mut rng = thread_rng();

    let mut klen = MIN_KEY_LEN;
    while klen <= MAX_KEY_LEN {
        let mut key = vec![0u8; klen];
        rng.fill_bytes(&mut key);

        let mut out0 = vec![0u8; 1024]; // random bytes generated after first init
        let mut out1 = vec![0u8; 1024]; // random bytes generated after reinit

        let mut prng = RC4ok::init(&key);
        prng.generate(&mut out0);
        prng.reset(&key);
        prng.generate(&mut out1);

        assert_eq!(out0, out1);

        klen += 1;
    }
}
