use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::accounts::{Account, AccountResponse, AccountTransferResponse, AccountType};

/// Basic account implementation with balance, currency, and account number
pub struct BasicAccount {
    balance: f64,
    currency: String,
    account_number: String,
    account_type: AccountType,
}

impl BasicAccount {
    pub fn new(currency: &str) -> Self {
        let account_number = format!(
            "{}-{}",
            AccountType::Basic,
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>()
        );

        BasicAccount {
            balance: 0.0,
            currency: currency.to_string(),
            account_number,
            account_type: AccountType::Basic,
        }
    }
}


impl Account for BasicAccount {
    fn get_balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) -> AccountResponse {
        if amount < 0.0 {
            return AccountResponse {
                account_number: self.account_number.clone(),
                account_type: self.account_type.clone(),
                currency: self.currency.clone(),
                balance: self.balance,
                is_successful: false,
                error_message: Some("Cannot deposit a negative amount".to_string()),
            };
        }
        self.balance += amount;
        AccountResponse {
            account_number: self.account_number.clone(),
            account_type: self.account_type.clone(),
            currency: self.currency.clone(),
            balance: self.balance,
            is_successful: true,
            error_message: None,
        }
    }

    fn withdraw(&mut self, amount: f64) -> AccountResponse {
        if amount < 0.0 {
            return AccountResponse {
                account_number: self.account_number.clone(),
                account_type: self.account_type.clone(),
                currency: self.currency.clone(),
                balance: self.balance,
                is_successful: false,
                error_message: Some("Cannot withdraw a negative amount".to_string()),
            };
        }
        if self.balance >= amount {
            self.balance -= amount;
            AccountResponse {
                account_number: self.account_number.clone(),
                account_type: self.account_type.clone(),
                currency: self.currency.clone(),
                balance: self.balance,
                is_successful: true,
                error_message: None,
            }
        } else {
            AccountResponse {
                account_number: self.account_number.clone(),
                account_type: self.account_type.clone(),
                currency: self.currency.clone(),
                balance: self.balance,
                is_successful: false,
                error_message: Some("Insufficient funds".to_string()),
            }
        }
    }

    fn get_currency(&self) -> &str {
        &self.currency
    }

    fn get_account_number(&self) -> &str {
        &self.account_number
    }

    fn get_account_type(&self) -> &AccountType {
        &self.account_type
    }

    fn transfer(&self, to_account: &mut dyn Account, amount: f64) -> AccountTransferResponse {
        todo!()
    }
}
