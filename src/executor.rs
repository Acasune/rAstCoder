use run_script::types::ScriptResult;

pub struct Executor();

impl Executor {
    pub fn code_build(target: &str, working_dir: &str) -> ScriptResult<(i32, String, String)> {
        let build_script = format!(
            "cargo build --bin {}  --offline --quiet --manifest-path={}/Cargo.toml",
            target, working_dir
        );
        run_script::run_script!(&build_script)
    }
    pub fn run(execute_program_path: &str) -> ScriptResult<(i32, String, String)> {
        let run_script = format!("{}", execute_program_path);
        run_script::run_script!(run_script)
    }
}
