mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.

mod types {
    pub type AccountID = String;
    pub type Balance = u128;
    pub type Nonce = u32;
    pub type BlockNumber = u32;
}


#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    
}
impl system::Config for Runtime {
	type AccountID = types::AccountID;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type AccountID = types::AccountID;
    type Balance = types::Balance;
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		/* TODO: Create a new `Runtime` by creating new instances of `system` and `balances`. */
		Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
	}
}


fn main() {
    let mut runtime = Runtime::new();
    println!("The before state system pallet contents is {:#?}", runtime.system);
    println!("The before state balances pallet contents is {:#?}", runtime.balances);
    
    let alice = &String::from("Alice");
    let bob = &String::from("Bob");
    let charlie = &String::from("Charlie");

    runtime.balances.set_balance(alice, 100);
    runtime.system.inc_block_number();
    runtime.system.inc_nonce(alice);
    
    let _res = runtime.balances.transfer(alice, bob, 30).map_err(|e| eprintln!("{}", e));
    runtime.system.inc_nonce(alice);
    
    let _ = runtime.balances.transfer(alice, charlie, 20);
    println!("The after state system pallet contents is {:#?}", runtime.system);
    println!("The after state balances pallet contents is {:#?}", runtime.balances);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime() {
        let mut runtime = Runtime::new();
        let alice = &String::from("Alice");
        runtime.balances.set_balance(alice, 100);
        runtime.system.inc_block_number();
        // TODO: Assert the block number is what we expect
        assert_eq!(runtime.system.block_number(), 1);
        // TODO: Increment the nonce of `alice`
        runtime.system.inc_nonce(alice);
        //assert the nonce of alice is 1.
        assert_eq!(runtime.system.get_nonce(alice), 1); //test passes
        //execute a transfer from alice to bob for 30 tokens.
        let bob = &String::from("Bob");
        //using map_err to handle err with anomymous function || https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err
        let _res = runtime.balances.transfer(alice, bob, 30).map_err(|e| eprintln!("{}", e));
        assert_eq!(runtime.balances.balance(alice), 70); //test passes
        //increment the nonce of alice again (to 2)
        runtime.system.inc_nonce(alice);
        assert_eq!(runtime.system.get_nonce(alice), 2);
        //do another bal xfer from alice to charlie
        let charlie = &String::from("Charlie");
        runtime.balances.transfer(alice, charlie, 20);
        assert_eq!(runtime.balances.balance(alice), 50);
    }
}