use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::BTreeMap, error::Error, ffi::OsString, process::Command};

fn main() {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();

    let result = run(&args);

    for (name, count) in result.iter() {
        println!("{} {}", name, count);
    }
}

fn run(args: &[OsString]) -> BTreeMap<String, usize> {
    args.par_iter()
        .map(|arg| {
            let porcelain = blame_porcelain(arg).unwrap();

            count(porcelain)
        })
        .reduce(|| BTreeMap::new(), {
            |mut a, b| {
                for (name, count) in b {
                    *a.entry(name).or_insert(0) += count;
                }

                a
            }
        })
}

fn blame_porcelain(path: &OsString) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["blame", &path.to_string_lossy(), "--line-porcelain"])
        .output()?;

    Ok(std::str::from_utf8(&output.stdout).map(|s| s.to_string())?)
}

fn count(porcelain: String) -> BTreeMap<String, usize> {
    let mut counter = BTreeMap::new();

    porcelain
        .lines()
        .filter(|line| line.starts_with("author "))
        .map(|line| {
            let (_, name) = line.split_once(' ').unwrap();

            name.to_string()
        })
        .for_each(|name| {
            *counter.entry(name).or_insert(0) += 1;
        });

    counter
}
