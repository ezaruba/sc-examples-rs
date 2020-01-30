
#![allow(unused_attributes)]

use features::*;
use elrond_wasm_debug::*;
use elrond_wasm_debug::HashMap;

static ADDR1: [u8; 32] = [0x11u8; 32];
static ADDR2: [u8; 32] = [0x22u8; 32];

fn main() {
    let mock_ref = ArwenMockState::new();

    mock_ref.add_account(AccountData{
        address: ADDR1.into(),
        nonce: 0,
        balance: 0.into(),
        storage: HashMap::new(),
        contract: None,
    });

    // tx 1: init
    let tx1 = TxData::new_create(
        Box::new(ApiFeatureExamplesInst::new(mock_ref.clone())), 
        ADDR1.into(), 
        ADDR2.into());
    let result1 = mock_ref.execute_tx(tx1);
    assert_eq!(0, result1.result_status);
    result1.print();


}
