use std::collections::BTreeMap;
use std::ops::AddAssign;
use num::traits::{CheckedAdd, CheckedSub, Zero, One};


//move the types definition out to main for instantiation in runtime

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountID, Nonce, BlockNumber> {
	/// The current block number.
	/* TODO: Create a field `block_number` that stores a `u32`. */
    block_number: BlockNumber,
	/// A map from an account to their nonce.
	/* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
    nonce: BTreeMap<AccountID, Nonce>,
}

impl <AccountID, Nonce, BlockNumber>Pallet<AccountID, Nonce, BlockNumber>
where
    AccountID: Ord + Clone,
    Nonce: Zero + One + AddAssign + Copy,
    BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy + AddAssign,
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
	}

    //return the currently stored block number. a getter, so okay to have the same name as field
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    //increment the block number by 1
    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one(); //nice :) used One trait. Not sure if i needed AddAssign, need to check.
    }

    //increment the number used once (nonce) for the specified user:
    pub fn inc_nonce(&mut self, who: &AccountID) {
        //love this sweet baby (entry) - a mutable retrieval of BTreeMap value by key :) , dont need who()
        let counter = self.nonce.entry(who.clone()).or_insert(Nonce::zero());
        //increment the value of the dereferenced counter var (val)
        *counter += Nonce::one(); //yep - this utils the AddAssign trait
    }

    //adding a public nonce_getter so that I can test assertions in the runtime...
    //must be reference to self so not moving the system pallet
    pub fn get_nonce(&self, who: &AccountID) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::one()) //added the Copy trait to satisfy
    }

}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn init_system() {
        let mut sys_pal = Pallet::<String, u32, u32>::new();

        //Increment the current block number.
        sys_pal.inc_block_number(); //should now be 1
        //Check the block number is what we expect.
        assert_eq!(sys_pal.block_number(), 1);
    	//Increment the nonce of `Alice`.
        sys_pal.nonce.insert(String::from("Alice"), 0);
        sys_pal.inc_nonce(&String::from("Alice"));
        //Check the nonce of `Alice` is what we expect.
        assert_eq!(sys_pal.nonce.get(&String::from("Alice")), Some(&1));
        //Check the nonce of `Bob` is what we expect (0 via the defaulting of unwrap_or())
        assert_eq!(*sys_pal.nonce.get(&String::from("Bob")).unwrap_or(&0), 0);
        
    }
}
