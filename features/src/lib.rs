
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

    fn store_big_uint(&self, bi: BigUint) {
        self.storage_store_big_uint(&ZERO_KEY.into(), &bi);
    }

    fn load_big_uint(&self) -> BigUint {
        self.storage_load_big_uint(&ZERO_KEY.into())
    }

    fn echo_big_int(&self, bi: BigInt) -> BigInt {
        bi
    }

    fn store_big_int(&self, bi: BigInt) {
        self.storage_store_big_int(&ZERO_KEY.into(), &bi);
    }

    fn load_big_int(&self) -> BigInt {
        self.storage_load_big_int(&ZERO_KEY.into())
    }

    fn echo_i64(&self, i: i64) -> i64 {
        i
    }

    fn store_i64(&self, i: i64) {
        self.storage_store_i64(&ZERO_KEY.into(), i);
    }

    fn load_i64(&self) -> (bool, i64) {
        let o = self.storage_load_i64(&ZERO_KEY.into());
        match o {
            Some(i) => (true, i),
            None => (false, 0)
        }
    }

    fn echo_vec_u8(&self, arg: Vec<u8>) -> (Vec<u8>, i64) {
        self.storage_store(&ZERO_KEY.into(), &arg);
        let l = arg.len() as i64;
        (arg, l)
    }

    fn store_vec_u8(&self, arg: Vec<u8>) {
        self.storage_store(&ZERO_KEY.into(), &arg)
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

    fn computeSha256(&self, input: Vec<u8>) -> Vec<u8> {
        self.sha256(&input).as_ref().into()
    }

    fn computeKeccak256(&self, input: Vec<u8>) -> Vec<u8> {
        self.keccak256(&input).as_ref().into()
    }

}
