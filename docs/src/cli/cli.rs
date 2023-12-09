//use futures::executor;
use getopts::{Matches, Options};
use std::{
    env,
    fs::File,
    io::{self, BufRead, Error},
    process::exit,
};

use crate::upsurge::{
    Config::Config,
    Config::{Deployment, Method, Output, Recap},
    ConnexionInfo::ConnexionInfo,
    UpSurge,
    UpSurge::SSHUpSurge,
};

/*
    Only for -c option (clients on one line, separated by a comma)
    Return value: vector of Client objects
*/
fn parse_connexions(connexions_str: &str) -> Vec<ConnexionInfo> {
    let mut res: Vec<ConnexionInfo> = Vec::new();

    let connexions = connexions_str.split(",");
    for connexion in connexions {
        // Create a new Client object
        if let Ok(connexion_info) = ConnexionInfo::from_formatted(connexion.to_string()) {
            res.push(connexion_info);
        } else {
            eprintln!(
                "Failed to create Client object from connexion: {}",
                connexion
            );
        }
    }
    return res;
}

/*
    Only for -C option (clients in a file)
    Return value: vector of Client objects
*/
fn parse_connexion_file(filename: &str) -> Vec<ConnexionInfo> {
    let mut res: Vec<ConnexionInfo> = Vec::new();

    // Open file and check error
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return res;
        }
    };
    // Read file line per line
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line_str = &line;
        if let Ok(connexion) = line_str {
            // Create a new Client object
            if let Ok(connexion_info) = ConnexionInfo::from_formatted(connexion.to_string()) {
                res.push(connexion_info.clone());
            } else {
                println!(
                    "Failed to create Client object from connexion: {:?}",
                    connexion
                );
            }
        }
    }
    return res;
}

/*
    Only for -K option (if one command, no need to parse)
    Return value: vector of commands
*/
fn parse_commands_file(filename: &str) -> Result<Vec<String>, Error> {
    let mut res: Vec<String> = Vec::new();
    // Open file and check error
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    // Read file line per line
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(command) = line {
            res.push(command);
        }
    }
    Ok(res)
}

/*
    Only for -H option
    Print help message
*/
fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

// -h, -v, -d, --Khelp, --Chelp execute and exit instantly
fn check_exiting_options(matches: &Matches, program: &str, opts: &Options) {
    // let mut matched: bool = false;
    if matches.opt_present("h") {
        print_usage(&program, opts);
        // matched = true;
        exit(1);
    }
    if matches.opt_present("v") {
        let version = env!("CARGO_PKG_VERSION");
        println!("UpSurge version {}", version);
        // matched = true;
        exit(1);
    }
    if matches.opt_present("d") {
        let config = Config::default();
        println!("{:#?}", config);
        //matched = true;
        exit(1);
    }
    if matches.opt_present("Khelp") {
        todo!();
        //matched = true;
        exit(1);
    }
    if matches.opt_present("Chelp") {
        todo!();
        //matched = true;
        exit(1);
    }
}

// Compare matches with options and flags and return config to run
fn parse_config<'a>(
    matches: &'a Matches,
    program: &'a str,
) -> Result<(Config, Vec<ConnexionInfo>, Vec<String>), Error> {
    // Values to return
    let mut o_config = Config::default();
    let mut o_connexions: Vec<ConnexionInfo> = Vec::new();
    let mut o_commands: Vec<String> = Vec::new();

    // Parse the config options
    if let Some(output) = matches.opt_str("o") {
        o_config.output = Output::OUTPUT(output);
    }
    if let Some(recap_path) = matches.opt_str("r") {
        o_config.recap = Recap::RECAP(recap_path);
    }

    if matches.opt_present("D") {
        o_config.deployment = Deployment::AUTODEPLOY;
    }

    if matches.opt_present("T") {
        o_config.method = Method::THREADED;
    }

    // Check compatibilities #3

    // Parse connexions and commands
    if let Some(connexions_str) = matches.opt_str("c") {
        o_connexions = parse_connexions(&connexions_str);
    }
    if let Some(connexions_file) = matches.opt_str("C") {
        o_connexions.append(&mut parse_connexion_file(&connexions_file));
    }
    if let Some(commands_str) = matches.opt_str("k") {
        o_commands.append(&mut vec![commands_str]);
    }
    if let Some(commands_file) = matches.opt_str("K") {
        let r_commands = parse_commands_file(&commands_file);
        match r_commands {
            Ok(mut r_commands) => o_commands.append(&mut r_commands),
            Err(e) => {
                println!("Failed to parse commands file: {}", e);
                return Err(e);
            }
        };
    }

    // Check compatibilities #3

    Ok((o_config, o_connexions, o_commands))
}

pub async fn run() {
    // Get arguments from command line
    let args: Vec<String> = env::args().collect();

    /*Args form: [program, ...]
    Example: cargo run -- -d -e
            ->["target/debug/UpSurge", "-d", "-e"]
    */
    let program = args[0].clone();

    //Vector options
    let mut opts = Options::new();

    /*  Defining options vector:
    Options form: [short, long, description, argument]
    Desc is used for explaining the option
    Hint is used for explaining the argument
    */
    opts.optopt("o", "output", "Set output file name", "FILENAME");

    //List of possible options
    opts.optopt(
        "c",
        "connexion",
        "Remote hosts connexion information, comma separated",
        "USER@IP:PORT",
    );
    opts.optopt(
        "C",
        "connexions",
        "Remote hosts connexion information file",
        "FILE",
    );
    opts.optopt(
        "k",
        "command",
        "Command to execute on remote machine",
        "COMMAND",
    );
    opts.optopt(
        "K",
        "commands",
        "File containing commands to execute on remote machine",
        "FILE",
    );
    opts.optopt(
        "r",
        "recap",
        "Generates recap of remote executions & performances",
        "FILENAME",
    );

    // Defining flag vector, same as options but without argument
    opts.optflag("h", "help", "Print help menu");
    opts.optflag("", "Khelp", "Print commands required file formatting");
    opts.optflag("", "Chelp", "Print connexions required file formatting");
    opts.optflag("v", "version", "Print current version");
    opts.optflag("d", "default", "Print default UpSurge configuration");
    opts.optflag("D", "deploy", "Activate autodeployment mode");
    opts.optflag("T", "threaded", "Set execution to threaded");

    /* Parse the arguments, if no panic, store them in matches
        https://docs.rs/getopts/latest/getopts/struct.Options.html#method.parse
    */
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            todo!();
        }
    };

    // -h, -v, -d, --Khelp, --Chelp execute and exit instantly (no config parsing)
    check_exiting_options(&matches, &program, &opts);

    // Compare matches with options and flags and return config to run
    let parsed = parse_config(&matches, &program);

    let (config, connexions, commands);
    match parsed {
        Ok((cf, con, com)) => {
            config = cf;
            connexions = con;
            commands = com;
        }
        Err(e) => {
            println!("Failed to parse config: {}", e);
            exit(1);
        }
    }
    println!("Creating UpSurge session\n");

    let app = UpSurge::SSHUpSurge::new(config);
    println!("Starting UpSurge");
    println!("Connexions: {:?}", connexions);
    println!("Commands: {:?}", commands);

    println!("Starting process\n");
    let result = app.process_all(&connexions, &commands, print_result).await;
}

fn print_result(res: String) {
    println!("{:?}\n", res);
}
