mod cli_app;
mod logging;

use anyhow::Result;

pub fn run() -> Result<()> {
    cli_app::run(|cli| {
        logging::init_log(cli.verbose, &cli.log);
    })
}
