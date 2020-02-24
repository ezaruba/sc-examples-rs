
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

static STORAGE_KEY_1: [u8; 32] = [0x11u8; 32];
static STORAGE_KEY_2: [u8; 32] = [0x22u8; 32];
static STORAGE_KEY_3: [u8; 32] = [0x33u8; 32];
static LAST_PAY_KEY: [u8; 32] = [0x44u8; 32];

#[elrond_wasm_derive::contract(BobImpl)]
pub trait Bob {

    fn init(&self) {
    }

    #[payable(payment)]
    fn payMe(&self, payment: BigInt, arg1: i64) {
        self.storage_store_big_int(&LAST_PAY_KEY.into(), &payment);
        self.storage_store_i64(&STORAGE_KEY_1.into(), arg1);
    }

    #[payable(payment)]
    fn payMeWithResult(&self, payment: BigInt, arg1: i64) -> i64 {
        self.payMe(payment, arg1);
        0x7777
    }

    fn messageMe(&self, arg1: i64, arg2: i64) {
        self.storage_store_i64(&STORAGE_KEY_2.into(), arg1);
        self.storage_store_i64(&STORAGE_KEY_3.into(), arg2);
    }
}
