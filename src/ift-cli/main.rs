use clap::{crate_authors, crate_version, App, AppSettings, SubCommand};
use ift::eval;

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
        .get_matches();

    match matches.subcommand() {
        ("eval", Some(eval_matches)) => {
            let template = eval_matches.value_of("template").unwrap();
            eprintln!("template [{}]", template);
            println!("{:?}", eval(template));
        }
        _ => unreachable!(),
    }
}
