use git2::Repository;
use std::{collections::BTreeMap, error::Error, ffi::OsString, path::Path};

fn main() {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();

    let result = run(&args);

    for (name, count) in result.unwrap().iter() {
        println!("{} {}", name, count);
    }
}

fn run(args: &[OsString]) -> Result<BTreeMap<String, usize>, Box<dyn Error>> {
    let repo = Repository::open(".")?;

    let mut counter = BTreeMap::new();

    for arg in args {
        // TODO: Make this faster. It's too slow on files with a lot of history.
        let blame = repo.blame_file(Path::new(arg), None)?;

        for hunk in blame.iter() {
            let count = hunk.lines_in_hunk();

            let signature = hunk.final_signature();
            let name = signature.name().unwrap_or("unknown");
            *counter.entry(name.to_string()).or_insert(0) += count;
        }
    }

    Ok(counter)
}
