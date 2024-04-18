// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/binary.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, Instance, Export};
use super::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 1
fn create_module_1() -> ResultObject {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 2

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    // We group the calls together
    start_module_1(&result_object);
}
fn create_module_2() -> ResultObject {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_2(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 3

#[test]
fn test_module_2() {
    let result_object = create_module_2();
    // We group the calls together
    start_module_2(&result_object);
}
fn create_module_3() -> ResultObject {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_3(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 4

#[test]
fn test_module_3() {
    let result_object = create_module_3();
    // We group the calls together
    start_module_3(&result_object);
}
fn create_module_4() -> ResultObject {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_4(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 6
#[test]
fn c4_l6_assert_malformed() {
    let wasm_binary = [];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 7
#[test]
fn c5_l7_assert_malformed() {
    let wasm_binary = [1];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 8
#[test]
fn c6_l8_assert_malformed() {
    let wasm_binary = [0, 97, 115];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 9
#[test]
fn c7_l9_assert_malformed() {
    let wasm_binary = [97, 115, 109, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 10
#[test]
fn c8_l10_assert_malformed() {
    let wasm_binary = [109, 115, 97, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 11
#[test]
fn c9_l11_assert_malformed() {
    let wasm_binary = [109, 115, 97, 0, 1, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 12
#[test]
fn c10_l12_assert_malformed() {
    let wasm_binary = [109, 115, 97, 0, 0, 0, 0, 1];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 13
#[test]
fn c11_l13_assert_malformed() {
    let wasm_binary = [97, 115, 109, 1, 0, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 14
#[test]
fn c12_l14_assert_malformed() {
    let wasm_binary = [119, 97, 115, 109, 1, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 15
#[test]
fn c13_l15_assert_malformed() {
    let wasm_binary = [127, 97, 115, 109, 1, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 16
#[test]
fn c14_l16_assert_malformed() {
    let wasm_binary = [128, 97, 115, 109, 1, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 17
#[test]
fn c15_l17_assert_malformed() {
    let wasm_binary = [130, 97, 115, 109, 1, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 18
#[test]
fn c16_l18_assert_malformed() {
    let wasm_binary = [255, 97, 115, 109, 1, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 21
#[test]
fn c17_l21_assert_malformed() {
    let wasm_binary = [0, 0, 0, 1, 109, 115, 97, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 24
#[test]
fn c18_l24_assert_malformed() {
    let wasm_binary = [97, 0, 109, 115, 0, 1, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 25
#[test]
fn c19_l25_assert_malformed() {
    let wasm_binary = [115, 109, 0, 97, 0, 0, 1, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 28
#[test]
fn c20_l28_assert_malformed() {
    let wasm_binary = [0, 65, 83, 77, 1, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 31
#[test]
fn c21_l31_assert_malformed() {
    let wasm_binary = [0, 129, 162, 148, 1, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 34
#[test]
fn c22_l34_assert_malformed() {
    let wasm_binary = [239, 187, 191, 0, 97, 115, 109, 1, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 36
#[test]
fn c23_l36_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 37
#[test]
fn c24_l37_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 38
#[test]
fn c25_l38_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 39
#[test]
fn c26_l39_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 0, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 40
#[test]
fn c27_l40_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 13, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 41
#[test]
fn c28_l41_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 14, 0, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 42
#[test]
fn c29_l42_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 0, 1, 0, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 43
#[test]
fn c30_l43_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 0, 0, 1, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 44
#[test]
fn c31_l44_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 0, 0, 0, 1];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 47

#[test]
fn test_module_4() {
    let result_object = create_module_4();
    // We group the calls together
    start_module_4(&result_object);
}
fn create_module_5() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 2))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_5(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 52

#[test]
fn test_module_5() {
    let result_object = create_module_5();
    // We group the calls together
    start_module_5(&result_object);
}
fn create_module_6() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 2))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_6(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 59

#[test]
fn test_module_6() {
    let result_object = create_module_6();
    // We group the calls together
    start_module_6(&result_object);
}
fn create_module_7() -> ResultObject {
    let module_str = "(module
      (global (;0;) i32 (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_7(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 66

#[test]
fn test_module_7() {
    let result_object = create_module_7();
    // We group the calls together
    start_module_7(&result_object);
}
fn create_module_8() -> ResultObject {
    let module_str = "(module
      (global (;0;) i32 (i32.const -1)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_8(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 73

#[test]
fn test_module_8() {
    let result_object = create_module_8();
    // We group the calls together
    start_module_8(&result_object);
}
fn create_module_9() -> ResultObject {
    let module_str = "(module
      (global (;0;) i32 (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_9(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 80

#[test]
fn test_module_9() {
    let result_object = create_module_9();
    // We group the calls together
    start_module_9(&result_object);
}
fn create_module_10() -> ResultObject {
    let module_str = "(module
      (global (;0;) i32 (i32.const -1)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_10(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 88

#[test]
fn test_module_10() {
    let result_object = create_module_10();
    // We group the calls together
    start_module_10(&result_object);
}
fn create_module_11() -> ResultObject {
    let module_str = "(module
      (global (;0;) i64 (i64.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_11(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 95

#[test]
fn test_module_11() {
    let result_object = create_module_11();
    // We group the calls together
    start_module_11(&result_object);
}
fn create_module_12() -> ResultObject {
    let module_str = "(module
      (global (;0;) i64 (i64.const -1)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_12(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 102

#[test]
fn test_module_12() {
    let result_object = create_module_12();
    // We group the calls together
    start_module_12(&result_object);
}
fn create_module_13() -> ResultObject {
    let module_str = "(module
      (global (;0;) i64 (i64.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_13(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 109

#[test]
fn test_module_13() {
    let result_object = create_module_13();
    // We group the calls together
    start_module_13(&result_object);
}
fn create_module_14() -> ResultObject {
    let module_str = "(module
      (global (;0;) i64 (i64.const -1)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_14(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 118

#[test]
fn test_module_14() {
    let result_object = create_module_14();
    // We group the calls together
    start_module_14(&result_object);
}
fn create_module_15() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 0)
      (data (;0;) (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_15(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 128

#[test]
fn test_module_15() {
    let result_object = create_module_15();
    // We group the calls together
    start_module_15(&result_object);
}
fn create_module_16() -> ResultObject {
    let module_str = "(module
      (table (;0;) 0 anyfunc)
      (elem (;0;) (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_16(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 139
#[test]
fn c44_l139_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 8, 1, 0, 130, 128, 128, 128, 128, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 149
#[test]
fn c45_l149_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 11, 1, 127, 0, 65, 128, 128, 128, 128, 128, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 159
#[test]
fn c46_l159_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 11, 1, 127, 0, 65, 255, 255, 255, 255, 255, 127, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 170
#[test]
fn c47_l170_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 16, 1, 126, 0, 66, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 180
#[test]
fn c48_l180_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 16, 1, 126, 0, 66, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 192
#[test]
fn c49_l192_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 7, 1, 0, 130, 128, 128, 128, 112];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 200
#[test]
fn c50_l200_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 7, 1, 0, 130, 128, 128, 128, 64];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 210
#[test]
fn c51_l210_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 10, 1, 127, 0, 65, 128, 128, 128, 128, 112, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 220
#[test]
fn c52_l220_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 10, 1, 127, 0, 65, 255, 255, 255, 255, 15, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 230
#[test]
fn c53_l230_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 10, 1, 127, 0, 65, 128, 128, 128, 128, 31, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 240
#[test]
fn c54_l240_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 10, 1, 127, 0, 65, 255, 255, 255, 255, 79, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 251
#[test]
fn c55_l251_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 15, 1, 126, 0, 66, 128, 128, 128, 128, 128, 128, 128, 128, 128, 126, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 261
#[test]
fn c56_l261_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 15, 1, 126, 0, 66, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 271
#[test]
fn c57_l271_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 15, 1, 126, 0, 66, 128, 128, 128, 128, 128, 128, 128, 128, 128, 2, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 281
#[test]
fn c58_l281_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 15, 1, 126, 0, 66, 255, 255, 255, 255, 255, 255, 255, 255, 255, 65, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 293
#[test]
fn c59_l293_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 4, 4, 1, 112, 0, 0, 10, 9, 1, 7, 0, 65, 0, 17, 0, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 312
#[test]
fn c60_l312_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 4, 4, 1, 112, 0, 0, 10, 10, 1, 7, 0, 65, 0, 17, 0, 128, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 331
#[test]
fn c61_l331_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 4, 4, 1, 112, 0, 0, 10, 11, 1, 8, 0, 65, 0, 17, 0, 128, 128, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 349
#[test]
fn c62_l349_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 4, 4, 1, 112, 0, 0, 10, 12, 1, 9, 0, 65, 0, 17, 0, 128, 128, 128, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 367
#[test]
fn c63_l367_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 4, 4, 1, 112, 0, 0, 10, 13, 1, 10, 0, 65, 0, 17, 0, 128, 128, 128, 128, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 386
#[test]
fn c64_l386_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 9, 1, 7, 0, 65, 0, 64, 1, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 406
#[test]
fn c65_l406_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 10, 1, 8, 0, 65, 0, 64, 128, 0, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 426
#[test]
fn c66_l426_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 11, 1, 9, 0, 65, 0, 64, 128, 128, 0, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 445
#[test]
fn c67_l445_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 12, 1, 10, 0, 65, 0, 64, 128, 128, 128, 0, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 464
#[test]
fn c68_l464_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 13, 1, 11, 0, 65, 0, 64, 128, 128, 128, 128, 0, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 484
#[test]
fn c69_l484_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 7, 1, 5, 0, 63, 1, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 503
#[test]
fn c70_l503_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 8, 1, 6, 0, 63, 128, 0, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 522
#[test]
fn c71_l522_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 9, 1, 7, 0, 63, 128, 128, 0, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 540
#[test]
fn c72_l540_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 10, 1, 8, 0, 63, 128, 128, 128, 0, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 558
#[test]
fn c73_l558_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 10, 11, 1, 9, 0, 63, 128, 128, 128, 128, 0, 26, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 577
#[test]
fn c74_l577_assert_malformed() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 12, 1, 10, 2, 255, 255, 255, 255, 15, 127, 2, 126, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

#[test]
fn test_module_16() {
    let result_object = create_module_16();
    // We group the calls together
    start_module_16(&result_object);
}
