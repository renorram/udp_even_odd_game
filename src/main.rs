mod server;
mod client;
mod game;

use std::env::args;
use std::process::exit;
use crate::client::run_client;
use crate::server::run_server;


fn main() {
    let mut args = args().skip(1);
    let mode_arg = args.next();

    if let Some(mode) = mode_arg {
        if mode.eq("client") {
            if let Err(err) = run_client(args.next()) {
                println!("An error happened: {:?}", err);
                exit(1);
            }
        }

        if mode.eq("server") {
            if let Err(err) = run_server() {
                println!("An error happened: {:?}", err);
                exit(1);
            }
        }

        println!("'{}' is not a valid run mode.", mode)
    }

    println!("You did not provide a run mode. modes available are 'client' and 'server'.");
    exit(1)
}