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
    pub fn code_build(&self, build_script: &str) -> ScriptResult<(i32, String, String)> {
        self.run(build_script)
    }
    pub fn run(&self, run_script: &str) -> ScriptResult<(i32, String, String)> {
        run_script::run_script!(run_script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const source_program_path: &str = "./playground/main.rs";
    const execute_program_path: &str = "./playground/a.out";

    #[test]
    fn test_build_success() {
        let build_script = r#"
        rustc ./resources/test_resources/case1_success.rs --out-dir ./resources/test_resources/
         "#;
        let run_script = r#"
         ./resources/test_resources/case1_success
          "#;
        let executor = Executor::new(source_program_path, execute_program_path);
        executor.code_build(build_script).unwrap();
        let (status, content, _) = executor.run(run_script).unwrap();
        assert_eq!(0, status);
        assert_eq!("Hello, world!\n", content)
    }
    #[test]
    fn test_compile_error() {
        let build_script = r#"
        rustc ./resources/test_resources/case2_compile_error.rs --out-dir ./resources/test_resources/
         "#;
        let executor = Executor::new(source_program_path, execute_program_path);
        let (status, _, error) = executor.code_build(build_script).unwrap();
        assert_eq!(1, status);
    }
    #[test]
    fn test_runtime_error() {
        let build_script = r#"
        rustc ./resources/test_resources/case3_runtime_error.rs --out-dir ./resources/test_resources/
         "#;
        let run_script = r#"
         ./resources/test_resources/case3_runtime_error
          "#;
        let executor = Executor::new(source_program_path, execute_program_path);
        executor.code_build(build_script).unwrap();
        let (status, _, error) = executor.run(run_script).unwrap();
        assert_eq!(101, status);
    }
}
