use std::env;
use clap::Parser;

pub trait Build {
    fn build() -> Self;
}

pub trait Select: Parser + Build {
    fn select() -> Self {
        if env::var("CLAP_BUILDER").is_ok() {
            Self::build()
        } else {
            Self::parse()
        }
    }
}