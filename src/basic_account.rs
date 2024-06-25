use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::accounts::{Account, AccountResponse, AccountTransferResponse, AccountType, DisplayAccount};

/// Basic account implementation with balance, currency, and account number
#[derive(Debug)]
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

impl DisplayAccount for BasicAccount {
    fn display_details(&self) {
        println!("Account Number: {}", self.account_number);
        println!("Account Type: {:?}", self.account_type);
        println!("Currency: {}", self.currency);
        println!("Balance: {}", self.balance);
    }
}

impl Account for BasicAccount {

    fn default_impl(&self) {
        println!("Default implementation for BasicAccount");
    }

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

    fn get_account_type(&self) -> AccountType {
        self.account_type.clone()
    }

    fn transfer(&mut self, to_account: &mut dyn Account, amount: f64) -> AccountTransferResponse {
        let recipient_account_number = to_account.get_account_number().to_string();
        let recipient_account_type = to_account.get_account_type().clone();
        let currency = self.currency.clone();

        if self.balance >= amount {
            // Withdraw from sender's account
            let withdrawal_response = self.withdraw(amount);
            if withdrawal_response.is_successful {
                // Deposit to recipient's account
                let deposit_response = to_account.deposit(amount);
                if deposit_response.is_successful {
                    return AccountTransferResponse {
                        account_number: self.account_number.clone(),
                        account_type: self.account_type.clone(),
                        currency: currency.clone(),
                        recipient_account_number,
                        recipient_account_type,
                        balance: self.balance,
                        is_successful: true,
                        error_message: None,
                    };
                }
            }
        }

        AccountTransferResponse {
            account_number: self.account_number.clone(),
            account_type: self.account_type.clone(),
            currency: currency.clone(),
            recipient_account_number,
            recipient_account_type,
            balance: self.balance,
            is_successful: false,
            error_message: Some("Insufficient funds".to_string()),
        }
    }

    fn get_overdraft_limit(&self) -> f64 {
        0.0
    }
}
