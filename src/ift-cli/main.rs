use clap::{crate_authors, crate_version, App, AppSettings, SubCommand};
use ift::eval;
use ift::rfc_parser;

fn main() {
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
            println!("{:?}", eval(template));
        }
        ("rfc", Some(rfc_matches)) => {
            let name = rfc_matches.value_of("name").unwrap();
            let info = match name {
                "6890" => rfc_parser::parse_tables(include_str!("rfc6890_entries.txt")),
                _ => unimplemented!("unknown rfc [{}]", name)
            };
            println!("{:?}", info)
        }
        _ => unreachable!(),
    }
}
