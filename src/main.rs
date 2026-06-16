use std::env;

use color_eyre::eyre::Result;
use schgraph::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<_> = env::args().collect();

    let app = App::from_file(&args[2])?;

    app.run()?;

    Ok(())
}
