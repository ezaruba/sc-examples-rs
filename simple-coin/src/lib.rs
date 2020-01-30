
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

static TOTAL_SUPPLY_KEY: [u8; 32] = [0u8; 32];

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

#[elrond_wasm_derive::contract]
pub trait SimpleCoinElrond {
    /// constructor function
    /// is called immediately after the contract is created
    /// will set the fixed global token supply and give all the supply to the creator
    fn init(&self, total_supply: &BigInt) {
        let sender = self.get_caller();

        // save total supply
        self.storage_store_big_int(&TOTAL_SUPPLY_KEY.into(), &total_supply);

        // sender balance <- total supply
        let balance_key = balance_key(&sender);
        self.storage_store_big_int(&balance_key, &total_supply);
    }

    /// getter function: retrieves total token supply
    fn totalSupply(&self) -> BigInt {
        let total_supply = self.storage_load_big_int(&TOTAL_SUPPLY_KEY.into());
        total_supply
    }

    /// getter function: retrieves balance for an account
    fn balanceOf(&self, subject: Address) -> BigInt {
        // load balance
        let balance_key = balance_key(&subject);
        let balance = self.storage_load_big_int(&balance_key);

        // return balance as big int
        balance
    }

    /// getter function: retrieves allowance granted from one account to another
    fn allowance(&self, sender: Address, recipient: Address) -> BigInt {
        // get allowance
        let allowance_key = allowance_key(&sender, &recipient);
        let res = self.storage_load_big_int(&allowance_key);

        // return allowance as big int
        res
    }

    #[private]
    fn _perform_transfer(&self, sender: Address, recipient: Address, amount: BigInt) -> Result<(), &str> {
        // load sender balance
        let sender_balance_key = balance_key(&sender);
        let mut sender_balance = self.storage_load_big_int(&sender_balance_key);
    
        // check if enough funds
        if &amount > &sender_balance {
            return Err("insufficient funds");
        }
    
        // update sender balance
        sender_balance -= &amount;
        self.storage_store_big_int(&sender_balance_key, &sender_balance);
    
        // load & update receiver balance
        let rec_balance_key = balance_key(&recipient);
        let mut rec_balance = self.storage_load_big_int(&rec_balance_key);
        rec_balance += &amount;
        self.storage_store_big_int(&rec_balance_key, &rec_balance);
    
        // log operation
        self.transfer_event(&sender, &recipient, &amount);

        Ok(())
    }

    /// transfers tokens from sender to another account
    fn transferToken(&self, recipient: Address, amount: BigInt) -> Result<(), &str> {
        // sender is the caller
        let sender = self.get_caller();

        if &amount < &BigInt::from(0) {
            return Err("transfer amount cannot be negative");
        }
        
        self._perform_transfer(sender, recipient, amount)
    }

    /// sender allows beneficiary to use given amount of tokens from sender's balance
    /// it will completely overwrite any previously existing allowance from sender to beneficiary
    fn approve(&self, recipient: Address, amount: BigInt) -> Result<(), &str> {
        // sender is the caller
        let sender = self.get_caller();

        if &amount < &BigInt::from(0) {
            return Err("approve amount cannot be negative");
        }
      
        // store allowance
        let allowance_key = allowance_key(&sender, &recipient);
        self.storage_store_big_int(&allowance_key, &amount);
      
        // log operation
        self.approve_event(&sender, &recipient, &amount);
        Ok(())
    }
 
    /// caller uses allowance to transfer funds between 2 other accounts
    fn transferFrom(&self, sender: Address, recipient: Address, amount: BigInt) -> Result<(), &str> {
        // get caller
        let caller = self.get_caller();

        if &amount < &BigInt::from(0) {
            return Err("transfer amount cannot be negative");
        }

        // load allowance
        let allowance_key = allowance_key(&sender, &caller);
        let mut allowance = self.storage_load_big_int(&allowance_key);

        // amount should not exceed allowance
        if &amount > &allowance {
            return Err("allowance exceeded");
        }

        // update allowance
        allowance -= &amount;
        self.storage_store_big_int(&allowance_key, &allowance);

        self._perform_transfer(sender, recipient, amount)
    }

    #[event("0x7134692b230b9e1ffa39098904722134159652b09c5bc41d88d6698779d228ff")]
    fn approve_event(&self, sender: &Address, recipient: &Address, amount: &BigInt);

    #[event("0xf099cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9")]
    fn transfer_event(&self, sender: &Address, recipient: &Address, amount: &BigInt);
}
