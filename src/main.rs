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
        .map(|arg| count_author_lines(arg).unwrap())
        .reduce(|| BTreeMap::new(), {
            |mut a, b| {
                for (name, count) in b {
                    *a.entry(name).or_insert(0) += count;
                }

                a
            }
        })
}

fn count_author_lines(path: &OsString) -> Result<BTreeMap<String, usize>, Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["blame", &path.to_string_lossy(), "--line-porcelain"])
        .output()?;

    let porcelain = std::str::from_utf8(&output.stdout)?;

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

    Ok(counter)
}
