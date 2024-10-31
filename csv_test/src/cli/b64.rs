use std::str::FromStr;

use anyhow::Ok;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64OpCmd {
    #[command(name = "encode", about = "base64 encode")]
    Encode(BaseCmd),
    #[command(name = "decode", about = "base64 decode")]
    Decode(BaseCmd),
}

#[derive(Debug, Parser)]
pub struct BaseCmd {
    #[arg(long, short, default_value = "-")]
    pub input: String,
}

#[derive(Debug, Clone, Parser)]
pub enum OpModeEnum {
    BASE,
    URLSAFA,
}

impl FromStr for OpModeEnum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "base" => Ok(OpModeEnum::BASE),
            "urlsafe" => Ok(OpModeEnum::URLSAFA),
            _ => anyhow::bail!("not match base64 op type"),
        }
    }
}
