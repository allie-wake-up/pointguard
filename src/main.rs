use clap::Clap;

fn main() {
    let opts: pg::Opts = pg::Opts::parse();
    let settings = pg::Settings::new().unwrap();
    pg::run(opts, settings);
}
