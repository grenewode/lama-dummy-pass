use clap::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let matches = App::new("pass")
        .subcommand(SubCommand::with_name("ls"))
        .subcommand(SubCommand::with_name("show").args(&[Arg::with_name("pass-name")]))
        .subcommand(SubCommand::with_name("insert").args(&[
            Arg::with_name("echo").short("e"),
            Arg::with_name("multiline").short("m"),
            Arg::with_name("force").short("f"),
            Arg::with_name("pass-name"),
        ]))
        .get_matches();

    match matches.subcommand() {
        ("ls", Some(_cmd)) => println!("ls"),
        ("show", Some(cmd)) => println!("{:?}", cmd.value_of("pass-name")),
        ("insert", Some(cmd)) => {
            File::create("./out.txt")?.write_all(cmd.value_of("pass-name").unwrap().as_bytes())?;
        }
        _ => {}
    }

    Ok(())
}
