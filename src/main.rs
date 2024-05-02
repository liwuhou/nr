use nr::Cli;

fn main() {
    let result = Cli::run();
    match result {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
