use runner::Cli;

fn main() {
    let result = Cli::run();
    match result {
        Ok(_) => (),
        _ => (),
    }
}
