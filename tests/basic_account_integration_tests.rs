use wallet_system::wallets::{Wallet, WalletType, TransferResponse};
use wallet_system::basic_account::BasicAccount;
use wallet_system::accounts::Account;

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use wallet_system::accounts::AccountType;
    use super::*;

    #[test]
    fn test_new_account() {
        let account = BasicAccount::new("USD");
        assert_eq!(account.get_balance(), 0.0);
        assert_eq!(account.get_currency(), "USD");
        // assert the accountId as it must start with prefix 'Basic'
        assert!(account.get_account_number().starts_with(&format!("{}", AccountType::Basic)));
    }

    #[test]
    fn test_deposit() {
        let mut account = BasicAccount::new("USD");
        let response = account.deposit(100.0);
        assert!(response.is_successful);
        assert_eq!(account.get_balance(), 100.0);
    }

    #[test]
    fn test_deposit_negative_amount() {
        let mut account = BasicAccount::new("USD");
        let response = account.deposit(-100.0);
        assert!(!response.is_successful);
        assert_eq!(account.get_balance(), 0.0);
    }

    #[test]
    fn test_withdraw() {
        let mut account = BasicAccount::new("USD");
        account.deposit(100.0);
        let response = account.withdraw(50.0);
        assert!(response.is_successful);
        assert_eq!(account.get_balance(), 50.0);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut account = BasicAccount::new("USD");
        let response = account.withdraw(50.0);
        assert!(!response.is_successful);
        assert_eq!(account.get_balance(), 0.0);
    }

    #[test]
    fn test_withdraw_negative_amount() {
        let mut account = BasicAccount::new("USD");
        account.deposit(100.0);
        let response = account.withdraw(-50.0);
        assert!(!response.is_successful);
        assert_eq!(account.get_balance(), 100.0);
    }

    //add tests for failed deposit and withdraw
    #[test]
    fn test_failed_deposit() {
        let mut account = BasicAccount::new("USD");
        let response = account.deposit(-100.0);
        assert!(!response.is_successful);
        assert_eq!(account.get_balance(), 0.0);
    }

    #[test]
    fn test_failed_withdraw() {
        let mut account = BasicAccount::new("USD");
        let response = account.withdraw(-100.0);
        assert!(!response.is_successful);
        assert_eq!(account.get_balance(), 0.0);
    }

    // #[test]
    // fn test_transfer() {
    //     let mut account1 = BasicAccount::new("USD");
    //     let mut account2 = BasicAccount::new("USD");
    //     account1.deposit(100.0);
    //     let response = account1.transfer(&mut account2, 50.0);
    //     assert!(response.is_successful);
    //     assert_eq!(account1.get_balance(), 50.0);
    //     assert_eq!(account2.get_balance(), 50.0);
    // }
}