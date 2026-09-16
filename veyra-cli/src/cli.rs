pub enum Command {
    Help,
    Version,
    Install(String),
    Remove(String),
    Search(String),
    Update,
    Info,
    Doctor,
}

pub fn parse(args: &[String]) -> Result<Command, String> {
    match args.get(1).map(String::as_str) {
        None | Some("help") | Some("--help") | Some("-h") => Ok(Command::Help),

        Some("version") | Some("--version") | Some("-V") => Ok(Command::Version),

        Some("install") => package_command(args, "install").map(Command::Install),

        Some("remove") => package_command(args, "remove").map(Command::Remove),

        Some("search") => package_command(args, "search").map(Command::Search),

        Some("update") => Ok(Command::Update),

        Some("info") => Ok(Command::Info),

        Some("doctor") => Ok(Command::Doctor),

        Some(command) => Err(format!("unknown command '{}'", command)),
    }
}

fn package_command(args: &[String], command: &str) -> Result<String, String> {
    match args.get(2) {
        Some(package) => Ok(package.clone()),

        None => Err(format!(
            "missing package name.\nUsage: veyra {} <package>",
            command
        )),
    }
}
