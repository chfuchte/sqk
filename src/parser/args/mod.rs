use clap::{ArgAction, arg, command, crate_name, crate_version};

/// parsed arguments from the command line (without just-printing flags)
pub(crate) enum ParsedArgs {
    None,
}

pub fn parse_args() -> ParsedArgs {
    let matches = command!()
        // help
        .next_line_help(false)
        .after_help(rtfm_text())
        .after_long_help(rtfm_text())
        // version
        .disable_version_flag(true)
        .arg(arg!(-v --version "Print version").action(ArgAction::SetTrue))
        .arg(arg!(--license "Print license information").action(ArgAction::SetTrue))
        // matches
        .get_matches();

    if matches.get_flag("version") {
        println!(
            "{} {} ({} {})",
            crate_name!(),
            crate_version!(),
            env!("LAST_COMMIT_ID"),
            env!("LAST_COMMIT_DATE"),
        );
        std::process::exit(0);
    }

    if matches.get_flag("license") {
        println!(include_str!("../../../LICENSE.txt"));
        std::process::exit(0);
    }

    ParsedArgs::None
}

fn rtfm_text() -> String {
    format!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"))
}
