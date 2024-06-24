use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::accounts::{Account, AccountResponse, AccountTransferResponse, AccountType};

/// Premium account implementation with balance, overdraft limit, currency, and account number
pub struct PremiumAccount {
    balance: f64,
    overdraft_limit: f64,
    currency: String,
    account_number: String,
    account_type: AccountType,
}

impl PremiumAccount {
    pub fn new(currency: &str, overdraft_limit: f64) -> Self {
        // account number is a combination of account type and 10 random alphanumeric characters
        let account_number = format!(
            "{}-{}",
            AccountType::Premium,
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>()
        );

        PremiumAccount {
            balance: 0.0,
            overdraft_limit,
            currency: currency.to_string(),
            account_number,
            account_type: AccountType::Premium,
        }
    }
}

impl Account for PremiumAccount {
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
        // wothdraw shd allow for overdraft
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
        if self.balance + self.overdraft_limit >= amount {
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
                error_message: Some("Overdraft limit exceeded".to_string()),
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