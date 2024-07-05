#[derive(Debug)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    CHF,
    AUD,
    CAD,
    BRL
}

pub struct Account {
    balance: f64,
    currency: Currency,
}

impl Account {
    pub fn new(balance: f64, currency: Currency) -> Account {
        Account{balance, currency}
    }
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount
    }
    pub fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount
        }
        self.balance >= amount
    }
    pub fn check_balance(&self) {
        println!("Saldo da Conta {} {:?}.", self.balance, self.currency)
    }
    pub fn convert_to(&mut self, target_currency: Currency) {
        let exchange_rate = 0.984;
        self.balance *= exchange_rate;
        self.currency = target_currency;
    }
}