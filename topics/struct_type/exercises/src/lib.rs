#[derive(Debug)]
pub struct Account {
    pub address: String,
    pub balance: u32,
}

pub fn new(address: String) -> Account {
    let result = Account {
        address,
        balance: 0,
    };
    result
}
