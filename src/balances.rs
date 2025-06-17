use std::collections::BTreeMap;

type AccountID = String;
type Balance = u128;

#[derive(Debug)]pub struct Pallet {
    balances: BTreeMap<AccountID, Balance>,
}
impl Pallet {
    pub fn new() -> Pallet {
        Pallet {
            balances: BTreeMap::new()
        }
    }
    //setter function for the balance by who defaulting to 0 is not found
    pub fn set_balance(&mut self, who: &AccountID, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }
    //getter for the balance by who
    pub fn balance(&self, who: &AccountID) ->  Balance {
        *self.balances.get(who).unwrap_or(&0)
    }

    //build the transfer method
    pub fn transfer(&mut self, from: &AccountID, to: &AccountID, amount: Balance) -> Result<(), &'static str> {
            let from_bal = self.balance(&from);
            let to_bal = self.balance(&to);
            //safe math for new "from" bal after amt transf out
            let new_from_bal = match from_bal.checked_sub(amount) {
                Some(nb) => nb,
                None => {
                        println!("Not enough funds.");
                        return Err("Not enough funds");
                }
            };
            //no need to handle overflow on add as no amount of DOT would overflow u128
            let new_to_bal = to_bal.checked_add(amount).unwrap();
            // Apply the balance updates
            self.set_balance(from, new_from_bal);

            self.set_balance(to, new_to_bal);
            
            Ok(()) //returning the Unit block if all okay.
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
        //NOTE I've decided NOT to use AccountID::from("name") becuase I dont think it improves readability.
        assert_eq!(balances.balance(&String::from("Alice")), 0);
		/* TODO: Set the balance of `alice` to 100. */
        balances.set_balance(&String::from("Alice"), 100);
		/* TODO: Assert the balance of `alice` is now 100. */
        assert_eq!(balances.balance(&String::from("Alice")), 100);
		/* TODO: Assert the balance of `bob` has not changed and is 0. */
        assert_eq!(balances.balance(&String::from("Bob")), 0);
	}
    #[test]
    fn transfer_amt() {
        let mut balances = Pallet::new();
        balances.set_balance(&String::from("Alice"), 100);
        //check that alice cannot xfer funds she doesnt have
        assert_eq!(balances.transfer(&String::from("Alice"), &String::from("Bob"), 101), Err("Not enough funds"));

        //check that within balance xfers will work - transfering 99 of 100
        assert_eq!(balances.transfer(&String::from("Alice"), &String::from("Bob"), 99), Ok(()));
        assert_eq!(balances.balance(&String::from("Alice")), 1); //100-99=1
        assert_eq!(balances.balance(&String::from("Bob")), 99); //0+99=1
    }
}