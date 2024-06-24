use std::borrow::Cow;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::accounts::{Account, AccountResponse, AccountType};
use crate::wallets::{Wallet, WalletType, TransferResponse, WithdrawWalletResponse};

pub struct BasicWallet {
    wallet_id: String,
    wallet_type: WalletType,
    account: Option<Box<dyn Account>>,
}

impl BasicWallet {
    pub fn new() -> Self {
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
            account: None,
            wallet_id,
            wallet_type: WalletType::Basic,
        }
    }
}

impl Wallet for BasicWallet {
    fn add_account(&mut self, account: Box<dyn Account>) -> Result<&dyn Account, String> {
        if self.account.as_ref().is_some() {
            return Err(String::from("BasicWallet can only hold one account"));
        }
        self.account = Some(account);
        Ok(self.account.as_ref().unwrap().as_ref())
    }

    // modify so that total balance is balance of all accounts in the wallet
    fn balance(&self, currency: &str) -> Result<f64, String> {
        match &self.account {
            Some(account) if account.get_currency() == currency => Ok(account.get_balance()),
            _ => Err(format!("No account found with currency: {}", currency)),
        }
    }

    fn get_wallet_id(&self) -> &str {
        &self.wallet_id
    }

    fn get_wallet_type(&self) -> WalletType {
        self.wallet_type.clone()
    }

    fn find_account_index_by_currency(&self, currency: &str) -> Option<usize> {
        match &self.account {
            Some(account) if account.get_currency() == currency => Some(0),
            _ => None,
        }
    }

    fn get_account_number_by_index(&self, index: usize) -> Option<&str> {
        match &self.account {
            Some(account) if index == 0 => Some(account.get_account_number()),
            _ => None,
        }
    }

    fn get_account_by_currency(&self, currency: &str) -> Option<&dyn Account> {
        match &self.account {
            Some(account) if account.get_currency() == currency => Some(&**account),
            _ => None,
        }
    }

    fn transfer(
        &mut self,
        to_wallet: &mut dyn Wallet,
        currency: &str,
        amount: f64,
    ) -> TransferResponse {
        let sender_account_number = self
            .account
            .as_ref()
            .map_or("".to_string(), |acc| acc.get_account_number().to_string());

        let recipient_account_number = to_wallet
            .find_account_index_by_currency(&currency)
            .map_or("".to_string(), |idx| {
                to_wallet.get_account_number_by_index(idx).unwrap().to_string()
            });

        let transfer_result = match &mut self.account {
            Some(account) if account.get_currency() == currency => {
                let withdraw_result = account.withdraw(amount);
                if !withdraw_result.is_successful {
                    return TransferResponse {
                        currency: currency.to_string(),
                        amount,
                        sender_account_number: account.get_account_number().to_string(),
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
                    account.deposit(amount); // Rollback withdrawal
                    return TransferResponse {
                        currency: currency.to_string(),
                        amount,
                        sender_account_number: account.get_account_number().to_string(),
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
                    sender_account_number: account.get_account_number().to_string(),
                    sender_wallet_id: self.wallet_id.clone(),
                    sender_wallet_type: self.wallet_type.clone(),
                    recipient_account_number,
                    recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                    recipient_wallet_type: to_wallet.get_wallet_type(),
                    is_successful: true,
                    error_message: None,
                }
            }
            _ => TransferResponse {
                currency: currency.to_string(),
                amount,
                sender_account_number,
                sender_wallet_id: self.wallet_id.clone(),
                sender_wallet_type: self.wallet_type.clone(),
                recipient_account_number,
                recipient_wallet_id: to_wallet.get_wallet_id().to_string(),
                recipient_wallet_type: to_wallet.get_wallet_type(),
                is_successful: false,
                error_message: Some("Source account not found or currency mismatch".to_string()),
            },
        };

        transfer_result
    }

    fn deposit(&mut self, currency: &str, amount: f64) -> AccountResponse {
        match &mut self.account {
            Some(account) if account.get_currency() == currency => {
                let deposit_result = account.deposit(amount);
                deposit_result
            }
            _ => AccountResponse {
                account_number: "".to_string(),
                account_type: AccountType::Basic, // Default account type
                currency: currency.to_string(),
                balance: 0.0,
                is_successful: false,
                error_message: Some("Currency mismatch".to_string()),
            },
        }
    }

    fn withdraw(&mut self, currency: &str, amount: f64) -> WithdrawWalletResponse {
        match &mut self.account {
            Some(account) if account.get_currency() == currency => {
                let withdrawal_result = account.withdraw(amount);
                WithdrawWalletResponse {
                    wallet_id: self.wallet_id.clone(),
                    wallet_type: self.wallet_type.clone(),
                    currency: currency.to_string(),
                    amount,
                    account_number: account.get_account_number().to_string(),
                    account_type: account.get_account_type().clone(),
                    balance: account.get_balance(),
                    is_successful: withdrawal_result.is_successful,
                    error_message: withdrawal_result.error_message,
                }
            }
            _ => WithdrawWalletResponse {
                wallet_id: self.wallet_id.clone(),
                wallet_type: self.wallet_type.clone(),
                currency: currency.to_string(),
                amount,
                account_number: "".to_string(),
                account_type: AccountType::Basic, // Default account type
                balance: 0.0,
                is_successful: false,
                error_message: Some("Currency mismatch".to_string()),
            },
        }
    }
}