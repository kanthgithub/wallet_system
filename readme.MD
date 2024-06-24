# Project Overview

This project is a banking system simulation written in Rust. It includes entities such as Wallets and Accounts, and uses traits, generics, and polymorphism to provide a flexible and extensible design.

## Entities

### Wallet

A Wallet can hold multiple accounts. It has a unique `wallet_id` and a `wallet_type` which can be either `Basic` or `MultiCurrency`.

### Account

An Account represents a bank account with a specific currency. 
It has a unique `account_number` and an `account_type` which can be either `Basic` or `Premium`.


`BasicWallet` and `MultiCurrencyWallet` are two different implementations of the `Wallet` trait, 
each with its own unique characteristics.

### BasicWallet

`BasicWallet` is a simple wallet that can hold only one account. 
It uses generics to allow for different types of accounts to be added to the wallet. 
The account is set during the creation of the `BasicWallet`, and it cannot be changed afterwards. 
This design is suitable for scenarios where a wallet is expected to have only one account.

### MultiCurrencyWallet

`MultiCurrencyWallet`, on the other hand, can hold multiple accounts, each with a different currency. 
Unlike `BasicWallet`, it does not use generics. Instead, it uses a vector to store multiple boxed `Account` trait objects. 
This allows for different types of accounts to be added to the wallet after its creation. 
This design is suitable for scenarios where a wallet is expected to handle multiple currencies.

The use of generics in `BasicWallet` and not in `MultiCurrencyWallet` is due to their different requirements. 
`BasicWallet` needs to know the exact type of its account at compile time, hence the use of generics. 
`MultiCurrencyWallet` needs to handle multiple accounts of potentially different types, hence the use of trait objects.


## Traits, Generics, and Polymorphism

### Traits

Traits are used to define shared behavior. In this project, the `Wallet` and `Account` traits define a common 
interface for all types of wallets and accounts.

### Generics

Generics are used in the `Wallet` trait to allow for different types of accounts to be added to a wallet.

### Polymorphism

Polymorphism is used to allow for different types of wallets and accounts to be treated the same way. 
For example, a `BasicWallet` and a `MultiCurrencyWallet` can both be treated as a `Wallet`.

## Functions

### Wallet Functions

- `add_account`: Adds an account to the wallet.
- `balance`: Returns the balance of the wallet for a specific currency.
- `get_wallet_id`: Returns the unique ID of the wallet.
- `get_wallet_type`: Returns the type of the wallet.
- `find_account_index_by_currency`: Finds the index of an account in the wallet by currency.
- `get_account_number_by_index`: Returns the account number of an account at a specific index in the wallet.
- `get_account_by_currency`: Returns an account in the wallet by currency.
- `transfer`: Transfers money from the wallet to another wallet.
- `deposit`: Deposits money into the wallet.
- `withdraw`: Withdraws money from the wallet.

### Account Functions

- `get_account_number`: Returns the unique account number.
- `get_account_type`: Returns the type of the account.
- `get_balance`: Returns the balance of the account.
- `get_currency`: Returns the currency of the account.
- `deposit`: Deposits money into the account.
- `withdraw`: Withdraws money from the account.

## Enums

### WalletType

This enum represents the type of a wallet. It can be either `Basic` or `MultiCurrency`.

### AccountType

This enum represents the type of an account. It can be either `Basic` or `Premium`.

## ID Generation

The `wallet_id` and `account_number` are unique identifiers for wallets and accounts respectively. Their generation is not shown in the provided code, but they should be generated in a way that ensures their uniqueness.

# Building and Testing

To build the project, navigate to the project directory and run the following command:

```bash
cargo build
```

To run the tests, use the following command:

```bash
cargo test
```

Please ensure that you have Rust and Cargo installed on your system before building or testing the project.