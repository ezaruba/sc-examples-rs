
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

#[elrond_wasm_derive::contract]
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

    fn badVec(&self, _bad: Vec<u8>) -> Result<Vec<u8>, &str> {
        Ok(Vec::with_capacity(0))
    }

}
