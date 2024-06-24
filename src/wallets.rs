use crate::accounts::{Account, AccountResponse, AccountType};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub enum WalletType {
    Basic,
    MultiCurrency,
}

pub struct TransferResponse {
    pub currency: String,
    pub amount: f64,
    pub sender_account_number: String,
    pub sender_wallet_id: String,
    pub sender_wallet_type: WalletType,
    pub recipient_account_number: String,
    pub recipient_wallet_id: String,
    pub recipient_wallet_type: WalletType,
    pub is_successful: bool,
    pub error_message: Option<String>,
}

pub struct WithdrawWalletResponse {
    pub wallet_id: String,
    pub wallet_type: WalletType,
    pub currency: String,
    pub amount: f64,
    pub account_number: String,
    pub account_type: AccountType,
    pub balance: f64,
    pub is_successful: bool,
    pub error_message: Option<String>,
}

// trait with functions that must be implemented by all wallets
pub trait Wallet {
    fn add_account(&mut self, account: Box<dyn Account>) -> Result<&dyn Account, String>;
    fn balance(&self, currency: &str) -> Result<f64, String>;

    fn get_wallet_id(&self) -> &str;

    fn get_wallet_type(&self) -> WalletType;

    fn find_account_index_by_currency(&self, currency: &str) -> Option<usize>;

    fn get_account_number_by_index(&self, index: usize) -> Option<&str>;

    fn get_account_by_currency(&self, currency: &str) -> Option<&dyn Account>;

    fn transfer(
        &mut self,
        to_wallet: &mut dyn Wallet,
        currency: &str,
        amount: f64,
    ) -> TransferResponse;

    fn deposit(&mut self, currency: &str, amount: f64) -> AccountResponse;

    fn withdraw(&mut self, currency: &str, amount: f64) -> WithdrawWalletResponse;
}

