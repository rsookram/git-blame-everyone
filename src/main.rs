use anyhow::{bail, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    process::Command,
};

fn main() -> Result<()> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();

    let result = run(&args)?;

    let mut result = result.into_iter().collect::<Vec<_>>();

    // Order by lines of code descending
    result.sort_by(|a, b| b.1.cmp(&a.1));

    for (name, count) in result {
        println!("{}\t{}", count, name);
    }

    Ok(())
}

fn run(args: &[OsString]) -> Result<BTreeMap<String, usize>> {
    args.par_iter()
        .map(|arg| count_author_lines(arg))
        .reduce(|| Ok(BTreeMap::new()), {
            |a, b| match (a, b) {
                (Ok(mut a), Ok(b)) => {
                    for (name, count) in b {
                        *a.entry(name).or_insert(0) += count;
                    }

                    Ok(a)
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        })
}

fn count_author_lines(path: &OsString) -> Result<BTreeMap<String, usize>> {
    let output = Command::new("git")
        .args(&[OsStr::new("blame"), path, OsStr::new("--line-porcelain")])
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let porcelain = std::str::from_utf8(&output.stdout)?;

    let mut counter = BTreeMap::new();

    porcelain
        .lines()
        .filter(|line| line.starts_with("author "))
        .map(|line| {
            let (_, name) = line.split_once(' ').expect("line starts with 'author '");

            name.to_string()
        })
        .for_each(|name| {
            *counter.entry(name).or_insert(0) += 1;
        });

    Ok(counter)
}
