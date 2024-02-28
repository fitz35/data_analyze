use std::io::{self, Read};
use std::process::{Command, Stdio};

#[macro_use]
extern crate approx;

mod common;

#[test]
fn test_random_stats_series() -> io::Result<()> {
    for size in common::get_stats_size_to_test() {
        for _ in 0..10 {
            let mut cmd = Command::new("python3");
            cmd.arg(common::PYTHON_STATS_EXE_PATH)
                .arg("-s")
                .arg(size.to_string());
            
            cmd.stdout(Stdio::piped());
            let mut child = cmd.spawn()?;

            let mut stdout = child.stdout.take().expect("Failed to get stdout handle");
            let mut output = String::new();
            stdout.read_to_string(&mut output)?;
            let status = child.wait()?;
            assert!(status.success());

            let test_serie : common::stats_helper::TestSerie = serde_json::from_str(output.as_str()).unwrap();
            common::stats_helper::test_stats(&test_serie, true);
        }
    }

    Ok(())
}