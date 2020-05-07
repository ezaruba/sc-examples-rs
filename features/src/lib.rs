
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

use ::serde::{Serialize, Deserialize};

/// Serialization example
#[derive(Serialize, Deserialize)]
pub struct SerExample1 {
    q1: u8,
    q2: i32
}

#[elrond_wasm_derive::contract(ApiFeatureExamplesImpl)]
pub trait ApiFeatureExamples {

    fn init(&self) {
    }

    fn panicWithMessage(&self) {
        panic!("example panic message");
    }

    // TEST ARGUMENT AND RETURN TYPE SERIALIZATION

    fn echo_big_uint(&self, bi: BigUint) -> BigUint {
        bi
    }

    fn echo_big_int(&self, bi: BigInt) -> BigInt {
        bi
    }

    fn echo_i64(&self, i: i64) -> i64 {
        i
    }

    fn echo_i32(&self, i: i32) -> i32 {
        i
    }

    fn echo_u32(&self, i: u32) -> u32 {
        i
    }

    fn echo_isize(&self, i: isize) -> isize {
        i
    }

    fn echo_usize(&self, i: usize) -> usize {
        i
    }
    
    fn echo_i8(&self, i: i8) -> i8 {
        i
    }

    fn echo_u8(&self, i: u8) -> u8 {
        i
    }

    fn echo_bool(&self, i: bool) -> bool {
        i
    }

    fn echo_array_u8(&self, s: [u8; 5]) -> [u8; 5] {
        s
    }

    fn echo_vec_u8(&self, arg: Vec<u8>) -> (Vec<u8>, i64) {
        let l = arg.len() as i64;
        (arg, l)
    }

    fn echo_multi_i32(&self, _n: usize, #[multi(_n)] m: Vec<i32>) -> Vec<i32> {
        m
    }

    fn echo_multi_vec_u8(&self, _n: usize, #[multi(_n)] m: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        m
    }

    fn echo_multi_h256(&self, _n: usize, #[multi(_n)] m: Vec<H256>) -> Vec<H256> {
        m
    }

    fn echo_ser_example_1(&self, se: SerExample1) -> SerExample1 {
        se
    }

    // STORAGE STORE

    #[storage_set("big_uint")]
    fn store_big_uint(&self, bi: BigUint);

    #[storage_set("big_int")]
    fn store_big_int(&self, bi: BigInt);

    #[storage_set("usize")]
    fn store_usize(&self, i: usize);

    #[storage_set("i64")]
    fn store_i64(&self, i: i64);

    #[storage_set("bool")]
    fn store_bool(&self, i: bool);

    #[storage_set("vec_u8")]
    fn store_vec_u8(&self, arg: Vec<u8>);

    #[storage_set("addr")]
    fn store_addr(&self, arg: Address);

    #[storage_set("opt_addr")]
    fn store_opt_addr(&self, opt_addr: Option<Address>);

    #[storage_set("map1")]
    fn store_map1(&self, addr: Address, bi: BigUint);

    #[storage_set("map2")]
    fn store_map2(&self, addr1: &Address, addr2: &Address, bi: &BigUint);

    #[storage_set("map3")]
    fn store_map3(&self, x: usize, b: bool);

    // STORAGE LOAD

    #[storage_get("big_uint")]
    fn load_big_uint(&self) -> BigUint;

    #[storage_get("big_int")]
    fn load_big_int(&self) -> BigInt;

    #[storage_get("usize")]
    fn load_usize(&self) -> usize;

    #[storage_get("i64")]
    fn load_i64(&self) -> i64;

    #[storage_get("bool")]
    fn load_bool(&self) -> bool;

    #[storage_get("vec_u8")]
    fn load_vec_u8(&self) -> Vec<u8>;

    #[storage_get("addr")]
    fn load_addr(&self) -> Address;

    #[storage_get("opt_addr")]
    fn load_opt_addr(&self) -> Option<Address>;

