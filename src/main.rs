fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.optopt("f", "filename", "Path to .env file", "FILENAME");
    opts.optflag("h", "help", "Print help");
    let matches = opts.parse(&args[1..]).expect("Unable to parse options");
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.free.is_empty() {
        print_usage(&program, opts);
        std::process::exit(1);
    }
    let path = matches.opt_str("f").unwrap_or(".env".to_owned());

    match denv::load(path) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
    let err = exec::execvp(matches.free[0].clone(), matches.free);
    panic!("{}", err);
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [OPTIONS] COMMAND...", program);
    print!("{}", opts.usage(&brief));
}
