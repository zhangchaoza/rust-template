mod cli_app;
mod logging;

use anyhow::Result;

pub fn main() -> Result<()> {
    cli_app::run(|cli| {
        logging::init_log(cli.verbose, &cli.log);
    })
}
