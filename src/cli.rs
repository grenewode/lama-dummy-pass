use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "ls")]
    List {
        #[structopt(name = "pass-name", parse(from_os_str))]
        name: Option<PathBuf>,
    },
    Show {
        #[structopt(name = "pass-name", parse(from_os_str))]
        name: Option<PathBuf>,
    },
    Insert {
        #[structopt(name = "pass-name", parse(from_os_str))]
        name: PathBuf,

        #[structopt(name = "echo", long, short)]
        echo: bool,

        #[structopt(name = "multiline", long, short, conflicts_with = "echo")]
        multiline: bool,

        #[structopt(name = "force", long, short)]
        force: bool,
    },
    #[structopt(name = "rm")]
    Remove {
        #[structopt(name = "pass-name", parse(from_os_str))]
        name: PathBuf,
        #[structopt(name = "recursive", long, short)]
        recursive: bool,
        #[structopt(name = "force", long, short)]
        force: bool,
    },
    DumpDb {
        #[structopt(name = "pretty", long, short)]
        pretty: bool,
    },
    Fool {
        #[structopt(name = "program", parse(from_os_str))]
        program: std::ffi::OsString,
        #[structopt(name = "arguments", parse(from_os_str))]
        arguments: Vec<std::ffi::OsString>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "imposter-pass")]
pub struct Cli {
    #[structopt(long, env = "IMPOSTER_PASS_DB")]
    pub db: Option<String>,

    #[structopt(short, long, env = "IMPOSTER_PASS_QUIET")]
    pub quiet: bool,

    #[structopt(subcommand)]
    pub cmd: Cmd,
}
