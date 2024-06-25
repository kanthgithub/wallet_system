#[cfg(test)]
mod tests {
    use super::*;
    use wallet_system::accounts::{Account, AccountType, DisplayAccount};
    use wallet_system::basic_account::BasicAccount;
    use wallet_system::premium_account::PremiumAccount;

    #[test]
    fn test_new_account() {
        let account = PremiumAccount::new("USD", 500.0);
        assert_eq!(account.get_balance(), 0.0);
        assert_eq!(account.get_currency(), "USD");
        assert_eq!(account.get_account_type(), AccountType::Premium);
    }

    #[test]
    fn test_withdraw_success() {
        let mut account = PremiumAccount::new("USD", 500.0);
        account.deposit(200.0);
        let response = account.withdraw(100.0);
        assert_eq!(response.is_successful, true);
        assert_eq!(account.get_balance(), 100.0);
    }

    #[test]
    fn test_withdraw_fail() {
        let mut account = PremiumAccount::new("USD", 500.0);
        let response = account.withdraw(500.1);
        assert_eq!(response.is_successful, false);
    }

    #[test]
    fn test_transfer_from_premium_to_basic() {
        let mut account1 = PremiumAccount::new("USD", 500.0);
        let mut account2 = BasicAccount::new("USD");
        account1.deposit(200.0);
        let response = account1.transfer(&mut account2, 700.0);
        assert_eq!(response.is_successful, true);
        assert_eq!(account1.get_balance(), -500.0);
        println!("Balance1: {}", format!("{:.2}", account1.get_balance()));
        println!("Balance2: {}", format!("{:.2}", account2.get_balance()));
        assert_eq!(account2.get_balance(), 700.0);
    }

    #[test]
    fn test_transfer_from_premium_to_premium() {
        let mut premium_account_1 = PremiumAccount::new("USD", 500.0);
        let mut premium_account_2 = PremiumAccount::new("USD", 500.0);
        premium_account_1.deposit(200.0);
        premium_account_1.transfer(&mut premium_account_2, 700.0);
        assert_eq!(premium_account_1.get_balance(), -500.0);
        assert_eq!(premium_account_2.get_balance(), 700.0);
        assert_eq!(premium_account_1.get_account_type(), AccountType::Premium);
        assert_eq!(premium_account_2.get_account_type(), AccountType::Premium);
        assert_eq!(premium_account_1.get_overdraft_limit(), 500.0);
        assert_eq!(premium_account_2.get_overdraft_limit(), 500.0);
    }

    #[test]
    fn test_withdraw_overdraft() {
        let mut account = PremiumAccount::new("USD", 500.0);
        account.deposit(200.0);
        let response = account.withdraw(600.0);
        assert_eq!(response.is_successful, true);
        assert_eq!(account.get_balance(), -400.0);
    }

    #[test]
    fn test_withdraw_overdraft_limit_exceeded() {
        let mut account = PremiumAccount::new("USD", 500.0);
        account.deposit(200.0);
        let response = account.withdraw(700.1);
        assert_eq!(response.is_successful, false);
    }

    #[test]
    fn test_transfer_fail() {
        let mut account1 = PremiumAccount::new("USD", 500.0);
        let mut account2 = PremiumAccount::new("USD", 500.0);
        let response = account1.transfer(&mut account2, 600.0);
        assert_eq!(response.is_successful, false);
    }
}