mod calculator;
mod cli;

// Public API

pub struct RpnCalc {
    stack: Vec<f64>,
}

impl RpnCalc {
    pub fn new() -> RpnCalc {
        RpnCalc { stack: vec![] }
    }

    pub fn cli() -> cli::Cli {
        cli::Cli::new()
    }

    pub fn process(&mut self, cmd: cli::CliCmd) {
        calculator::process(self, cmd)
    }
}

impl Default for RpnCalc {
    fn default() -> Self {
        Self::new()
    }
}
