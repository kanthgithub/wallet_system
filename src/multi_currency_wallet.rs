use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::accounts::{Account, AccountResponse};
use crate::wallets::{DisplayWallet, TransferResponse, Wallet, WalletType, WithdrawWalletResponse};

/// Multi-currency wallet managing multiple accounts
pub struct MultiCurrencyWallet {
    accounts: Vec<Box<dyn Account>>,
    wallet_id: String,
    wallet_type: WalletType,
}

impl MultiCurrencyWallet {
    pub fn new() -> Self {
        let wallet_id = format!(
            "{:?}-{}",
            WalletType::MultiCurrency,
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>()
        );

        MultiCurrencyWallet {
            accounts: Vec::new(),
            wallet_id,
            wallet_type: WalletType::MultiCurrency,
        }
    }
}

impl DisplayWallet for MultiCurrencyWallet {
    fn display_details(&self) {
        println!("Wallet ID: {}", self.wallet_id);
        println!("Wallet Type: {:?}", self.wallet_type);
        println!("-----------------------------");
        for (i, account) in self.accounts.iter().enumerate() {
            println!("Account {} Details: ", i+1);
            println!("-----------------------------");
            account.display_details();
            println!("-----------------------------");
        }
    }
}

impl Wallet for MultiCurrencyWallet {

    fn get_account_by_currency(&self, currency: &str) -> Option<&dyn Account> {
       // self.accounts.iter().position(|acc| acc.get_currency() == currency)

        // iterate the accounts in self and return the matching account for the currency
        // if no account is found, return None
        self.accounts.iter()
            .find(|acc| acc.get_currency() == currency)
            .map(|acc| &**acc)
    }

    fn add_account(&mut self, account: Box<dyn Account>) -> Result<&dyn Account, String> {
        if self.get_account_by_currency(account.get_currency()).is_some() {
            return Err(String::from("Account with this currency already exists"));
        }

        self.accounts.push(account);
        Ok(self.accounts.last().unwrap().as_ref())
    }

    fn balance(&self, currency: &str) -> Result<f64, String> {
        // iterate the accounts in self and sum the balances of the accounts with the matching currency
        match self.accounts.iter().find(|acc| acc.get_currency() == currency) {
            Some(account) => Ok(account.get_balance()),
            None => Err(format!("No account found with currency: {}", currency)),
        }
    }

    fn get_wallet_id(&self) -> &str {
        &self.wallet_id
    }

    fn get_wallet_type(&self) -> WalletType {
        self.wallet_type.clone()
    }

    fn find_account_index_by_currency(&self, currency: &str) -> Option<usize> {
        // iterate the accounts in self and return the index of the account with the matching currency
        // if no account is found, return None
        match self.accounts.iter().position(|acc| acc.get_currency() == currency) {
            Some(index) => Some(index),
            None => None,
        }
    }

    fn get_account_number_by_index(&self, index: usize) -> Option<&str> {
        // return the account number of the account at the given index
        // if the index is out of bounds, return None
        match self.accounts.get(index) {
            Some(account) => Some(account.get_account_number()),
            None => None,
        }
    }


    fn transfer(&mut self, to_wallet: &mut dyn Wallet, currency: &str, amount: f64) -> TransferResponse {
    match self.accounts.iter_mut().find(|account| account.get_currency() == currency) {
        Some(account) => {
            let withdraw_response = account.withdraw(amount);
            if withdraw_response.is_successful {
                let deposit_response = to_wallet.deposit(currency, amount);
                if deposit_response.is_successful {
                    TransferResponse {
                        currency: currency.to_string(),
                        amount,
                        sender_account_number: withdraw_response.account_number,
                        sender_wallet_id: self.wallet_id.clone(),
                        sender_wallet_type: self.wallet_type.clone(),
                        recipient_account_number: deposit_response.account_number,
                        recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                        recipient_wallet_type: to_wallet.get_wallet_type(),
                        is_successful: true,
                        error_message: None,
                    }
                } else {
                    // Deposit failed, so revert the withdrawal
                    account.deposit(amount);
                    TransferResponse {
                        currency: currency.to_string(),
                        amount,
                        sender_account_number: withdraw_response.account_number,
                        sender_wallet_id: self.wallet_id.clone(),
                        sender_wallet_type: self.wallet_type.clone(),
                        recipient_account_number: "".to_string(),
                        recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                        recipient_wallet_type: to_wallet.get_wallet_type(),
                        is_successful: false,
                        error_message: deposit_response.error_message,
                    }
                }
            } else {
                TransferResponse {
                    currency: currency.to_string(),
                    amount,
                    sender_account_number: withdraw_response.account_number,
                    sender_wallet_id: self.wallet_id.clone(),
                    sender_wallet_type: self.wallet_type.clone(),
                    recipient_account_number: "".to_string(),
                    recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                    recipient_wallet_type: to_wallet.get_wallet_type(),
                    is_successful: false,
                    error_message: withdraw_response.error_message,
                }
            }
        }

        _ => {
            TransferResponse {
                currency: currency.to_string(),
                amount,
                sender_account_number: "".to_string(),
                sender_wallet_id: self.wallet_id.clone(),
                sender_wallet_type: self.wallet_type.clone(),
                recipient_account_number: "".to_string(),
                recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                recipient_wallet_type: to_wallet.get_wallet_type(),
                is_successful: false,
                error_message: Some(format!("No account found with currency: {}", currency)),
            }
        }
    }
}


    fn deposit(&mut self, currency: &str, amount: f64) -> AccountResponse {
        // find the account with the matching currency
        // if the account is found, deposit the amount and return the response
        // if the account is not found, return an error response
        match self.accounts.iter_mut().find (| account|  account.get_currency() == currency) {
            Some(account) => {
                let account_response = account.deposit(amount);
                account_response
            }

            _ => {
                AccountResponse {
                    account_number: "".to_string(),
                    account_type: self.accounts[0].get_account_type().clone(),
                    currency: currency.to_string(),
                    balance: 0.0,
                    is_successful: false,
                    error_message: Some(format!("No account found with currency: {}", currency)),
                }
            }
        }
    }

    fn withdraw(&mut self, currency: &str, amount: f64) -> WithdrawWalletResponse {
        // find the account with the matching currency
        // if the account is found, withdraw the amount and return the response
        // if the account is not found, return an error response
        match self.accounts.iter_mut().find (| account|  account.get_currency() == currency) {
            Some(account) => {
                let account_response = account.withdraw(amount);
                WithdrawWalletResponse {
                    wallet_id: self.wallet_id.clone(),
                    wallet_type: self.wallet_type.clone(),
                    currency: currency.to_string(),
                    amount,
                    account_number: account_response.account_number,
                    account_type: account_response.account_type,
                    balance: account_response.balance,
                    is_successful: account_response.is_successful,
                    error_message: account_response.error_message,
                }
            }

            _ => {
                WithdrawWalletResponse {
                    wallet_id: self.wallet_id.clone(),
                    wallet_type: self.wallet_type.clone(),
                    currency: currency.to_string(),
                    amount,
                    account_number: "".to_string(),
                    account_type: self.accounts[0].get_account_type().clone(),
                    balance: 0.0,
                    is_successful: false,
                    error_message: Some(format!("No account found with currency: {}", currency)),
                }
            }
        }
    }
}
