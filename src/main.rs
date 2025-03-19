use std::io::Result;
use std::path::Path;
use std::time::{Instant, SystemTime};

fn main() -> Result<()> {
    let n = 100;
    let reps = 10;
    let mut uniques = Vec::new();
    for _ in 0..reps {
        uniques.push(run_experiment(n)?);
    }
    println!("{:?}", uniques);
    Ok(())
}

fn run_experiment(n: u32) -> Result<u32> {
    let start = Instant::now();
    let tmp = tempfile::tempdir().unwrap();
    let times = unique_times(n, tmp.path())?;
    let finish = Instant::now();
    println!("{} / {}", times.len(), n);
    println!("{times:?}");
    println!("Experiment took {:?}.", finish - start);
    Ok(times.len().try_into().expect("it's out of n, which is u32"))
}

fn unique_times(n: u32, root: &Path) -> Result<Vec<SystemTime>> {
    let names: Vec<_> = (0..n).map(|i| format!("{i:03}")).collect();
    for name in &names {
        std::fs::write(root.join(name), name)?;
    }
    let mut times: Vec<_> = names
        .iter()
        .map(|name| {
            root.join(&name)
                .metadata()
                .expect("file exists and has metadata")
                .modified()
                .expect("file metadata has mtime")
        })
        .collect();
    times.sort();
    times.dedup();
    Ok(times)
}
