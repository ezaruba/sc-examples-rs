
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

static ZERO_KEY: [u8; 32] = [0u8; 32];

#[elrond_wasm_derive::contract(ApiFeatureExamplesImpl)]
pub trait ApiFeatureExamples {

    fn init(&self) {
    }

    fn panicWithMessage(&self) {
        panic!("example panic message");
    }

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

    fn store_big_uint(&self, bi: BigUint) {
        self.storage_store_big_uint(&ZERO_KEY.into(), &bi);
    }

    fn store_big_int(&self, bi: BigInt) {
        self.storage_store_big_int(&ZERO_KEY.into(), &bi);
    }

    fn store_i64(&self, i: i64) {
        self.storage_store_i64(&ZERO_KEY.into(), i);
    }

    fn store_vec_u8(&self, arg: Vec<u8>) {
        self.storage_store(&ZERO_KEY.into(), &arg)
    }

    

    fn load_big_uint(&self) -> BigUint {
        self.storage_load_big_uint(&ZERO_KEY.into())
    }

    fn load_big_int(&self) -> BigInt {
        self.storage_load_big_int(&ZERO_KEY.into())
    }

    fn load_i64(&self) -> Option<i64> {
        self.storage_load_i64(&ZERO_KEY.into())
    }

    fn load_vec_u8(&self) -> (Vec<u8>, i64) {
        let v = self.storage_load(&ZERO_KEY.into());
        let l = v.len() as i64;
        (v, l)
    }

    fn storageLoadVec(&self) -> (Vec<u8>, i64) {
        let key: StorageKey = [0x01u8; 32].into();
        let value = self.storage_load(&key);
        let l = value.len() as i64;
        (value, l)
    }

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
    

    fn computeSha256(&self, input: Vec<u8>) -> Vec<u8> {
        self.sha256(&input).as_ref().into()
    }

    fn computeKeccak256(&self, input: Vec<u8>) -> Vec<u8> {
        self.keccak256(&input).as_ref().into()
    }

}
