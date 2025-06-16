use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}
impl Pallet {
    pub fn new() -> Pallet {
        Pallet {
            balances: BTreeMap::new()
        }
    }
    //setter function for the balance by who defaulting to 0 is not found
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }
    //getter for the balance by who
    pub fn balance(&self, who: &String) ->  u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn init_balances() {
		/* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
        let mut balances = Pallet::new();
		/* TODO: Assert that the balance of `alice` starts at zero. */
        //these init of Alice and Bob or whoever work becuase of the defaul &0 return from unwrap_or
        assert_eq!(balances.balance(&String::from("Alice")), 0);
		/* TODO: Set the balance of `alice` to 100. */
        balances.set_balance(&String::from("Alice"), 100);
		/* TODO: Assert the balance of `alice` is now 100. */
        assert_eq!(balances.balance(&String::from("Alice")), 100);
		/* TODO: Assert the balance of `bob` has not changed and is 0. */
        assert_eq!(balances.balance(&String::from("Bob")), 0);
	}
}