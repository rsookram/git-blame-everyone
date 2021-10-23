use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::BTreeMap,
    error::Error,
    ffi::{OsStr, OsString},
    process::Command,
};

fn main() {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();

    let result = run(&args);

    let mut result = result.into_iter().collect::<Vec<_>>();

    // Order by lines of code descending
    result.sort_by(|a, b| b.1.cmp(&a.1));

    for (name, count) in result {
        println!("{}\t{}", count, name);
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
        .args(&[OsStr::new("blame"), path, OsStr::new("--line-porcelain")])
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
