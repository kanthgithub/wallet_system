use std::borrow::Cow;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::accounts::{Account, AccountResponse, AccountType};
use crate::wallets::{Wallet, WalletType, TransferResponse, WithdrawWalletResponse};

pub struct BasicWallet<T: Account> {
    wallet_id: String,
    wallet_type: WalletType,
    account: T,
}

impl<T: Account> BasicWallet<T> {
    pub fn new(account: T) -> Self {
        let wallet_id = format!(
            "{:?}-{}",
            WalletType::Basic,
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>()
        );

        BasicWallet {
            account,
            wallet_id,
            wallet_type: WalletType::Basic,
        }
    }
}

impl<T: Account> Wallet for BasicWallet<T> {
    fn add_account(&mut self, account: Box<dyn Account>) -> Result<&dyn Account, String> {
        Err(String::from("Cannot add an account to a BasicWallet after it's created"))
    }

    // modify so that total balance is balance of all accounts in the wallet
    fn balance(&self, currency: &str) -> Result<f64, String> {
        if self.account.get_currency() == currency {
            Ok(self.account.get_balance())
        } else {
            Err(format!("No account found with currency: {}", currency))
        }
    }

    fn get_wallet_id(&self) -> &str {
        &self.wallet_id
    }

    fn get_wallet_type(&self) -> WalletType {
        self.wallet_type.clone()
    }

    fn find_account_index_by_currency(&self, currency: &str) -> Option<usize> {
        if self.account.get_currency() == currency {
            Some(0)
        } else {
            None
        }
    }

    fn get_account_number_by_index(&self, index: usize) -> Option<&str> {
        if index == 0 {
            Some(self.account.get_account_number())
        } else {
            None
        }
    }

    fn get_account_by_currency(&self, currency: &str) -> Option<&dyn Account> {
        if self.account.get_currency() == currency {
            Some(&self.account)
        } else {
            None
        }
    }

    fn transfer(
        &mut self,
        to_wallet: &mut dyn Wallet,
        currency: &str,
        amount: f64,
    ) -> TransferResponse {
        let sender_account_number = self.account.get_account_number().to_string();

        let recipient_account_number = to_wallet
            .find_account_index_by_currency(&currency)
            .map_or("".to_string(), |idx| {
                to_wallet.get_account_number_by_index(idx).unwrap().to_string()
            });

        let transfer_result = if self.account.get_currency() == currency {
            let withdraw_result = self.account.withdraw(amount);
            if !withdraw_result.is_successful {
                return TransferResponse {
                    currency: currency.to_string(),
                    amount,
                    sender_account_number: self.account.get_account_number().to_string(),
                    sender_wallet_id: self.wallet_id.clone(),
                    sender_wallet_type: self.wallet_type.clone(),
                    recipient_account_number,
                    recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                    recipient_wallet_type: to_wallet.get_wallet_type(),
                    is_successful: false,
                    error_message: withdraw_result.error_message,
                };
            }

            let deposit_result = to_wallet.deposit(currency, amount);
            if !deposit_result.is_successful {
                self.account.deposit(amount); // Rollback withdrawal
                return TransferResponse {
                    currency: currency.to_string(),
                    amount,
                    sender_account_number: self.account.get_account_number().to_string(),
                    sender_wallet_id: self.wallet_id.clone(),
                    sender_wallet_type: self.wallet_type.clone(),
                    recipient_account_number,
                    recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                    recipient_wallet_type: to_wallet.get_wallet_type(),
                    is_successful: false,
                    error_message: Some(deposit_result.error_message.unwrap()),
                };
            }

            TransferResponse {
                currency: currency.to_string(),
                amount,
                sender_account_number: self.account.get_account_number().to_string(),
                sender_wallet_id: self.wallet_id.clone(),
                sender_wallet_type: self.wallet_type.clone(),
                recipient_account_number,
                recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                recipient_wallet_type: to_wallet.get_wallet_type(),
                is_successful: true,
                error_message: None,
            }
        } else {
            TransferResponse {
                currency: currency.to_string(),
                amount,
                sender_account_number,
                sender_wallet_id: self.wallet_id.clone(),
                sender_wallet_type: self.wallet_type.clone(),
                recipient_account_number,
                recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                recipient_wallet_type: to_wallet.get_wallet_type(),
                is_successful: false,
                error_message: Some(format!("Source account currency: {}, Receiver account currency: {}, mismatch", self.account.get_currency(), currency)),
            }
        };

        transfer_result
    }

    fn deposit(&mut self, currency: &str, amount: f64) -> AccountResponse {
        if self.account.get_currency() == currency {
            let deposit_result = self.account.deposit(amount);
            deposit_result
        } else {
            AccountResponse {
                account_number: "".to_string(),
                account_type: AccountType::Basic, // Default account type
                currency: currency.to_string(),
                balance: 0.0,
                is_successful: false,
                error_message: Some("Currency mismatch".to_string()),
            }
        }
    }

    fn withdraw(&mut self, currency: &str, amount: f64) -> WithdrawWalletResponse {
    if self.account.get_currency() == currency {
        let withdrawal_result = self.account.withdraw(amount);
        WithdrawWalletResponse {
            wallet_id: self.wallet_id.clone(),
            wallet_type: self.wallet_type.clone(),
            currency: currency.to_string(),
            amount,
            account_number: self.account.get_account_number().to_string(),
            account_type: self.account.get_account_type().clone(),
            balance: self.account.get_balance(),
            is_successful: withdrawal_result.is_successful,
            error_message: withdrawal_result.error_message,
        }
    } else {
        WithdrawWalletResponse {
            wallet_id: self.wallet_id.clone(),
            wallet_type: self.wallet_type.clone(),
            currency: currency.to_string(),
            amount,
            account_number: "".to_string(),
            account_type: AccountType::Basic, // Default account type
            balance: 0.0,
            is_successful: false,
            error_message: Some("Currency mismatch".to_string()),
        }
    }
}
}