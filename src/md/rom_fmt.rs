use std::{str::FromStr, fmt::Display};

#[derive(Debug)]
#[repr(u8)]
pub enum Format {
    Bin,
    Smd,
    Md,
}

impl FromStr for Format {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bin" => Ok(Self::Bin),
            "smd" => Ok(Self::Smd),
            "md" => Ok(Self::Md),
            _ => Err("invalid or unknown format"),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Bin => "bin",
            Self::Smd => "smd",
            Self::Md  => "md",
        })
    }
}
