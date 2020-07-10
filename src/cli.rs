use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Cmd {
    /// Lists all the entries in the password store
    ///
    /// This command tries to have the same behaviour as pass ls
    #[structopt(name = "ls")]
    List {
        /// The password or password folder you want to list. If this option is not set, then the
        /// whole password store will be listed
        #[structopt(name = "pass-name", parse(from_os_str))]
        name: Option<PathBuf>,
    },

    /// Displays the value of a password.
    ///
    /// This command tries to have the same behaviour as pass show <PASS_NAME>
    Show {
        /// The name of the password who's value you want to get.
        ///
        /// If this is not set, then this has the same output as imposter-pass ls
        #[structopt(name = "pass-name", help = "The password to show", parse(from_os_str))]
        name: Option<PathBuf>,
    },

    /// Inserts a new password value into the database.
    ///
    /// This command tries to have the same behaviour as pass insert <PASS_NAME>
    Insert {
        /// The path of the password you want to insert
        #[structopt(name = "pass-name", parse(from_os_str))]
        name: PathBuf,

        /// This has the same behaviour as pass insert --echo <PASS_NAME>
        #[structopt(name = "echo", long, short)]
        echo: bool,

        /// This has the same behaviour as pass insert --multiline <PASS_NAME>
        #[structopt(name = "multiline", long, short, conflicts_with = "echo")]
        multiline: bool,

        /// This has the same behaviour as pass insert --force <PASS_NAME>
        #[structopt(name = "force", long, short)]
        force: bool,
    },
    /// Removes an existing password value into the database.
    ///
    /// This command tries to have the same behaviour as pass insert <PASS_NAME>
    #[structopt(name = "rm")]
    Remove {
        /// The path to the password or password folder you want to remove
        #[structopt(name = "pass-name", parse(from_os_str))]
        name: PathBuf,

        /// If you specified a password folder to remove, then you must set this flag.
        ///
        /// This has the same effect as pass rm --recursive <PASS_NAME>
        #[structopt(name = "recursive", long, short)]
        recursive: bool,

        /// Disable interactive prompts for removal
        ///
        /// This has the same effect as pass rm --force <PASS_NAME>
        #[structopt(name = "force", long, short)]
        force: bool,
    },
    /// Executes the given program as though imposter-pass was the real pass
    Fool {
        /// The program you want to fool
        ///
        /// This program will be executed with a new entry added to its path. This new entry will
        /// contain an exectuable pass, which is a wrapper around this imposter-pass exectuable
        /// with some default settings applied
        #[structopt(name = "program", parse(from_os_str))]
        program: std::ffi::OsString,

        /// Additional arguments you want to pass to the specified program
        #[structopt(name = "arguments", parse(from_os_str))]
        arguments: Vec<std::ffi::OsString>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "imposter-pass")]
pub struct Cli {
    /// The password store to use.
    ///
    /// This may be a json object mapping password paths to values, or it may be a file path to use for the password store
    #[structopt(long, env = "IMPOSTER_PASS_STORE")]
    pub store: Option<String>,

    /// Indicates that imposter pass should avoid showing information in addition to what the real
    /// pass would have output
    ///
    /// For example, setting this to true will mean that if the store is an environment variable,
    /// imposter-pass will exit silently without showing you the changes made to the database.
    ///
    /// In general, you probably don't want to set this unless you know what you're doing. It
    /// mostly exists to make the "fool" command work.
    #[structopt(short, long, env = "IMPOSTER_PASS_QUIET")]
    pub quiet: bool,

    #[structopt(subcommand)]
    pub cmd: Cmd,
}
