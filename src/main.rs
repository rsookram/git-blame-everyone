use std::{collections::BTreeMap, error::Error, ffi::OsString, process::Command};

fn main() {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();

    let result = run(&args);

    for (name, count) in result.unwrap().iter() {
        println!("{} {}", name, count);
    }
}

fn run(args: &[OsString]) -> Result<BTreeMap<String, usize>, Box<dyn Error>> {
    let mut counter = BTreeMap::new();

    for arg in args {
        let output = Command::new("git")
            .args(&["blame", &arg.to_string_lossy(), "--line-porcelain"])
            .output()?;

        let porcelain = std::str::from_utf8(&output.stdout)?;

        for line in porcelain.lines() {
            let parts = line.split_once(' ');

            let (field, value) = match parts {
                Some(f) => f,
                None => continue,
            };

            if field != "author" {
                continue;
            }

            *counter.entry(value.to_string()).or_insert(0) += 1;
        }
    }

    Ok(counter)
}
