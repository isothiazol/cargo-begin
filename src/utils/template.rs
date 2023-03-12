use crate::prelude::*;
use std::{
    env,
    fs::{self, File},
    io::Write,
    process::Command,
    thread,
    time::Duration,
};
pub trait Template {
    /// Crate name@version, Features
    fn crates(&self) -> Vec<(&'static str, &'static str)>;
    /// File name/path, Content
    fn files(&self) -> Vec<(&'static str, &'static str)>;
}

pub fn run_template(name: &String, template: &dyn Template) -> Result<()> {
    // Init in a quiet way
    Command::new("cargo")
        .arg("init")
        .arg(name)
        .arg("-q")
        .spawn()?;

    // Give cargo some time
    thread::sleep(Duration::from_millis(100));

    env::set_current_dir(format!("./{}", name))?;

    println!("Adding crates.");
    for add in template.crates() {
        let features: Vec<&str> = add.1.split(' ').collect();

        if features.len() == 0 {
            Command::new("cargo")
                .arg("add")
                .arg(add.0)
                .arg("-q")
                .spawn()?;
        } else {
            Command::new("cargo")
                .arg("add")
                .arg(add.0)
                .arg("--features")
                .args(features)
                .arg("-q")
                .spawn()?;
        }
    }

    println!("Creating files.");
    for file in template.files() {
        let file_name = file.0;
        let file_content = file.1;

        // Create all the paths if necessary
        let file_path = format!("./src/{}", file_name);
        match file_path.rsplit_once("/") {
            Some(value) => {
                fs::create_dir_all(value.0)?;
            }
            None => {}
        };

        // Remove the newline character
        let mut file_content = file_content.to_string();
        if file_content.len() > 0 {
            file_content.remove(0);
        }

        let mut file = File::create(file_path)?;
        file.write_all(file_content.as_bytes())?;
    }

    println!("All done!");
    Ok(())
}
