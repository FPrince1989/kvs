use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").required(true))
                .arg(Arg::with_name("VALUE").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .get_matches();
    if let Some(get_matches) = matches.subcommand_matches("get") {
        let key = get_matches.value_of("KEY");
        println!("{}", key.unwrap());
        eprintln!("unimplemented");
        std::process::exit(1);
    } else if let Some(set_matches) = matches.subcommand_matches("set") {
        let key = set_matches.value_of("KEY");
        let value = set_matches.value_of("VALUE");
        println!("{} {}", key.unwrap(), value.unwrap());
        eprintln!("unimplemented");
        std::process::exit(1);
    } else if let Some(rm_matches) = matches.subcommand_matches("rm") {
        let key = rm_matches.value_of("KEY");
        println!("{}", key.unwrap());
        eprintln!("unimplemented");
        std::process::exit(1);
    } else {
        std::process::exit(1);
    }
}
