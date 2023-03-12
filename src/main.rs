// #![allow(unused)] // For beginning only.

use std::env;

use utils::argument_parser::parse_args;

use crate::{
    prelude::*,
    utils::{
        template::run_template,
        templates::{ActixTemplate, JCTemplate},
    },
};

mod error;
mod prelude;
mod utils;

#[derive(Debug)]
pub enum TemplateType {
    JC,
    Actix,
}

impl TemplateType {
    fn from_str(str: &str) -> Result<Self> {
        match str {
            "jc" => Ok(Self::JC),
            "actix" => Ok(Self::Actix),
            _ => Err(Error::BadTemplate(str.to_owned())),
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let (name, template) = match parse_args(args) {
        Ok(value) => value,
        Err(_) => {
            return Ok(());
        }
    };

    println!(
        "Creating project named \"{}\" of type \"{:?}\".",
        name, template
    );

    match template {
        TemplateType::JC => {
            run_template(&name, &JCTemplate {})?;
        }
        TemplateType::Actix => {
            run_template(&name, &ActixTemplate {})?;
        }
    };

    Ok(())
}
