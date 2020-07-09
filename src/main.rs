mod cli;
mod db;
mod error;

use error::{Error, Result};

fn check(msg: &str, expected: &str) -> Result<bool> {
    use std::io::prelude::*;

    print!("{}", msg);
    std::io::stdout().flush()?;

    let mut reply = String::new();
    std::io::stdin().read_line(&mut reply)?;

    Ok(reply.eq_ignore_ascii_case(expected))
}

fn main() -> Result<()> {
    use structopt::StructOpt;

    let opt = cli::Cli::from_args();

    let (mut db, db_path): (db::Database, Option<std::path::PathBuf>) =
        if let Some(db_opt) = &opt.db {
            let db_opt: serde_json::Value = serde_json::from_str(db_opt)
                .unwrap_or_else(|_| serde_json::Value::String(db_opt.to_string()));

            if let Some(db_path) = db_opt.as_str().map(std::path::Path::new) {
                (
                    if db_path.exists() {
                        serde_json::from_reader(std::fs::File::open(db_path)?)?
                    } else {
                        db::Database::default()
                    },
                    Some(db_path.to_path_buf()),
                )
            } else {
                (serde_json::from_value(db_opt)?, None)
            }
        } else {
            (db::Database::default(), None)
        };

    if !opt.quiet {
        eprintln!("BEWARE! THIS IS NOT THE REAL PASS (https://www.passwordstore.org/), BUT ONLY A CLEVER IMPOSTER!");
        eprintln!("If you did not expect to see this message, STOP doing whatever you're doing! You've been bamboozeled!");
        if let Some(db_path) = db_path.as_ref() {
            eprintln!("Warning: database will be saved and loaded from {:?}. THE DATABASE IS NOT ENCRYPTED!", db_path);
        } else if opt.db.is_some() {
            eprintln!("Warning: database will be loaded from the environment variable IMPOSTER_PASS_DB. THE DATABASE IS NOT ENCRYPTED and changes WILL NOT BE SAVED!");
        } else {
            eprintln!("Warning: no database specified. Defaulting to empty database. Changes WILL NOT BE SAVED!");
        }
    }

    match opt.cmd {
        cli::Cmd::List { name } => {
            for i in db.ls(name.as_ref())? {
                println!("{:?}", i);
            }
        }
        cli::Cmd::Show { name } => match name {
            Some(name) => {
                if let Some(entry) = db.get(&name)? {
                    print!("{}", entry.data)
                }
            }
            _ => {
                for i in db.ls(Option::<std::path::PathBuf>::None)? {
                    println!("{:?}", i);
                }
            }
        },
        cli::Cmd::Insert {
            name,
            echo,
            multiline,
            force,
        } => {
            use std::collections::btree_map::Entry;
            use std::io::Write;

            let entry = db.entry(&name)?;

            if !force {
                match &entry {
                    Entry::Vacant(_) => {}
                    Entry::Occupied(e) => {
                        if !check(
                            &format!(
                                "An entry already exists for {}. Overwrite it? [y/N] ",
                                e.key().display()
                            ),
                            "y",
                        )? {
                            return Ok(());
                        }
                    }
                }
            }

            let password = if echo {
                print!("Enter password for {}: ", entry.key().display());
                std::io::stdout().flush()?;
                let mut reply = String::new();
                std::io::stdin().read_line(&mut reply)?;
                reply
            } else if multiline {
                use std::io::Read;
                println!(
                    "Enter contents of {} and press Ctrl+D when finished:\n",
                    entry.key().display()
                );
                std::io::stdout().flush()?;

                let mut reply = String::new();
                std::io::stdin().read_to_string(&mut reply)?;
                reply
            } else {
                let password = rpassword::read_password_from_tty(Some(&format!(
                    "Enter password for {}: ",
                    entry.key().display()
                )))?;
                let retype_password = rpassword::read_password_from_tty(Some(&format!(
                    "Retype password for {}: ",
                    entry.key().display()
                )))?;

                if password != retype_password {
                    eprintln!("Error: the entered passwords do not match.");
                    return Ok(());
                }

                password
            };

            entry.or_default().data = password;
        }
        cli::Cmd::Remove {
            recursive,
            force,
            name,
        } => {
            fn delete_check(path: &std::path::Path) -> Result<bool> {
                check(
                    &format!(
                        "Are you sure you would like to delete {}? [y/N] ",
                        path.display()
                    ),
                    "y",
                )
            }
            let count = db.ls(Some(name.as_path()))?.count();
            match count {
                0 => return Err(Error::NotInStore(name)),
                1 => {
                    if force || delete_check(name.as_path())? {
                        db.remove(&name)?;
                    }
                }
                _ => {
                    if recursive {
                        if force || delete_check(name.as_path())? {
                            db.remove(&name)?;
                        }
                    } else {
                        return Err(Error::IsADirectory(name));
                    }
                }
            }
        }
        cli::Cmd::DumpDb { pretty } => println!(
            "{}",
            if pretty {
                serde_json::to_string_pretty(&opt.db)?
            } else {
                serde_json::to_string(&opt.db)?
            }
        ),
        cli::Cmd::Fool { program, arguments } => {
            let exec_dir = std::env::current_exe()?
                .parent()
                .unwrap_or_else(|| std::path::Path::new("/"))
                .to_path_buf();

            let path = match std::env::var_os("PATH") {
                Some(path) => {
                    let mut new_path = exec_dir.into_os_string();
                    new_path.push(":");
                    new_path.push(path);
                    new_path
                }
                None => exec_dir.into_os_string(),
            };

            let mut cmd = std::process::Command::new(&program);
            cmd.args(&arguments)
                .envs(std::env::vars_os())
                .env("PATH", path);

            if let Some(db_opt) = &opt.db {
                dbg!(&db_opt);
                cmd.env("IMPOSTER_PASS_DB", db_opt.to_string());
            } else {
                cmd.env("IMPOSTER_PASS_DB", db.to_string());
            }

            cmd.env("IMPOSTER_PASS_QUIET", "true").spawn()?.wait()?;
        }
    }

    if let Some(db_path) = db_path.map(std::path::PathBuf::from) {
        if let Some(basename) = db_path.parent() {
            std::fs::create_dir_all(basename)?;
        }

        serde_json::to_writer_pretty(std::fs::File::create(db_path)?, &db)?;
    } else if !opt.quiet {
        serde_json::to_writer_pretty(std::io::stdout(), &db)?;
    }

    Ok(())
}
