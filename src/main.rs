use structopt::StructOpt;
use exitfailure::ExitFailure;
pub mod proxy;
use proxy::proxy::{ApplicationArguments, Command, server, client};

fn main() -> Result<(), ExitFailure> {
    let opt = ApplicationArguments::from_args();
    let key = opt.command;
    let val:String;
    match key {
        Command::Server(s) => {
            val = server(s).unwrap();
        }

        Command::Client(s) => {
            val = client(s).unwrap();
        }
    };
    println!("{}", val);
    Ok(())
}
