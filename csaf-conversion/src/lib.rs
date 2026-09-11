#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct ConversionArgs {}

pub fn convert(_args: ConversionArgs) {
    let _csaf2_0 = csaf_schema::csaf2_0::Csaf2_0 {};

    // ...
}
