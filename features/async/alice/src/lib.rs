
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

static CALEE_STORAGE_KEY: [u8; 32] = [0u8; 32];
static CALLBACK_INFO_KEY: [u8; 32] = [0x77u8; 32];
static SOME_ADDRESS: [u8; 32] = [0xfeu8; 32];

#[elrond_wasm_derive::callable(PayMeProxy)]
pub trait PayMe {

    #[payable(_payment)]
    fn payMe(&self, _payment: BigInt, _arg1: i64);

    #[payable(_payment)]
    #[callback(payCallback)]
    fn payMeWithResult(&self, _payment: BigInt, _arg1: i64);
}

#[elrond_wasm_derive::callable(MessageMeProxy)]
pub trait MessageMe {
    fn messageMe(&self, arg1: i64, arg2: &BigInt, arg3: &Vec<u8>, arg4: &Address);
}

#[elrond_wasm_derive::callable(MessageMeProxy)]
pub trait MessageMeWithCallback {
    #[callback(messageCallback)]
    fn messageMe(&self, arg1: i64, arg2: BigInt, arg3: Vec<u8>, arg4: Address);
}

#[elrond_wasm_derive::contract(BobImpl)]
pub trait Alice {

    fn init(&self, calee_address: Address) {
        self.storage_store_bytes32(&CALEE_STORAGE_KEY.into(), &calee_address.into());
    }

    #[payable(payment)]
    fn forwardToOtherContract(&self, payment: BigInt) {
        let calee_address: Address = self.storage_load_bytes32(&CALEE_STORAGE_KEY.into()).into();

        let target_contract = contract_proxy!(self, &calee_address, PayMe);
        target_contract.payMe(payment, 0x56);
    }

    #[payable(payment)]
    fn forwardToOtherContractWithCallback(&self, payment: BigInt) {
        let calee_address: Address = self.storage_load_bytes32(&CALEE_STORAGE_KEY.into()).into();

        let target_contract = contract_proxy!(self, &calee_address, PayMe);
        target_contract.payMeWithResult(payment, 0x56);
    }

    fn messageOtherContract(&self) {
        let calee_address: Address = self.storage_load_bytes32(&CALEE_STORAGE_KEY.into()).into();

        let target_contract = contract_proxy!(self, &calee_address, MessageMe);
        target_contract.messageMe(0x01, &BigInt::from(0x02), &create_a_vec(), &SOME_ADDRESS.into());
    }

    fn messageOtherContractWithCallback(&self) {
        let calee_address: Address = self.storage_load_bytes32(&CALEE_STORAGE_KEY.into()).into();

        let target_contract = contract_proxy!(self, &calee_address, MessageMeWithCallback);
        target_contract.messageMe(0x01, BigInt::from(0x02), create_a_vec(), SOME_ADDRESS.into());
    }

    #[callback]
    fn payCallback(&self, cb_arg: i64) {
        self.storage_store_i64(&CALLBACK_INFO_KEY.into(), cb_arg);
    }

    #[callback]
    fn messageCallback(&self) {
        self.storage_store_i64(&CALLBACK_INFO_KEY.into(), 0x5555);
    }
}

fn create_a_vec() -> Vec<u8> {
    let mut res = Vec::with_capacity(3);
    res.push(3);
    res.push(3);
    res.push(3);
    res
}
