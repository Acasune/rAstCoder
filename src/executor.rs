use run_script::types::ScriptResult;

pub struct Executor {
    source_program_path: String,
    execute_program_path: String,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            source_program_path: "./playground/main.rs".to_string(),
            execute_program_path: "./playground/a.out".to_string(),
        }
    }
    pub fn build() -> ScriptResult<(i32, String, String)> {
        run_script::run_script!(
            r#"
            cargo build --release --offline --quiet --manifest-path=./playground/Cargo.toml
            cp ./playground/target/release/main ./playground/a.out
            exit 0
             "#
        )
    }
    pub fn run(self, args: Vec<String>) -> ScriptResult<(i32, String, String)> {
        run_script::run_script!(self.execute_program_path)
    }
}
