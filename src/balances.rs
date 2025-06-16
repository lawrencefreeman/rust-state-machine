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

}