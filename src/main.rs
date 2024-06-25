use clap::{Arg, App, SubCommand};
use wallet_system::accounts::{BasicAccount, PremiumAccount, Account};
use wallet_system::wallets::{BasicWallet, MultiCurrencyWallet, Wallet};

fn main() {
    let matches = App::new("Wallet CLI")
        .version("1.0")
        .author("kanth")
        .about("Manages wallets and accounts")
        .subcommand(
            SubCommand::with_name("create_wallet")
                .about("Creates a new wallet")
                .arg(Arg::with_name("type")
                    .help("The type of wallet to create (basic or multi)")
                    .required(true)
                    .index(1)),
        )
        .subcommand(
            SubCommand::with_name("create_account")
                .about("Creates a new account in a wallet")
                .arg(Arg::with_name("wallet_id")
                    .help("The ID of the wallet")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("type")
                    .help("The type of account to create (basic or premium)")
                    .required(true)
                    .index(2))
                .arg(Arg::with_name("currency")
                    .help("The currency of the account")
                    .required(true)
                    .index(3))
                .arg(Arg::with_name("overdraft")
                    .help("The overdraft limit for premium accounts")
                    .required(false)
                    .index(4)),
        )
        .subcommand(
            SubCommand::with_name("deposit")
                .about("Deposits money into an account")
                .arg(Arg::with_name("wallet_id")
                    .help("The ID of the wallet")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("amount")
                    .help("The amount to deposit")
                    .required(true)
                    .index(2))
                .arg(Arg::with_name("currency")
                    .help("The currency of the account")
                    .required(true)
                    .index(3)),
        )
        .subcommand(
            SubCommand::with_name("withdraw")
                .about("Withdraws money from an account")
                .arg(Arg::with_name("wallet_id")
                    .help("The ID of the wallet")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("amount")
                    .help("The amount to withdraw")
                    .required(true)
                    .index(2))
                .arg(Arg::with_name("currency")
                    .help("The currency of the account")
                    .required(true)
                    .index(3)),
        )
        .subcommand(
            SubCommand::with_name("transfer")
                .about("Transfers money between accounts")
                .arg(Arg::with_name("from_wallet_id")
                    .help("The ID of the source wallet")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("to_wallet_id")
                    .help("The ID of the destination wallet")
                    .required(true)
                    .index(2))
                .arg(Arg::with_name("amount")
                    .help("The amount to transfer")
                    .required(true)
                    .index(3))
                .arg(Arg::with_name("currency")
                    .help("The currency of the accounts")
                    .required(true)
                    .index(4)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create_wallet") {
        let wallet_type = matches.value_of("type").unwrap();
        match wallet_type {
            "basic" => {
                let wallet = BasicWallet::new();
                println!("Created a BasicWallet with ID: {}", wallet.get_wallet_id());
            },
            "multi" => {
                let wallet = MultiCurrencyWallet::new();
                println!("Created a MultiCurrencyWallet with ID: {}", wallet.get_wallet_id());
            },
            _ => println!("Invalid wallet type"),
        }
    } else if let Some(matches) = matches.subcommand_matches("create_account") {
        let wallet_id = matches.value_of("wallet_id").unwrap();
        let account_type = matches.value_of("type").unwrap();
        let currency = matches.value_of("currency").unwrap();
        let overdraft = matches.value_of("overdraft");

        // Handle creating accounts in the specified wallet here
        // For simplicity, this example does not store wallets persistently
        println!(
            "Creating a {} account with currency {} in wallet {}",
            account_type, currency, wallet_id
        );

        // Additional logic to add the account to the specified wallet would go here
    } else if let Some(matches) = matches.subcommand_matches("deposit") {
        let wallet_id = matches.value_of("wallet_id").unwrap();
        let amount: f64 = matches.value_of("amount").unwrap().parse().unwrap();
        let currency = matches.value_of("currency").unwrap();

        // Handle depositing into the specified account here
        println!(
            "Depositing {} {} into wallet {}",
            amount, currency, wallet_id
        );

        // Additional logic to perform the deposit would go here
    } else if let Some(matches) = matches.subcommand_matches("withdraw") {
        let wallet_id = matches.value_of("wallet_id").unwrap();
        let amount: f64 = matches.value_of("amount").unwrap().parse().unwrap();
        let currency = matches.value_of("currency").unwrap();

        // Handle withdrawing from the specified account here
        println!(
            "Withdrawing {} {} from wallet {}",
            amount, currency, wallet_id
        );

        // Additional logic to perform the withdrawal would go here
    } else if let Some(matches) = matches.subcommand_matches("transfer") {
        let from_wallet_id = matches.value_of("from_wallet_id").unwrap();
        let to_wallet_id = matches.value_of("to_wallet_id").unwrap();
        let amount: f64 = matches.value_of("amount").unwrap().parse().unwrap();
        let currency = matches.value_of("currency").unwrap();

        // Handle transferring between the specified accounts here
        println!(
            "Transferring {} {} from wallet {} to wallet {}",
            amount, currency, from_wallet_id, to_wallet_id
        );
    }
}
