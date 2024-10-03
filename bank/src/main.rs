#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account)
}

fn update_account(account: &mut Account) {
    account.balance = 10;
}

fn main() {
    // let bank = Bank::new();
    let mut account = Account::new(1, String::from("techarm"));

    // let account_ref = &account;
    let account_up = &mut account;

    // account.balance = 10;
    // print_account(account_ref);

    update_account(account_up);

    // print_account(account_ref);

    // let new_account = account;

    // println!("{:#?}", bank);
    println!("{:#?}", account);
}