    #[storage_get("map1")]
    fn load_map1(&self, addr: Address) -> BigUint;

    #[storage_get("map2")]
    fn load_map2(&self, addr1: &Address, addr2: &Address) -> BigUint;

    #[storage_get("map3")]
    fn load_map3(&self, x: usize) -> bool;

    // EVENTS

    fn logEventA(&self, data: &BigUint) {
        self.event_a(data);
    }

    fn logEventB(&self, arg1: &BigUint, arg2: &Address, data: &BigUint) {
        self.event_b(arg1, arg2, data);
    }

    #[event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000a")]
    fn event_a(&self, data: &BigUint);

    #[event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000b")]
    fn event_b(&self, arg1: &BigUint, arg2: &Address, data: &BigUint);

    // BIG INT OPERATIONS

    // arithmetic ooperators: + - * / %
    fn add_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a + b }
    fn add_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a + b }
    fn add_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a + b }
    fn add_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a + b }
    fn sub_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a - b }
    fn sub_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a - b }
    fn sub_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a - b }
    fn sub_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a - b }
    fn mul_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a * b }
    fn mul_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a * b }
    fn mul_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a * b }
    fn mul_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a * b }
    fn div_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a / b }
    fn div_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a / b }
    fn div_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a / b }
    fn div_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a / b }
    fn rem_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a % b }
    fn rem_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a % b }
    fn rem_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a % b }
    fn rem_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a % b }

    // assign version of all operators above
    fn add_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r += b; r }
    fn add_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r += b; r }
    fn add_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r += b; r }
    fn add_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r += b; r }
    fn sub_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r -= b; r }
    fn sub_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r -= b; r }
    fn sub_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r -= b; r }
    fn sub_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r -= b; r }
    fn mul_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r *= b; r }
    fn mul_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r *= b; r }
    fn mul_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r *= b; r }
    fn mul_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r *= b; r }
    fn div_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r /= b; r }
    fn div_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r /= b; r }
    fn div_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r /= b; r }
    fn div_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r /= b; r }
    fn rem_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r %= b; r }
    fn rem_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r %= b; r }
    fn rem_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r %= b; r }
    fn rem_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r %= b; r }

    fn bit_and_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a & b }
    fn bit_and_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a & b }
    fn bit_or_big_uint(&self, a: BigUint, b: BigUint) -> BigUint        { a | b }
    fn bit_or_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint  { a | b }
    fn bit_xor_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a ^ b }
    fn bit_xor_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a ^ b }

    fn bit_and_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r &= b; r }
    fn bit_and_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r &= b; r }
    fn bit_or_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint        { let mut r = a.clone(); r |= b; r }
    fn bit_or_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint  { let mut r = a.clone(); r |= b; r }
    fn bit_xor_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r ^= b; r }
    fn bit_xor_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r ^= b; r }

    fn shr_big_uint(&self, a: BigUint, b: usize) -> BigUint      { a >> b }
    fn shr_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint { a >> b }
    fn shl_big_uint(&self, a: BigUint, b: usize) -> BigUint      { a << b }
    fn shl_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint { a << b }

    fn shr_assign_big_uint(&self, a: BigUint, b: usize) -> BigUint      { let mut r = a.clone(); r >>= b; r }
    fn shr_assign_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint { let mut r = a.clone(); r >>= b; r }
    fn shl_assign_big_uint(&self, a: BigUint, b: usize) -> BigUint      { let mut r = a.clone(); r <<= b; r }
    fn shl_assign_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint { let mut r = a.clone(); r <<= b; r }

    // CRYPTO FUNCTIONS

    fn computeSha256(&self, input: Vec<u8>) -> Vec<u8> {
        self.sha256(&input).as_ref().into()
    }

    fn computeKeccak256(&self, input: Vec<u8>) -> Vec<u8> {
        self.keccak256(&input).as_ref().into()
    }

}
