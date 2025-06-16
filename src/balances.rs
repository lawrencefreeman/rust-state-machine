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
    //setter function fpor the balance by who defaulting to 0 is not found
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        *self.balances.get(who).unwrap_or(&0);
    }
    //getter for the balance by who
    pub fn balance(&self, who: &String) ->  u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

}