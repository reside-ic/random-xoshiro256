//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use random_xoshiro256::RandomStateXoshiro256Plus;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_create() {
    let mut generator = RandomStateXoshiro256Plus::new(42);
    let x = generator.random();
    assert!(generator.random() != x);
    generator.seed(42);
    assert_eq!(generator.random(), x);

    let mut generator2 = generator.clone();
    assert_eq!(generator2.random(), generator.random());

    generator2.jump();
    assert!(generator2.random() != generator.random());

    generator.jump();
    assert_eq!(generator2.random(), generator.random());

    generator.long_jump();
    assert!(generator2.random() != generator.random());
}
