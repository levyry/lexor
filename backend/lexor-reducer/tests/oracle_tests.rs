use lexor_reducer::SkiReductionStrat;
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::Command;
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug)]
enum SkiAst {
    S,
    K,
    I,
    App(Box<SkiAst>, Box<SkiAst>),
}

impl std::fmt::Display for SkiAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkiAst::S => write!(f, "S"),
            SkiAst::K => write!(f, "K"),
            SkiAst::I => write!(f, "I"),
            SkiAst::App(left, right) => {
                write!(f, "{}", left)?;
                match **right {
                    SkiAst::App(_, _) => write!(f, "({})", right),
                    _ => write!(f, "{}", right),
                }
            }
        }
    }
}

// Generate random SKI Expression Trees
fn ski_strategy() -> impl Strategy<Value = SkiAst> {
    let leaf = prop_oneof![Just(SkiAst::S), Just(SkiAst::K), Just(SkiAst::I),];

    leaf.prop_recursive(10, 300, 200, |inner| {
        (inner.clone(), inner).prop_map(|(l, r)| SkiAst::App(Box::new(l), Box::new(r)))
    })
}

// Oracle connection client
fn ask_oracle(expr: &str) -> Result<String, io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:1500")?;

    stream.set_read_timeout(Some(Duration::from_millis(5500)))?;
    stream.set_write_timeout(Some(Duration::from_millis(5100)))?;

    // server.sh uses read line, so we must append a newline
    writeln!(stream, "{}", expr)?;

    let mut result = String::new();
    stream.read_to_string(&mut result)?;

    let cleaned_result = result
        .lines()
        .filter(|line| !line.trim().is_empty())
        .last()
        .unwrap_or("")
        .trim()
        .to_owned();

    Ok(cleaned_result)
}

struct OracleGuard;

impl OracleGuard {
    fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let temp_dir = std::env::temp_dir().join(format!("ski_oracle_{}", timestamp));
        std::fs::create_dir_all(&temp_dir).expect("Failed to create temp build dir");

        let dockerfile_path = temp_dir.join("Dockerfile");
        let server_sh_path = temp_dir.join("server.sh");

        std::fs::write(&dockerfile_path, include_str!("Dockerfile")).unwrap();
        std::fs::write(&server_sh_path, include_str!("server.sh")).unwrap();

        let temp_dir_str = temp_dir.to_str().unwrap();

        let build_status = Command::new("podman")
            .args(["build", "-t", "ski-oracle", temp_dir_str])
            .status()
            .expect("Failed to execute podman build");
        assert!(build_status.success(), "Podman build failed");

        let _ = Command::new("podman")
            .args(["stop", "-t", "0", "ski-oracle-run"])
            .status();

        let run_status = Command::new("podman")
            .args([
                "run",
                "-d",
                "--rm",
                "--name",
                "ski-oracle-run",
                "-p",
                "1500:1500",
                "ski-oracle",
            ])
            .status()
            .expect("Failed to execute podman run");
        assert!(run_status.success(), "Podman run failed to start container");

        let mut attempts = 0;
        loop {
            if TcpStream::connect("127.0.0.1:1500").is_ok() {
                break;
            }
            attempts += 1;
            if attempts > 50 {
                panic!("Oracle container failed to open port 1500 within 5 seconds");
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        let _ = std::fs::remove_dir_all(temp_dir);

        OracleGuard
    }
}

impl Drop for OracleGuard {
    fn drop(&mut self) {
        let _ = Command::new("podman")
            .args(["stop", "-t", "0", "ski-oracle-run"])
            .status();
    }
}

#[test]
#[ignore = "Requires Podman. Run with `cargo test -- --ignored`"]
fn oracle_agrees_with_normal_form() {
    let _guard = OracleGuard::new();

    let mut runner = TestRunner::new(Config::with_cases(100));

    let result = runner.run(&ski_strategy(), |ast| {
        let expr_str = ast.to_string();

        let oracle_res = ask_oracle(&expr_str);
        prop_assume!(
            oracle_res.is_ok(),
            "Oracle timed out/failed, likely infinite loop."
        );

        let oracle_str = oracle_res.unwrap();
        let rust_res = SkiReductionStrat::NormalForm.reduce(&expr_str);

        prop_assert_eq!(rust_res.unwrap(), oracle_str,);

        Ok(())
    });

    result.unwrap();
}
