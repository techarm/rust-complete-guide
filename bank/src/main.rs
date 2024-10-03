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

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }

    fn summary(&self) -> String {
        format!(
            "{}: {} has a balance {}",
            self.id, self.holder, self.balance
        )
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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();

    let mut account1 = Account::new(1, String::from("me"));
    account1.deposit(200);
    account1.withdraw(100);
    println!("account1 info: {}", account1.summary());

    let mut account2 = Account::new(2, String::from("you"));
    account2.deposit(500);
    println!("account2 info: {}", account2.summary());

    bank.add_account(account1);
    bank.add_account(account2);
    println!("bank total balance: {}", bank.total_balance());
    println!("bank summary: {:#?}", bank.summary());
}
