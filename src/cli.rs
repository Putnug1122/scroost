use clap::{arg, ArgAction, Command};

pub fn cli() -> Command {
    Command::new("scroost")
        .about("Scrape Product from Tokopedia")
        .version("v0.0.1")
        .arg(
            arg!(-q --query <VALUE>)
                .default_value("Samsung")
                .action(ArgAction::Set),
        )
        // Number of pages argument
        .arg(
            arg!(-p --page <VALUE>)
                .default_value("1")
                .action(ArgAction::Set),
        )
        // Output format argument
        .arg(
            arg!(-f --format <VALUE>)
                .default_value("json")
                .action(ArgAction::Set),
        )
}
