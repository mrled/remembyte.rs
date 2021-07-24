use std::{num::ParseIntError, vec::Vec};

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

use data_encoding::BASE64_NOPAD;

use thrussh_keys::{self, key::PublicKey};

mod maps;

fn main() {
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

    let mapname = arguments.value_of("mapname").unwrap();
    println!("mapname: {}", mapname);

    match arguments.subcommand() {
        ("ssh-pubkey", Some(sub_args)) => {
            println!("ssh-pubkey subcmd");
            let pubkey_path = sub_args.value_of("PUBKEY").unwrap();
            println!("Pubkey at {}", pubkey_path);
            let pubkey = thrussh_keys::load_public_key(pubkey_path).unwrap();
            println!("Pubkey: {:?}", pubkey);
            let fingerprint_str = pubkey.fingerprint();
            println!("Fingerprint string: {:?}", fingerprint_str);
            let fingerprint_bytes = BASE64_NOPAD.decode(fingerprint_str.as_bytes()).unwrap();
            println!("Fingerprint bytes: {:?}", fingerprint_bytes);
            let mapped = map(&fingerprint_bytes, name2composedbytemap(mapname));
            println!("{}", mapped);
        }
        _ => println!("No subcommand specified"),
    }
}

fn name2composedbytemap(name: &str) -> maps::ComposedBytemap {
    let cmdmap = match name {
        "emoji" => maps::ComposedBytemap {
            maps: vec![maps::WINDYTAN_EMOJI_MAP],
            separator: " :",
        },
        "pgp" => maps::ComposedBytemap {
            maps: vec![maps::PGP_WORDLIST_TWO_MAP, maps::PGP_WORDLIST_THREE_MAP],
            separator: ", ",
        },
        _ => panic!("Unknown map name: {}", name),
    };
    cmdmap
}

fn map(bytes: &[u8], cmpmap: maps::ComposedBytemap) -> String {
    let mut result = String::new();
    for (idx, &byte) in bytes.iter().enumerate() {
        let map = cmpmap.maps[idx % cmpmap.maps.len()];
        result.push_str(map[byte as usize]);
        if idx < bytes.len() - 1 {
            result.push_str(cmpmap.separator);
        }
    }
    result
}

pub fn hex2bytes(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
