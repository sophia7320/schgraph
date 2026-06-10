use color_eyre::eyre::{Ok, Result};

use crate::base::graph::Graph;

#[derive(Debug, Default)]
pub struct App {
    should_exit: bool,

    gra: Graph,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn run(self) -> Result<()> {
        println!("hello schgraph");
        println!("should_exit is {:?}", self.should_exit);
        Ok(())
    }
}
