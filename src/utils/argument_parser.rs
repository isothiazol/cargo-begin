use crate::prelude::*;
use crate::TemplateType;

pub fn parse_args(mut args: Vec<String>) -> Result<(String, TemplateType)> {
    let template_aliases = ["--template", "--t", "-template", "-t"];

    // Remove the path
    args.remove(0);

    // If called using "cargo begin", remove "begin"
    match args.get(0) {
        Some(value) => {
            if value == "begin" {
                args.remove(0);
            }
        }
        None => {}
    }

    // Project name or nothing
    let name = match args.get(0) {
        Some(value) => {
            if !template_aliases.contains(&&value.as_str()) {
                value
            } else {
                ""
            }
        }
        None => "",
    };

    let mut template = "jc".to_string();

    // Bad solution
    let mut counter = 0;
    for arg in args.iter() {
        // Argument must be template parameter
        if template_aliases.contains(&arg.as_str()) {
            match args.get((counter + 1) as usize) {
                Some(value) => {
                    template = value.to_string();
                    break;
                }
                None => {}
            }
        }

        counter += 1;
    }

    let template = template.to_lowercase();

    // let template = match  {
    let template = match TemplateType::from_str(&template) {
        Ok(value) => value,
        Err(err) => {
            println!("\"{}\"     isn't an avalible template.", &template);
            return Err(err);
        }
    };

    return Ok((name.to_owned(), template));
}
