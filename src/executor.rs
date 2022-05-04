use run_script::types::ScriptResult;

pub struct Executor {
    source_program_path: String,
    execute_program_path: String,
}

impl Executor {
    pub fn new(source_program_path: &str, execute_program_path: &str) -> Self {
        Executor {
            source_program_path: source_program_path.to_string(),
            execute_program_path: execute_program_path.to_string(),
        }
    }
    pub fn code_build(&self) -> ScriptResult<(i32, String, String)> {
        run_script::run_script!(
            r#"
            cargo build --release --offline --quiet --manifest-path=./playground/Cargo.toml
            cp ./playground/target/release/main ./playground/a.out
             "#
        )
    }
    pub fn run(&self, script: String) -> ScriptResult<(i32, String, String)> {
        run_script::run_script!(script)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_run_testcase() {
        // let executor = Executor::new();
        // executor.run();
    }
}
