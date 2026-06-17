use std::env;

use color_eyre::eyre::{Result, eyre};
use schgraph::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut args = env::args().skip(1);

    let mut file_path = None;

    while let Some(arg) = args.next() {
        if arg == "-f" {
            file_path = args.next()
        }
    }
    let file_path = file_path.ok_or_else(|| eyre!("用法: schgraph -f <数据文件>"))?;

    let app = App::from_file(file_path.as_str())?;

    app.run()?;

    Ok(())
}
