mod cli;

fn main() {
    let args = cli::parse_args();
    log::info!("{:?}", args);
}
