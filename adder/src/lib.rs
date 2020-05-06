
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

#[elrond_wasm_derive::contract(AdderImpl)]
pub trait Adder {

    #[storage_get("sum")]
    fn getSum(&self) -> BigInt;

    #[private]
    #[storage_set("sum")]
    fn _set_sum(&self, sum: &BigInt);

    fn init(&self, initial_value: &BigInt) {
        self._set_sum(&initial_value);
    }

    fn add(&self, value: &BigInt) {
        let mut sum = self.getSum();
        sum += value;
        self._set_sum(&sum);
    }
}
