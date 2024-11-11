use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPass {
    #[arg(short, long, default_value_t = 12)]
    pub length: u8,
}
