use clap::{crate_version, App, Arg, ArgMatches, SubCommand};

pub fn argue() -> ArgMatches<'static> {
    let arguments = App::new("remembyte - experiments in human-memorable byte arrays")
        .version(crate_version!())
        .author("Micah R Ledbetter")
        .about("Map bytes to items that are easier for humans to work with")
        .arg(
            Arg::with_name("mapname")
                .short("m")
                .long("mapname")
                .value_name("MAPNAME")
                .help("The map to use")
                .default_value("emoji"),
        )
        .subcommand(
            SubCommand::with_name("ssh-pubkey")
                .about("Show an SSH public key remembyte map")
                .arg(
                    Arg::with_name("PUBKEY")
                        .help("Path to a public key on the local filesystem")
                        .required(true),
                ),
        )
        .get_matches();
    return arguments;
}
