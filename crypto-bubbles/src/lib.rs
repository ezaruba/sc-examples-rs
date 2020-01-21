
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

static OWNER_KEY: [u8; 32] = [0u8; 32];

static ERR_NOT_OWNER: i32 = 101; // a function that should only be called by the owner is called by someone else
static ERR_JOIN_NOT_ENOUGH_FUNDS: i32 = 103; // cannot join game with given bet, because player does not have enough funds in the game 
static ERR_WITHDRAW_TOO_MUCH: i32 = 104; // trying to withdraw more than the funds the player owns in game
//static ERR_REWARD_TOO_MUCH: i32 = 105; // trying to reward more that the bet remaining in game
//static ERR_CLEANING_GAME_WITH_NO_PLAYERS: i32 = 106; // when cleaning up a game, the game has no players

/// Generates a balance key for some address.
/// Used to map balances with their owners.
fn player_balance_key(address: &Address) -> StorageKey {
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

#[elrond_wasm_derive::contract]
pub trait CryptoBubbles
{
    /// constructor function
    /// is called immediately after the contract is created
    /// will set the fixed global token supply and give all the supply to the creator
    fn init(&self) {
        let sender = self.get_caller();
        self.storage_store_bytes32(&OWNER_KEY.into(), &sender.into());
    }

    /// getter function: retrieves balance for an account
    fn balanceOf(&self, subject: Address) -> BI {
        let balance_key = player_balance_key(&subject);
        let balance = self.storage_load_big_int(&balance_key);
        balance
    }

    /// player adds funds
    #[payable]
    fn topUp(&self) {
        let caller = self.get_caller();
        let call_value = self.get_call_value_big_int();
        
        let balance_key = player_balance_key(&caller);
        let mut balance = self.storage_load_big_int(&balance_key);
        balance += &call_value;
        self.storage_store_big_int(&balance_key, &balance);

        self.top_up_event(&caller, &call_value);
    }

    /// player withdraws funds
    fn withdraw(&self, amount: &BI) {
        self._transferBackToPlayerWallet(&self.get_caller(), amount);
    }

    /// server calls withdraw on behalf of the player
    #[private]
    fn _transferBackToPlayerWallet(&self, player: &Address, amount: &BI) {
        let balance_key = player_balance_key(&player);
        let mut balance = self.storage_load_big_int(&balance_key);
        if amount > &balance {
            self.signal_exit(ERR_WITHDRAW_TOO_MUCH);
            return;
        }
        balance -= amount;
        self.storage_store_big_int(&balance_key, &balance);

        self.send_tx(player, &amount, "crypto bubbles");

        self.withdraw_event(player, amount);
    }

    /// player joins game
    #[private]
    fn _addPlayerToGameStateChange(&self, game_index: &BI, player: &Address, bet: &BI) {
        let balance_key = player_balance_key(&player);
        let mut balance = self.storage_load_big_int(&balance_key);
        if bet > &balance {
            self.signal_exit(ERR_JOIN_NOT_ENOUGH_FUNDS);
            return;
        }
        balance -= bet;
        self.storage_store_big_int(&balance_key, &balance);

        self.player_joins_game_event(game_index, player, bet);
    }

    // player tops up + joins a game
    #[payable]
    fn joinGame(&self, game_index: BI) {
        let player = self.get_caller();
        let bet = self.get_call_value_big_int();

        self.topUp();
        self._addPlayerToGameStateChange(&game_index, &player, &bet);
    }

    // owner transfers prize into winner SC account
    fn rewardWinner(&self, game_index: &BI, winner: &Address, prize: &BI) {
        let caller = self.get_caller();
        let owner: Address = self.storage_load_bytes32(&OWNER_KEY.into()).into();
        if caller != owner {
            self.signal_exit(ERR_NOT_OWNER);
            return;
        }

        let balance_key = player_balance_key(&winner);
        let mut balance = self.storage_load_big_int(&balance_key);
        balance += prize;
        self.storage_store_big_int(&balance_key, &balance);

        self.reward_winner_event(game_index, &winner, &prize)
    }

    // owner transfers prize into winner SC account, then transfers funds to player wallet
    fn rewardAndSendToWallet(&self, game_index: &BI, winner: &Address, prize: &BI) {
        self.rewardWinner(game_index, winner, prize);
        self._transferBackToPlayerWallet(winner, prize);
    }

    #[event("0x1000000000000000000000000000000000000000000000000000000000000001")]
    fn top_up_event(&self, player: &Address, amount: &BI);

    #[event("0x1000000000000000000000000000000000000000000000000000000000000002")]
    fn withdraw_event(&self, player: &Address, amount: &BI);
    
    #[event("0x1000000000000000000000000000000000000000000000000000000000000003")]
    fn player_joins_game_event(&self, game_index: &BI, player: &Address, bet: &BI);
    
    #[event("0x1000000000000000000000000000000000000000000000000000000000000004")]
    fn reward_winner_event(&self, game_index: &BI, winner: &Address, prize: &BI);
}
