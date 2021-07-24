# Remembyte

Experiments in human-memorable byte arrays.

Version 2!

## What is this?

A command to map raw bytes to something easier to remember, like words or emoji.

For instance, in the default `emoji` mapping, a short array of three bytes `[0x1, 0x2, 0x3]` would map to `🌀: 🌂: 🌅`. It can also map to both the two and three syllable versions of the [PGP word list](https://en.wikipedia.org/wiki/PGP_word_list).

It's very basic right now. Currently it can show a mapping of the fingerprint of an SSH public key from local disk. Eventually it will be able to connect to remote SSH servers and show a mapping of the host key fingerprint, show a mapping of the fingerprint of a local or remote TLS server (like HTTPS), and accept hex input.

## How do you use it?

It's a command-line app. Here's how I run it with `cargo` in development:

```
> cargo run -- -h
   Compiling remembyte v2.0.0 (/Users/mrled/Documents/Repositories/remembyte.rs)
    Finished dev [unoptimized + debuginfo] target(s) in 2.81s
     Running `target/debug/remembyte -h`
remembyte - experiments in human-memorable byte arrays 2.0.0
Micah R Ledbetter
Map bytes to items that are easier for humans to work with

USAGE:
    remembyte [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --mapname <MAPNAME>    The map to use [default: emoji]

SUBCOMMANDS:
    help          Prints this message or the help of the given subcommand(s)
    ssh-pubkey    Show an SSH public key remembyte map

> cargo run -- ssh-pubkey ./id_ecdsa_TEST.pub
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/remembyte ssh-pubkey ./id_ecdsa_TEST.pub`
mapname: emoji
ssh-pubkey subcmd
Pubkey at ./id_ecdsa_TEST.pub
Pubkey: RSA { key: OpenSSLPKey { (hidden) }, hash: SHA2_256 }
Fingerprint string: "UmAraj5LC3Brl27N5+QyjTxzEIFS4q2Vw6BOiV6JDbY"
Fingerprint bytes: [82, 96, 43, 106, 62, 75, 11, 112, 107, 151, 110, 205, 231, 228, 50, 141, 60, 115, 16, 129, 82, 226, 173, 149, 195, 160, 78, 137, 94, 137, 13, 182]
🎁 :🎓 :🍔 :🎩 :🍪 :🍷 :🌳 :🎯 :🎪 :🐎 :🎭 :👆 :👠 :👝 :🍞 :🐄 :🍨 :🎲 :🌹 :🏂 :🎁 :👛 :🐤 :🐌 :🐺 :🐗 :🍺 :🐀 :🎏 :🐀 :🌵 :🐭

> cargo run -- -m pgp ssh-pubkey ./id_ecdsa_TEST.pub
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/remembyte -m pgp ssh-pubkey ./id_ecdsa_TEST.pub`
mapname: pgp
ssh-pubkey subcmd
Pubkey at ./id_ecdsa_TEST.pub
Pubkey: RSA { key: OpenSSLPKey { (hidden) }, hash: SHA2_256 }
Fingerprint string: "UmAraj5LC3Brl27N5+QyjTxzEIFS4q2Vw6BOiV6JDbY"
Fingerprint bytes: [82, 96, 43, 106, 62, 75, 11, 112, 107, 151, 110, 205, 231, 228, 50, 141, 60, 115, 16, 129, 82, 226, 173, 149, 195, 160, 78, 137, 94, 137, 13, 182]
Dupont, Galveston, briefcase, hazardous, concert, divisive, alone, hurricane, Geiger, newsletter, glucose, sardonic, trauma, travesty, checkup, millionaire, cobra, impetus, assume, Jupiter, Dupont, torpedo, reward, narrative, snapshot, pandemic, drifter, megaton, eyeglass, megaton, ancient, provincial
```

## TODO

- Implement `remembyte ssh TARGET`, where TARGET can be either a local public key file or a host to SSH to.
- Implement `remembyte tls TARGET`, where TARGET can be either a local public certificate file or a host to connect to over TLS.
- Implement `remembyte hex DATA`, where DATA is a string of hex numbers.
- Implement `remembyte hex < DATA`, where it reads DATA from stdin (including over a pipe).
- Add testing.
- Publish a crate on the Internet somehow.
- Add a three wordlist mapping of noun (subject), adverb, verb, noun (object).
- Implement nicer debug logging

## History

I wrote the original `remembyte` back in 2014 in C: <https://github.com/mrled/remembyte>

It was a C learning project for me, and also intended to scratch an itch - it was a continual annoyance when I was using a new machine that it couldn't validate SSH keys.

Today, I decided I wanted to learn Rust, and what better way than to rewrite the experiment I used to lean C. This time around, I don't expect it'll be a useful way to manage SSH keys most of the time, but I really would like to implement my idea for using nouns, verbs, and adverbs to generate sentences.
