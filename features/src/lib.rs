
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

#[elrond_wasm_derive::contract(ApiFeatureExamplesImpl)]
pub trait ApiFeatureExamples {

    fn init(&self) {
    }

    fn panicWithMessage(&self) {
        panic!("example panic message");
    }

    fn variableLengthArg(&self, arg: Vec<u8>) -> (Vec<u8>, i64) {
        let l = arg.len() as i64;
        (arg, l)
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
