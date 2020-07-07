use clap::Clap;

fn main() {
    let opts: pg::Opts = pg::Opts::parse();
    let settings = pg::Settings::new().unwrap();
    let mut stdout = std::io::stdout();
    match pg::run(&mut stdout, opts, settings) {
        Ok(_) => (),
        Err(e) => match e {
            pg::PointGuardError::GpgError(status, message) => {
                eprintln!("GPG Error: {}", message);
                std::process::exit(status);
            }
            _ => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
    }
}
