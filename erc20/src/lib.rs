
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

static TOTAL_SUPPLY_KEY: [u8; 32] = [0u8; 32];

static APPROVE_EVENT:  [u8; 32] = [0x71,0x34,0x69,0x2B,0x23,0x0B,0x9E,0x1F,0xFA,0x39,0x09,0x89,0x04,0x72,0x21,0x34,0x15,0x96,0x52,0xB0,0x9C,0x5B,0xC4,0x1D,0x88,0xD6,0x69,0x87,0x79,0xD2,0x28,0xFF];
static TRANSFER_EVENT: [u8; 32] = [0xF0,0x99,0xCD,0x8B,0xDE,0x55,0x78,0x14,0x84,0x2A,0x31,0x21,0xE8,0xDD,0xFD,0x43,0x3A,0x53,0x9B,0x8C,0x9F,0x14,0xBF,0x31,0xEB,0xF1,0x08,0xD1,0x2E,0x61,0x96,0xE9];



/// Generates a balance key for some address.
/// Used to map balances with their owners.
fn balance_key(address: &Address) -> StorageKey {
  let mut key = [0u8; 32];
  // reserve one byte of the key to indicate key type
  // "1" is for balance keys
  key[0] = 1;

  // the last 2 bytes of the address are only used to identify the shard, 
  // so they are disposable when constructing the key
  let addr_bytes: [u8; 32] = *address.as_fixed_bytes();
  for i in 0..30 {
    key[2+i] = addr_bytes[i];
  }

  key.into()
}

fn allowance_key(from: &Address, to: &Address) -> StorageKey {
    let mut key = [0u8; 32];
    // reserve one byte of the key to indicate key type
    // "2" is for allowance keys
    key[0] = 2;
  
    // Note: in smart contract addresses, the first 10 bytes are all 0
    // therefore we read from byte 10 onwards to provide more significant bytes
    // and to minimize the chance for collisions
    // TODO: switching to a hash instead of a concatenation of addresses might make it safer
    let from_bytes: [u8; 32] = *from.as_fixed_bytes();
    for i in 0..15 {
      key[1+i] = from_bytes[10+i];
    }
    let to_bytes: [u8; 32] = *to.as_fixed_bytes();
    for i in 0..16 {
      key[16+i] = to_bytes[10+i];
    }
  
    key.into()
  }

fn save_log_with_3_topics<T, BI>(contract: &T, topic1: &[u8; 32], topic2: &[u8; 32], topic3: &[u8; 32], value: &BI)
where 
    BI: BigIntApi,
    T: ContractHookApi<BI>
{
    let topics = [*topic1, *topic2, *topic3];

    let data_vec = value.get_bytes_big_endian_pad_right(32);
  
    // call api
    contract.write_log(&topics[..], data_vec.as_slice());
}

fn perform_transfer<T, BI>(contract: &T, sender: Address, recipient: Address, amount: BI)
where 
    BI: BigIntApi,
    for<'b> BI: AddAssign<&'b BI>,
    for<'b> BI: SubAssign<&'b BI>,
    T: ContractHookApi<BI>
{
    // load sender balance
    let sender_balance_key = balance_key(&sender);
    let mut sender_balance = contract.storage_load_big_int(&sender_balance_key);
  
    // check if enough funds
    if &amount > &sender_balance {
        contract.signal_error();
      return;
    }
  
    // update sender balance
    sender_balance -= &amount;
    contract.storage_store_big_int(&sender_balance_key, &sender_balance);
  
    // load & update receiver balance
    let rec_balance_key = balance_key(&recipient);
    let mut rec_balance = contract.storage_load_big_int(&rec_balance_key);
    rec_balance += &amount;
    contract.storage_store_big_int(&rec_balance_key, &rec_balance);
  
    // log operation
    save_log_with_3_topics(contract, &TRANSFER_EVENT, &sender.into(), &recipient.into(), &amount);
}

#[elrond_wasm_derive::contract]
pub trait Erc20Elrond: ContractHookApi<BI, BU> + Sized 
where 
    BI: BigIntApi + 'static,
    BU: BigUintApi<BI> + 'static,
    for<'b> BI: AddAssign<&'b BI>,
    for<'b> BI: SubAssign<&'b BI>,
{
    /// constructor function
    /// is called immediately after the contract is created
    /// will set the fixed global token supply and give all the supply to the creator
    fn init(&self, total_supply: &BI) {
        let sender = self.get_caller();

        // save total supply
        self.storage_store_big_int(&TOTAL_SUPPLY_KEY.into(), &total_supply);

        // sender balance <- total supply
        let balance_key = balance_key(&sender);
        self.storage_store_big_int(&balance_key, &total_supply);
    }

    /// getter function: retrieves total token supply
    fn totalSupply(&self) -> BI {
        let total_supply = self.storage_load_big_int(&TOTAL_SUPPLY_KEY.into());
        total_supply
    }

    /// getter function: retrieves balance for an account
    fn balanceOf(&self, subject: Address) -> BI {
        // load balance
        let balance_key = balance_key(&subject);
        let balance = self.storage_load_big_int(&balance_key);

        // return balance as big int
        balance
    }

    /// getter function: retrieves allowance granted from one account to another
    fn allowance(&self, sender: Address, recipient: Address) -> BI {
        // get allowance
        let allowance_key = allowance_key(&sender, &recipient);
        let res = self.storage_load_big_int(&allowance_key);

        // return allowance as big int
        res
    }

    /// transfers tokens from sender to another account
    fn transferToken(&self, recipient: Address, amount: BI) -> bool {
        // sender is the caller
        let sender = self.get_caller();

        if &amount < &BI::from(0) {
            self.signal_error();
            false;
        }
        
        perform_transfer(self, sender, recipient, amount);
        true
    }

    /// sender allows beneficiary to use given amount of tokens from sender's balance
    /// it will completely overwrite any previously existing allowance from sender to beneficiary
    fn approve(&self, recipient: Address, amount: BI) -> bool {
        // sender is the caller
        let sender = self.get_caller();
      
        // store allowance
        let allowance_key = allowance_key(&sender, &recipient);
        self.storage_store_big_int(&allowance_key, &amount);
      
        // log operation
        save_log_with_3_topics(self, &APPROVE_EVENT, &sender.into(), &recipient.into(), &amount);      
        true
    }
 
    /// caller uses allowance to transfer funds between 2 other accounts
    fn transferFrom(&self, sender: Address, recipient: Address, amount: BI) -> bool {
        // get caller
        let caller = self.get_caller();

        // load allowance
        let allowance_key = allowance_key(&sender, &caller);
        let mut allowance = self.storage_load_big_int(&allowance_key);

        // amount should not exceed allowance
        if &amount > &allowance {
            self.signal_error();
            return false;
        }

        // update allowance
        allowance -= &amount;
        self.storage_store_big_int(&allowance_key, &allowance);

        perform_transfer(self, sender, recipient, amount);
        true
    }
}


