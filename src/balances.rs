use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config:crate::system::Config {//inherited from system crate so to remove ambiguety and dupe of AccountID def
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T:Config> {
    balances: BTreeMap<T::AccountID, T::Balance>,
}
impl<T:Config> Pallet<T>
{
    // changed from explicit Pallet to Self.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }
    //setter function for the balance by who defaulting to 0 is not found
    pub fn set_balance(&mut self, who: &T::AccountID, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }
    //getter for the balance by who
    pub fn balance(&self, who: &T::AccountID) ->  T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero()) //call the Zero traits zero() method to guarantee compat with our generic. (0 will not ocmpile as a literal of i32)
    }

    //build the transfer method
    pub fn transfer(&mut self, from: &T::AccountID, to: &T::AccountID, amount: T::Balance) -> crate::support::DispatchResult {
            let from_bal = self.balance(&from);
            let to_bal = self.balance(&to);
            //safe math for new "from" bal after amt transf out
            let new_from_bal = match from_bal.checked_sub(&amount) {
                Some(nb) => nb,
                None => {
                        println!("Not enough funds.");
                        return Err("Not enough funds");
                }
            };
            //no need to handle overflow on add as no amount of DOT would overflow u128
            //making the amount argument a reference (&) now as we are using generics we dont know it will live on the stack.
            let new_to_bal = to_bal.checked_add(&amount).unwrap();
            // Apply the balance updates
            self.set_balance(from, new_from_bal);

            self.set_balance(to, new_to_bal);
            
            Ok(()) //returning the Unit block if all okay.
        }

}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestConfig;
    impl crate::system::Config for TestConfig {
        type AccountID = String;
        type BlockNumber = u128;
        type Nonce = u128;
    }

    impl super::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		/* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
        let mut balances = Pallet::<TestConfig>::new();
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
        let mut balances = Pallet::<TestConfig>::new();
        balances.set_balance(&String::from("Alice"), 100);
        //check that alice cannot xfer funds she doesnt have
        assert_eq!(balances.transfer(&String::from("Alice"), &String::from("Bob"), 101), Err("Not enough funds"));

        //check that within balance xfers will work - transfering 99 of 100
        assert_eq!(balances.transfer(&String::from("Alice"), &String::from("Bob"), 99), Ok(()));
        assert_eq!(balances.balance(&String::from("Alice")), 1); //100-99=1
        assert_eq!(balances.balance(&String::from("Bob")), 99); //0+99=1
    }
}