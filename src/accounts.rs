use rand::{distributions::Alphanumeric, Rng};
use std::fmt;

/// Enum defining account types
#[derive(Clone, Debug, PartialEq)]
pub enum AccountType {
    Basic,
    Premium,
}


/// Struct defining the response for account operations
#[derive(Debug)]
pub struct AccountResponse {
    pub account_number: String,
    pub account_type: AccountType,
    pub currency: String,
    pub balance: f64,
    pub is_successful: bool,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct AccountTransferResponse {
    pub account_number: String,
    pub account_type: AccountType,
    pub currency: String,
    pub recipient_account_number: String,
    pub recipient_account_type: AccountType,
    pub balance: f64,
    pub is_successful: bool,
    pub error_message: Option<String>,
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountType::Basic => write!(f, "Basic"),
            AccountType::Premium => write!(f, "Premium"),
        }
    }
}

pub trait DisplayAccount {
    fn display_details(&self);
}

/// Trait defining common account operations
pub trait Account : DisplayAccount {
    fn get_balance(&self) -> f64;
    fn deposit(&mut self, amount: f64) -> AccountResponse;
    fn withdraw(&mut self, amount: f64) -> AccountResponse;
    fn get_currency(&self) -> &str;
    fn get_account_number(&self) -> &str;
    fn get_account_type(&self) -> AccountType;
    fn transfer(&mut self, to_account: &mut dyn Account, amount: f64) -> AccountTransferResponse;
    fn get_overdraft_limit(&self) -> f64;
}