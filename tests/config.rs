use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

const CARGO_TARGET_TMPDIR: &str = env!("CARGO_TARGET_TMPDIR");
const CARGO_BIN_EXE: &str = env!("CARGO_BIN_EXE_snoop");

const WORKSPACE_CONFIG: &str = r#"
{
  "$schema": "mock-config",
  "packages": [
    { "name": "package1", "path": "./package1" },
    { "name": "package2", "path": "package2" }
  ]
}
"#;

const PACKAGE1_CONFIG: &str = r#"
{
  "$schema": "mock-config",
  "commands": [
    { "name": "build", "command": "echo", "args": ["build"] },
    { "name": "run", "command": "echo", "args": ["run"] }
  ]
}
"#;

const PACKAGE2_CONFIG: &str = r#"
{
  "commands": [
    { "name": "build", "command": "echo" },
    { "name": "run", "command": "echo", "args": ["run"] }
  ]
}
"#;

struct TestContext {
    workspace_path: PathBuf,
}

impl TestContext {
    fn new() -> Self {
        let workspace_path = Path::new(CARGO_TARGET_TMPDIR).join("config");
        let package1_path = workspace_path.join("package1");
        let package2_path = workspace_path.join("package2");

        Self::save_json(&workspace_path, WORKSPACE_CONFIG);
        Self::save_json(&package1_path, PACKAGE1_CONFIG);
        Self::save_json(&package2_path, PACKAGE2_CONFIG);

        Self { workspace_path }
    }

    fn save_json(path: &Path, contents: &str) {
        fs::create_dir_all(path).unwrap();
        fs::write(path.join("snoop.json"), contents).unwrap();
    }

    fn run_command(&self, args: &[&str], current_dir: &Path) -> Output {
        Command::new(CARGO_BIN_EXE)
            .current_dir(current_dir)
            .args(args)
            .output()
            .unwrap()
    }

    fn assert_output(&self, output: Output, expected_stdout: &str) {
        assert!(output.status.success(), "Command failed");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            expected_stdout,
            "Unexpected output"
        );
    }
}

fn setup() -> TestContext {
    TestContext::new()
}

#[test]
fn workspace() {
    let context = setup();

    let cases = [
        (["build", "package1"], "build\n"),
        (["run", "package1"], "run\n"),
        (["build", "package2"], "\n"),
        (["run", "package2"], "run\n"),
    ];

    for (args, expected_output) in cases {
        let output = context.run_command(&args, &context.workspace_path);
        context.assert_output(output, expected_output);
    }
}

#[test]
fn package() {
    let context = setup();
    let package2_path = context.workspace_path.join("package2");

    let cases = [(["build"], "\n"), (["run"], "run\n")];

    for (args, expected_output) in cases {
        let output = context.run_command(&args, &package2_path);
        context.assert_output(output, expected_output);
    }
}

#[test]
fn different_root_path() {
    let context = setup();
    let binding = context.workspace_path.join("package2");
    let package2_path = binding.to_str().unwrap();
    let root_path = Path::new("/");

    let cases = [
        (["build", "--root-path", package2_path], "\n"),
        (["run", "--root-path", package2_path], "run\n"),
    ];

    for (args, expected_output) in cases {
        let output = context.run_command(&args, root_path);
        context.assert_output(output, expected_output);
    }
}
