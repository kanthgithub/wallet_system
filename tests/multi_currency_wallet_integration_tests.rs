#[cfg(test)]
mod tests {
    use super::*;
    use wallet_system::accounts::{Account, AccountType};
    use wallet_system::basic_account::BasicAccount;
    use wallet_system::wallets::{Wallet, WalletType, TransferResponse};
    use wallet_system::basic_wallet::BasicWallet;
    use wallet_system::multi_currency_wallet::MultiCurrencyWallet;

    #[test]
    fn test_new_multi_currency_wallet() {
        let wallet = MultiCurrencyWallet::new();
        assert_eq!(wallet.get_wallet_type(), WalletType::MultiCurrency);
    }

    #[test]
    fn test_deposit_multi_currency_wallet() {
        let mut wallet = MultiCurrencyWallet::new();
        let account = BasicAccount::new("USD");
        wallet.add_account(Box::new(account)).expect("Failed to add account");
        let response = wallet.deposit("USD", 100.0);
        assert!(response.is_successful);
        assert_eq!(wallet.balance("USD").unwrap(), 100.0);
    }

    #[test]
    fn test_withdraw_multi_currency_wallet() {
        let mut wallet = MultiCurrencyWallet::new();
        let account = BasicAccount::new("USD");
        wallet.add_account(Box::new(account)).expect("Failed to add account");
        wallet.deposit("USD", 100.0);
        let response = wallet.withdraw("USD", 50.0);
        assert!(response.is_successful);
        assert_eq!(wallet.balance("USD").unwrap(), 50.0);
    }

    #[test]
    fn test_transfer_between_basic_and_multi_currency_wallet() {
        let account1 = BasicAccount::new("USD");
        let mut wallet1: BasicWallet<BasicAccount> = BasicWallet::new(account1);
        let mut wallet2 = MultiCurrencyWallet::new();
        let account2 = BasicAccount::new("USD");
        wallet2.add_account(Box::new(account2)).expect("Failed to add account");
        wallet1.deposit("USD", 100.0);
        let response = wallet1.transfer(&mut wallet2, "USD", 50.0);
        assert!(response.is_successful);
        assert_eq!(wallet1.balance("USD").unwrap(), 50.0);
        assert_eq!(wallet2.balance("USD").unwrap(), 50.0);
    }

    #[test]
    fn test_transfer_between_multi_currency_and_basic_wallet() {
        let account1 = BasicAccount::new("USD");
        let mut wallet1: BasicWallet<BasicAccount> = BasicWallet::new(account1);
        let mut wallet2 = MultiCurrencyWallet::new();
        let account2 = BasicAccount::new("USD");
        wallet2.add_account(Box::new(account2)).expect("Failed to add account");
        wallet2.deposit("USD", 100.0);
        let response = wallet2.transfer(&mut wallet1, "USD", 50.0);
        assert!(response.is_successful);
        assert_eq!(wallet2.balance("USD").unwrap(), 50.0);
        assert_eq!(wallet1.balance("USD").unwrap(), 50.0);
    }

    #[test]
    fn test_multi_currency_wallet_multiple_currencies() {
        let mut wallet = MultiCurrencyWallet::new();
        let account_usd = BasicAccount::new("USD");
        let account_eur = BasicAccount::new("EUR");
        wallet.add_account(Box::new(account_usd)).expect("Failed to add account");
        wallet.add_account(Box::new(account_eur)).expect("Failed to add account");
        wallet.deposit("USD", 100.0);
        wallet.deposit("EUR", 200.0);
        assert_eq!(wallet.balance("USD").unwrap(), 100.0);
        assert_eq!(wallet.balance("EUR").unwrap(), 200.0);
    }

    #[test]
    fn test_deposit_wrong_currency_multi_currency_wallet() {
        let mut wallet = MultiCurrencyWallet::new();
        let account = BasicAccount::new("USD");
        wallet.add_account(Box::new(account)).expect("Failed to add account");
        let response = wallet.deposit("EUR", 100.0);
        assert!(!response.is_successful);
        assert_eq!(response.error_message.unwrap(), "No account found with currency: EUR");
    }

    #[test]
    fn test_withdraw_wrong_currency_multi_currency_wallet() {
        let mut wallet = MultiCurrencyWallet::new();
        let account = BasicAccount::new("USD");
        wallet.add_account(Box::new(account)).expect("Failed to add account");
        wallet.deposit("USD", 100.0);
        let response = wallet.withdraw("EUR", 50.0);
        assert!(!response.is_successful);
        assert_eq!(response.error_message.unwrap(), "No account found with currency: EUR");
    }

    #[test]
    fn test_transfer_insufficient_funds_multi_currency_wallet() {
        let account1 = BasicAccount::new("USD");
        let mut wallet1: BasicWallet<BasicAccount> = BasicWallet::new(account1);
        let mut wallet2 = MultiCurrencyWallet::new();
        let account2 = BasicAccount::new("USD");
        wallet2.add_account(Box::new(account2)).expect("Failed to add account");
        let response = wallet1.transfer(&mut wallet2, "USD", 50.0);
        assert!(!response.is_successful);
        assert_eq!(response.error_message.unwrap(), "Insufficient funds");
    }

    #[test]
    fn test_transfer_wrong_currency_multi_currency_wallet() {
        let account1 = BasicAccount::new("USD");
        let mut wallet1: BasicWallet<BasicAccount> = BasicWallet::new(account1);
        let mut wallet2 = MultiCurrencyWallet::new();

        let account2 = BasicAccount::new("USD");
        let account = wallet2.add_account(Box::new(account2)).expect("Failed to add account");
        assert_eq!(account.get_currency(), "USD");

        wallet1.deposit("USD", 100.0);
        let response = wallet1.transfer(&mut wallet2, "EUR", 50.0);
        assert!(!response.is_successful);
        assert_eq!(response.error_message.unwrap(), "Source account currency: USD, Receiver account currency: EUR, mismatch");
    }
}