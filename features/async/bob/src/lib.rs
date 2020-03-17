
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

static STORAGE_KEY_1: [u8; 32] = [0x11u8; 32];
static STORAGE_KEY_2: [u8; 32] = [0x22u8; 32];
static STORAGE_KEY_3: [u8; 32] = [0x33u8; 32];
static STORAGE_KEY_4: [u8; 32] = [0x44u8; 32];
static STORAGE_KEY_5: [u8; 32] = [0x55u8; 32];
static LAST_PAY_KEY: [u8; 32] = [0xffu8; 32];

#[elrond_wasm_derive::contract(BobImpl)]
pub trait Bob {

    fn init(&self) {
    }

    #[payable(payment)]
    fn payMe(&self, payment: BigUint, arg1: i64) {
        self.storage_store_big_uint(&LAST_PAY_KEY.into(), &payment);
        self.storage_store_i64(&STORAGE_KEY_1.into(), arg1);
    }

    #[payable(payment)]
    fn payMeWithResult(&self, payment: BigUint, arg1: i64) -> i64 {
        self.payMe(payment, arg1);
        0x7777
    }

    fn messageMe(&self, arg1: i64, arg2: &BigUint, arg3: &Vec<u8>, arg4: Address) {
        self.storage_store_i64(&STORAGE_KEY_2.into(), arg1);
        self.storage_store_big_uint(&STORAGE_KEY_3.into(), arg2);
        self.storage_store(&STORAGE_KEY_4.into(), arg3);
        self.storage_store_bytes32(&STORAGE_KEY_5.into(), &arg4.into());
    }
}
