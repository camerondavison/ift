#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! { }
}
use crate::errors::*;
use clap::{
    crate_authors,
    crate_version,
    App,
    AppSettings,
    SubCommand,
};
use ift::{
    eval,
    rfc::WithRfc6890,
};

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() {
    let matches = App::new("ift")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("eval")
                .about("Evaluate an ift template")
                .args_from_usage("<template> 'Template string to evaluate'"),
        )
        .subcommand(
            SubCommand::with_name("rfc")
                .about("Dump rfc content")
                .args_from_usage("<name> 'rfc name to dump'"),
        )
        .get_matches();

    match matches.subcommand() {
        ("eval", Some(eval_matches)) => {
            let template = eval_matches.value_of("template").chain_err(|| "unable to find template argument")
            let ips: Vec<String> = eval(template).into_iter().map(|ip_addr| ip_addr.to_string()).collect();

            println!("[{}]", ips.join(" "));
        }
        ("rfc", Some(rfc_matches)) => {
            let name = rfc_matches.value_of("name").unwrap();
            let rfc = match name {
                "6890" => WithRfc6890::create(),
                _ => bail!("unknown rfc [{}]", name),
            };
            for entry in rfc.entries {
                println!("{:?}", entry)
            }
        }
        _ => bail!("unknown sub command"),
    }
}
