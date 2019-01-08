use clap::{
    crate_authors,
    crate_version,
    App,
    AppSettings,
    SubCommand,
};
use failure::{
    bail,
    Error,
};
use ift::{
    eval,
    rfc::WithRfc6890,
};

fn main() {
    if let Err(ref e) = run() {
        eprintln!("error: {}", e);
        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
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
            let template = eval_matches.value_of("template").unwrap();
            let ips: Vec<String> = eval(template)?.into_iter().map(|ip_addr| ip_addr.to_string()).collect();

            println!("[{}]", ips.join(" "));
            Ok(())
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
            Ok(())
        }
        _ => bail!("unknown sub command"),
    }
}
