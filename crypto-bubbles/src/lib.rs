
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

static OWNER_KEY: [u8; 32] = [0u8; 32];

#[elrond_wasm_derive::contract(CryptoBubblesImpl)]
pub trait CryptoBubbles
{
    /// constructor function
    /// is called immediately after the contract is created
    /// will set the fixed global token supply and give all the supply to the creator
    fn init(&self) {
        let sender = self.get_caller();
        self.storage_store_bytes32(&OWNER_KEY.into(), &sender.into());
    }

    /// generates the balance key that maps balances with their owners
    #[private]
    fn _player_balance_key(&self, address: &Address) -> StorageKey {
        let mut raw_key: Vec<u8> = Vec::with_capacity(33);
        raw_key.push(1u8); // "1" is for balance keys
        raw_key.extend_from_slice(address.as_fixed_bytes()); // append the entire address
        let key = self.keccak256(&raw_key); // this compresses the key down to 32 bytes
        key.into()
    }

    /// getter function: retrieves balance for an account
    fn balanceOf(&self, subject: Address) -> BigUint {
        let balance_key = self._player_balance_key(&subject);
        let balance = self.storage_load_big_uint(&balance_key);
        balance
    }

    /// player adds funds
    #[payable(payment)]
    fn topUp(&self, payment: BigUint) {
        let caller = self.get_caller();
        
        let balance_key = self._player_balance_key(&caller);
        let mut balance = self.storage_load_big_uint(&balance_key);
        balance += &payment;
        self.storage_store_big_uint(&balance_key, &balance);

        self.top_up_event(&caller, &payment);
    }

    /// player withdraws funds
    fn withdraw(&self, amount: &BigUint) -> Result<(), &str> {
        self._transferBackToPlayerWallet(&self.get_caller(), amount)
    }

    /// server calls withdraw on behalf of the player
    #[private]
    fn _transferBackToPlayerWallet(&self, player: &Address, amount: &BigUint) -> Result<(), &str> {
        let balance_key = self._player_balance_key(&player);
        let mut balance = self.storage_load_big_uint(&balance_key);
        if amount > &balance {
            return Err("amount to withdraw must be less or equal to balance");
        }
        balance -= amount;
        self.storage_store_big_uint(&balance_key, &balance);

        self.send_tx(player, &amount, "crypto bubbles");

        self.withdraw_event(player, amount);

        Ok(())
    }

    /// player joins game
    #[private]
    fn _addPlayerToGameStateChange(&self, game_index: &BigUint, player: &Address, bet: &BigUint) -> Result<(), &str> {
        let balance_key = self._player_balance_key(&player);
        let mut balance = self.storage_load_big_uint(&balance_key);
        if bet > &balance {
            return Err("insufficient funds to join game");
        }
        balance -= bet;
        self.storage_store_big_uint(&balance_key, &balance);

        self.player_joins_game_event(game_index, player, bet);

        Ok(())
    }

    // player tops up + joins a game
    #[payable]
    fn joinGame(&self, game_index: BigUint) -> Result<(), &str> {
        let player = self.get_caller();
        let bet = self.get_call_value_big_uint();

        self.topUp(self.get_call_value_big_uint());
        self._addPlayerToGameStateChange(&game_index, &player, &bet)
    }

    // owner transfers prize into winner SC account
    fn rewardWinner(&self, game_index: &BigUint, winner: &Address, prize: &BigUint) -> Result<(), &str> {
        let caller = self.get_caller();
        let owner: Address = self.storage_load_bytes32(&OWNER_KEY.into()).into();
        if caller != owner {
            return Err("invalid sender: only contract owner can reward winner");
        }

        let balance_key = self._player_balance_key(&winner);
        let mut balance = self.storage_load_big_uint(&balance_key);
        balance += prize;
        self.storage_store_big_uint(&balance_key, &balance);

        self.reward_winner_event(game_index, &winner, &prize);

        Ok(())
    }

    // owner transfers prize into winner SC account, then transfers funds to player wallet
    fn rewardAndSendToWallet(&self, game_index: &BigUint, winner: &Address, prize: &BigUint) -> Result<(), &str> {
        self.rewardWinner(game_index, winner, prize)?;
        self._transferBackToPlayerWallet(winner, prize)
    }

    #[event("0x1000000000000000000000000000000000000000000000000000000000000001")]
    fn top_up_event(&self, player: &Address, amount: &BigUint);

    #[event("0x1000000000000000000000000000000000000000000000000000000000000002")]
    fn withdraw_event(&self, player: &Address, amount: &BigUint);
    
    #[event("0x1000000000000000000000000000000000000000000000000000000000000003")]
    fn player_joins_game_event(&self, game_index: &BigUint, player: &Address, bet: &BigUint);
    
    #[event("0x1000000000000000000000000000000000000000000000000000000000000004")]
    fn reward_winner_event(&self, game_index: &BigUint, winner: &Address, prize: &BigUint);
}
