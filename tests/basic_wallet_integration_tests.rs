#[cfg(test)]
mod tests {
    use super::*;
    use wallet_system::accounts::AccountType;
    use wallet_system::wallets::{Wallet, WalletType, TransferResponse};
    use wallet_system::basic_account::BasicAccount;
    use wallet_system::basic_wallet::BasicWallet;

    #[test]
    fn test_new_wallet() {
        let account = BasicAccount::new("USD");
        let wallet: BasicWallet<BasicAccount> = BasicWallet::new(account);
        assert_eq!(wallet.get_wallet_type(), WalletType::Basic);
        assert_eq!(wallet.balance("USD").unwrap(), 0.0);
    }

    #[test]
    fn test_deposit() {
        let account = BasicAccount::new("USD");
        let mut wallet: BasicWallet<BasicAccount> = BasicWallet::new(account);
        let response = wallet.deposit("USD", 100.0);
        assert!(response.is_successful);
        assert_eq!(wallet.balance("USD").unwrap(), 100.0);
    }

    #[test]
    fn test_withdraw() {
        let account = BasicAccount::new("USD");
        let mut wallet: BasicWallet<BasicAccount> = BasicWallet::new(account);
        wallet.deposit("USD", 100.0);
        let response = wallet.withdraw("USD", 50.0);
        assert!(response.is_successful);
        assert_eq!(wallet.balance("USD").unwrap(), 50.0);
    }

    #[test]
    fn test_transfer() {
        let account1 = BasicAccount::new("USD");
        let mut wallet1: BasicWallet<BasicAccount> = BasicWallet::new(account1);
        let account2 = BasicAccount::new("USD");
        let mut wallet2: BasicWallet<BasicAccount> = BasicWallet::new(account2);
        wallet1.deposit("USD", 100.0);
        let response = wallet1.transfer(&mut wallet2, "USD", 50.0);
        assert!(response.is_successful);
        assert_eq!(wallet1.balance("USD").unwrap(), 50.0);
        assert_eq!(wallet2.balance("USD").unwrap(), 50.0);
    }

    #[test]
    fn test_deposit_wrong_currency() {
        let account = BasicAccount::new("USD");
        let mut wallet: BasicWallet<BasicAccount> = BasicWallet::new(account);
        let response = wallet.deposit("EUR", 100.0);
        assert!(!response.is_successful);
        assert_eq!(response.error_message.unwrap(), "Currency mismatch");
    }

    #[test]
    fn test_withdraw_wrong_currency() {
        let account = BasicAccount::new("USD");
        let mut wallet: BasicWallet<BasicAccount> = BasicWallet::new(account);
        wallet.deposit("USD", 100.0);
        let response = wallet.withdraw("EUR", 50.0);
        assert!(!response.is_successful);
        assert_eq!(response.error_message.unwrap(), "Currency mismatch");
    }

    #[test]
    fn test_transfer_insufficient_funds() {
        let account1 = BasicAccount::new("USD");
        let mut wallet1: BasicWallet<BasicAccount> = BasicWallet::new(account1);
        let account2 = BasicAccount::new("USD");
        let mut wallet2: BasicWallet<BasicAccount> = BasicWallet::new(account2);
        let response = wallet1.transfer(&mut wallet2, "USD", 50.0);
        assert!(!response.is_successful);
        assert_eq!(response.error_message.unwrap(), "Insufficient funds");
    }

    #[test]
    fn test_transfer_wrong_currency() {
        let account1 = BasicAccount::new("USD");
        let mut wallet1: BasicWallet<BasicAccount> = BasicWallet::new(account1);
        let account2 = BasicAccount::new("USD");
        let mut wallet2: BasicWallet<BasicAccount> = BasicWallet::new(account2);
        wallet1.deposit("USD", 100.0);
        let response = wallet1.transfer(&mut wallet2, "EUR", 50.0);
        assert!(!response.is_successful);
        assert_eq!(response.error_message.unwrap(), "Source account currency: USD, Receiver account currency: EUR, mismatch");
    }
}