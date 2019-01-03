use clap::{
    crate_authors,
    crate_version,
    App,
    AppSettings,
    SubCommand,
};
use ift::rfc::{
    Rfc6890Entry,
    RfcEntry::Rfc6890,
};

mod rfc_parser;

fn main() {
    let matches = App::new("gen")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("rfc")
                .about("generate rust code for rfc content")
                .args_from_usage("<name> 'rfc name to dump'"),
        )
        .get_matches();

    match matches.subcommand() {
        ("rfc", Some(rfc_matches)) => {
            let name = rfc_matches.value_of("name").unwrap();
            let info = match name {
                "6890" => rfc_parser::parse_tables(include_str!("rfc6890_entries.txt")),
                _ => unimplemented!("unknown rfc [{}]", name),
            };

            for entry in info {
                match entry.output {
                    Rfc6890(r) => {
                        if r.termination_date != "N/A" {
                            println!(r"/*{},*/", as_code(&r));
                        } else {
                            println!("{},", as_code(&r));
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }
}

fn escape_quotes(s: &str) -> String { s.replace('"', r#"\""#) }

fn as_code(entry: &Rfc6890Entry) -> String {
    format!(
        "\
Rfc6890Entry {{
    address_block: \"{}\".parse().unwrap(),
    name: \"{}\".to_owned(),
    rfc: \"{}\".to_owned(),
    allocation_date: \"{}\".to_owned(),
    termination_date: \"{}\".to_owned(),
    source: {},
    destination: {},
    forwardable: {},
    global: {},
    reserved_by_protocol: {}
}}",
        entry.address_block,
        escape_quotes(&entry.name),
        escape_quotes(&entry.rfc),
        escape_quotes(&entry.allocation_date),
        escape_quotes(&entry.termination_date),
        entry.source,
        entry.destination,
        entry.forwardable,
        entry.global,
        entry.reserved_by_protocol
    )
}
