mod cli_app;
mod logging;
mod versions;

fn main() {
    cli_app::run(|cli| {
        logging::init_log(cli.verbose, &cli.log);
    });
}
