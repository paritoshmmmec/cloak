use clap::{App, Arg, ArgMatches, SubCommand};
use fs;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("Delete an account")
        .arg(
            Arg::with_name("account")
                .required(true)
                .help("Name of the account"),
        )
}

pub fn run(args: &ArgMatches) {
    let account_name = args.value_of("account").unwrap();
    match fs::read() {
        Ok(mut accounts) => {
            if accounts.get(account_name).is_some() {
                accounts.remove(account_name);
                match fs::write(&accounts) {
                    Ok(_) => println!("Account successfully deleted"),
                    Err(err) => println!("Error {}", err),
                };
            } else {
                println!("Account does not exist");
            }
        }
        Err(err) => println!("Error {}", err),
    }
}
