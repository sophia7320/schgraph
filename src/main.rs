use color_eyre::eyre::{Ok, Result};
use schgraph::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let app = App::new();

    app.run()?;

    Ok(())
}
