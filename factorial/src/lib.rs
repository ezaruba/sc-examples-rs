
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

#[elrond_wasm_derive::contract(FactorialImpl)]
pub trait Factorial {

    fn init(&self) {
    }

    fn factorial(&self, value: BigInt) -> BigInt {
        let mut result = BigInt::from(1);
        let one = BigInt::from(1);
        let mut x = BigInt::from(1);
        while &x <= &value {
            result *= &x;
            x += &one;
        }

        result
    }
}
