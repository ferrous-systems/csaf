#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct ValidationArgs {}

pub fn validate(_args: ValidationArgs) {
    // ...
}
