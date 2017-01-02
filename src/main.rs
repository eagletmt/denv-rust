extern crate denv;
extern crate exec;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // TODO: Add `-f` flag
    let path = ".env";
    denv::load(path).expect("Unable to load .env file");
    let err = exec::execvp(args[1].clone(), args[1..args.len()].into_iter());
    panic!("{}", err);
}
