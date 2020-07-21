use clap::Clap;

fn main() {
    let opts: pointguard::Opts = pointguard::Opts::parse();
    let settings = pointguard::Settings::new().unwrap();
    let mut stdout = std::io::stdout();
    match pointguard::run(&mut stdout, opts, settings) {
        Ok(_) => (),
        Err(e) => match e {
            pointguard::PointGuardError::GpgError(status, message) => {
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
