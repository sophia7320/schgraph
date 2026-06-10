use color_eyre::eyre::{Ok, Result};
use schgraph::App;

fn main() -> Result<()> {
    let app = App::new();

    app.run()?;

    Ok(())
}
