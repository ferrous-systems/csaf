pub use csaf_schema as schema;
pub use csaf_validation as validation;

#[cfg(feature = "conversion")]
pub use csaf_conversion as conversion;

#[derive(Debug, Clone, clap::Parser)]
pub enum Cmd {
    Validate(validation::ValidationArgs),
    #[cfg(feature = "conversion")]
    Convert(conversion::ConversionArgs),
}

pub fn run(cmd: Cmd) {
    match cmd {
        Cmd::Validate(validation_args) => validation::validate(validation_args),
        #[cfg(feature = "conversion")]
        Cmd::Convert(conversion_args) => conversion::convert(conversion_args),
    }
}
