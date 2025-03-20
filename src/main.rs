use std::io::Result;
use std::path::Path;
use std::time::{Instant, SystemTime};

fn main() -> Result<()> {
    let reps = 50;
    let n = 100;

    let mut counts = Vec::new();
    for _ in 0..reps {
        counts.push(run_timed_experiment(n)?);
    }
    println!("{:?}", counts);
    println!(
        "has 1? {},  has {}? {}",
        counts.contains(&1),
        n,
        counts.contains(&n)
    );
    Ok(())
}

fn run_timed_experiment(n: u32) -> Result<u32> {
    let start = Instant::now();
    let count = run_experiment(n)?;
    let finish = Instant::now();
    println!("Experiment took {:?}.", finish - start);
    Ok(count)
}

fn run_experiment(n: u32) -> Result<u32> {
    let tmp = tempfile::tempdir().expect("create temporary directory");
    let times = distinct_times(n, tmp.path())?;
    println!("{} / {}", times.len(), n);
    println!("{times:?}");
    Ok(times.len().try_into().expect("it's out of n, which is u32"))
}

fn distinct_times(n: u32, root: &Path) -> Result<Vec<SystemTime>> {
    let names: Vec<_> = (0..n).map(|i| format!("{i:03}")).collect();
    for name in &names {
        std::fs::write(root.join(name), name)?;
    }
    let mut times = Vec::new();
    for name in names {
        times.push(root.join(name).symlink_metadata()?.modified()?);
    }
    times.sort();
    times.dedup();
    Ok(times)
}
