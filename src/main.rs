mod cli;
mod error;
mod store;

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

    let (store, store_path): (store::Store, Option<std::path::PathBuf>) =
        if let Some(store_opt) = &opt.store {
            let store_opt: serde_json::Value = serde_json::from_str(store_opt)
                .unwrap_or_else(|_| serde_json::Value::String(store_opt.to_string()));

            if let Some(store_path) = store_opt.as_str().map(std::path::Path::new) {
                (
                    if store_path.exists() {
                        serde_json::from_reader(std::fs::File::open(store_path)?)?
                    } else {
                        store::Store::default()
                    },
                    Some(store_path.to_path_buf()),
                )
            } else {
                (serde_json::from_value(store_opt)?, None)
            }
        } else {
            (store::Store::default(), None)
        };

    let save_store = |new_store: &store::Store| -> Result<()> {
        if !opt.quiet && store != *new_store {
            eprintln!(
                "
Previous database : {prev_store}
New database      : {next_store}",
                prev_store = store.to_string(),
                next_store = new_store.to_string()
            )
        }

        if let Some(store_path) = &store_path {
            if let Some(basename) = store_path.parent() {
                std::fs::create_dir_all(basename)?;
            }

            serde_json::to_writer_pretty(std::fs::File::create(store_path)?, &new_store)?;
        }

        Ok(())
    };

    if !opt.quiet {
        eprintln!("BEWARE! THIS IS NOT THE REAL PASS (https://www.passwordstore.org/), BUT ONLY A CLEVER IMPOSTER!");
        eprintln!("If you did not expect to see this message, STOP doing whatever you're doing! You've been bamboozeled!");
        if let Some(store_path) = store_path.as_ref() {
            eprintln!("Warning: database will be saved and loaded from {:?}. THE DATABASE IS NOT ENCRYPTED!", store_path);
        } else if opt.store.is_some() {
            eprintln!("Warning: database will be loaded from the environment variable IMPOSTER_PASS_STORE. THE DATABASE IS NOT ENCRYPTED and changes WILL NOT BE SAVED!");
        } else {
            eprintln!("Warning: no database specified. Defaulting to empty database. Changes WILL NOT BE SAVED!");
        }
    }

    match opt.cmd {
        cli::Cmd::List { ref name } => {
            store.show(name.as_ref())?;
        }
        cli::Cmd::Show { ref name } => {
            store.show(name.as_ref())?;
        }
        cli::Cmd::Insert {
            ref name,
            echo,
            multiline,
            force,
        } => {
            use std::collections::btree_map::Entry;
            use std::io::Write;

            let mut store = store.clone();

            let entry = store.entry(&name)?;

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

            *entry.or_default() = password;

            save_store(&store)?;
        }
        cli::Cmd::Remove {
            recursive,
            force,
            ref name,
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

            let mut store = store.clone();

            let count = store.list(Some(name.as_path()))?.count();
            match count {
                0 => return Err(Error::NotInStore(name.clone())),
                1 => {
                    if force || delete_check(name.as_path())? {
                        store.remove(&name)?;
                    }
                }
                _ => {
                    if recursive {
                        if force || delete_check(name.as_path())? {
                            store.remove(&name)?;
                        }
                    } else {
                        return Err(Error::IsADirectory(name.clone()));
                    }
                }
            }
        }
        cli::Cmd::Fool {
            ref program,
            ref arguments,
        } => {
            use std::path::*;
            let exec = std::env::current_exe()?;
            let exec_dir = exec.parent().map(Path::to_path_buf).unwrap_or_default();

            let tmp_dir = std::env::temp_dir().join("imposter-pass").join(
                {
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::*;
                    use std::io::prelude::*;

                    let mut data = Vec::new();
                    std::fs::File::open(&exec)?.read_to_end(&mut data)?;

                    let mut hasher = DefaultHasher::new();
                    data.hash(&mut hasher);

                    hasher.finish()
                }
                .to_string(),
            );

            std::fs::create_dir_all(&tmp_dir)?;

            let wrapped_exec = tmp_dir.join("pass");

            // We will put the current configuration in this file.
            // We use the hash so that we don't have to create a new file everytime,
            // But we won't overwrite anything if multiple instances are running at once
            let store_tmp_path = tmp_dir.join(
                {
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::*;

                    let mut hasher = DefaultHasher::new();
                    std::time::Instant::now().hash(&mut hasher);

                    hasher.finish()
                }
                .to_string(),
            );

            if !opt.quiet {
                eprintln!(
                    "Calling {program:?} with wrapped pass={exec:?} and temporary store={store:?}.
Note: if you enter Ctrl-C before this program exits, or something else goes wrong,
look in the temporary password store at {store:?}. It will contain all the changes to the store
up until the execution was interupted",
                    program = program,
                    exec = wrapped_exec.display(),
                    store = store_tmp_path.display()
                );
            }

            // Write a wrapper script to the tmp dir. This will be used to pass settings to the
            // fool exec.
            #[cfg(target_family = "unix")]
            {
                use std::fs::Permissions;
                use std::os::unix::fs::*;

                std::fs::write(
                    &wrapped_exec,
                    format!(
                        "#!/usr/bin/env sh
                        export IMPOSTER_PASS_QUIET=true
                        export IMPOSTER_PASS_STORE={store}
                        {exec} $@",
                        exec = exec.display(),
                        store = store_tmp_path.display()
                    ),
                )?;

                // Mark the file as executable
                std::fs::set_permissions(&wrapped_exec, Permissions::from_mode(0o775))?;
            }

            #[cfg(target_family = "windows")]
            {
                todo!("the fool command is not supported on windows at this time.
                        You can still use this application by manually setting environment variables.")
            }

            serde_json::to_writer(std::fs::File::create(&store_tmp_path)?, &store)?;

            let path = match std::env::var_os("PATH") {
                Some(path) => {
                    let mut new_path = tmp_dir.into_os_string();
                    new_path.push(":");
                    new_path.push(path);
                    new_path
                }
                None => exec_dir.into_os_string(),
            };

            let mut cmd = std::process::Command::new(&program);
            cmd.args(arguments)
                .envs(std::env::vars_os())
                .env("PATH", path)
                .spawn()?
                .wait()?;

            let new_store = serde_json::from_reader(std::fs::File::open(&store_tmp_path)?)?;
            save_store(&new_store)?;
        }
    }

    Ok(())
}
