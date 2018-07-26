#[macro_use]
extern crate clap;
extern crate data_encoding;
extern crate dirs;
extern crate open;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use clap::{App, Arg, SubCommand};
use data_encoding::BASE32_NOPAD;

mod otp;
mod storage;

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new account")
                .arg(
                    Arg::with_name("account")
                        .required(true)
                        .help("Name of the account"),
                )
                .arg(
                    Arg::with_name("key")
                        .required(true)
                        .help("Secret key of the OTP"),
                )
                .arg(Arg::with_name("counter").required(true).help("Counter")),
        )
        .subcommand(
            SubCommand::with_name("view")
                .about("View the OTP for an account")
                .arg(
                    Arg::with_name("account")
                        .required(true)
                        .help("Name of the account"),
                )
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("length")
                        .takes_value(true)
                        .value_name("NUMBER")
                        .help("Length of the OTP"),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List OTP for all accounts"))
        .subcommand(
            SubCommand::with_name("edit").about("Edit an account").arg(
                Arg::with_name("account")
                    .required(true)
                    .help("Name of the account"),
            ),
        )
        .subcommand(
            SubCommand::with_name("recovery_codes")
                .about("View recovery codes for an account")
                .arg(
                    Arg::with_name("account")
                        .required(true)
                        .help("Name of the account"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_m)) => add_account(
            sub_m.value_of("account").unwrap(),
            sub_m.value_of("key").unwrap(),
            sub_m.value_of("counter").unwrap().parse::<u64>().unwrap(),
        ),
        ("view", Some(sub_m)) => {
            let length = match sub_m.value_of("length") {
                Some(length) => length.parse::<usize>().unwrap(),
                None => 6,
            };
            view_account(sub_m.value_of("account").unwrap(), length)
        }
        ("list", Some(_)) => list_accounts(),
        ("edit", Some(_sub_m)) => {}
        ("recovery_codes", Some(sub_m)) => view_recovery_codes(sub_m.value_of("account").unwrap()),
        _ => println!("No subcommand chosen"),
    }
}

fn view_recovery_codes(account_name: &str) {
    match storage::open_recovery_codes(account_name) {
        Ok(_) => {}
        Err(err) => println!("Error {}", err),
    };
}

fn add_account(_account_name: &str, _key: &str, _counter: u64) {}

fn view_account(account_name: &str, length: usize) {
    match storage::read() {
        Ok(accounts) => match accounts.get(account_name) {
            Some(account) => print_otp_code(account_name, account, Some(length)),
            None => println!(
                "Account with the name {} does not exist. Consider adding it.",
                account_name
            ),
        },
        Err(err) => println!("Error {}", err),
    };
}

fn list_accounts() {
    match storage::read() {
        Ok(accounts) => {
            for (name, account) in accounts {
                print_otp_code(&name, &account, None);
                println!("\n");
            }
        }
        Err(err) => println!("Error {}", err),
    };
}

fn print_otp_code(name: &str, account: &storage::Account, code_length: Option<usize>) {
    let decoded_key = BASE32_NOPAD.decode(account.key.as_bytes()).unwrap();
    let hash_function = match account.hash_function.as_ref() {
        "SHA1" => otp::HashFunction::SHA1,
        "SHA256" => otp::HashFunction::SHA256,
        "SHA384" => otp::HashFunction::SHA384,
        "SHA512" => otp::HashFunction::SHA512,
        "SHA512_256" => otp::HashFunction::SHA512_256,
        _ => otp::HashFunction::SHA1,
    };
    let otp = otp::OTP::new(
        decoded_key,
        account.totp,
        hash_function,
        account.counter,
        code_length,
    );

    if account.totp {
        println!("Account: {}\nTOTP: {}", name, otp.generate());
    } else {
        println!("Account: {}\nHOTP: {}", name, otp.generate());
    }
}
