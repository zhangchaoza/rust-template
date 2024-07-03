mod cli_app;
mod logging;
mod versions;

use anyhow::Result;

pub fn run() -> Result<()> {
    cli_app::run(|cli| {
        logging::init_log(cli.verbose, &cli.log);
    })
}
