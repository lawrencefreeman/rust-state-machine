use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
	/// The current block number.
	/* TODO: Create a field `block_number` that stores a `u32`. */
    block_number: u32,
	/// A map from an account to their nonce.
	/* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
	}

    //return the currently stored block number. a getter, so okay to have the same name as field
    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    //increment the block number by 1
    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    //increment the number used once (nonce) for the specified user:
    pub fn inc_nonce(&mut self, who: &String) {
        //love this sweet baby (entry) - a mutable retrieval of BTreeMap value by key :) , dont need who()
        let counter = self.nonce.entry(who.clone()).or_insert(0);
        //increment the value of the dereferenced counter var (val)
        *counter += 1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn init_system() {
        let mut sys_pal = Pallet::new();

        //Increment the current block number.
        sys_pal.inc_block_number(); //should now be 1
        //Check the block number is what we expect.
        assert_eq!(sys_pal.block_number(), 1);
    	//Increment the nonce of `alice`.
        sys_pal.nonce.insert(String::from("alice"), 0);
        sys_pal.inc_nonce(&String::from("alice"));
        //Check the nonce of `alice` is what we expect.
        assert_eq!(sys_pal.nonce.get(&String::from("alice")), Some(&1u32));
        //Check the nonce of `bob` is what we expect (0 via the defaulting of unwrap_or())
        assert_eq!(*sys_pal.nonce.get(&String::from("bob")).unwrap_or(&0), 0);
        
    }
}
