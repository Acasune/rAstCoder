use run_script::types::ScriptResult;

pub struct Executor {
    source_program_path: String,
    binary_file_path: String,
    target: String,
}

impl Executor {
    pub fn code_build(target: &str, working_dir: &str) -> ScriptResult<(i32, String, String)> {
        let build_script = format!(
            "cargo build --bin {}  --offline --quiet --manifest-path={}/Cargo.toml",
            target, working_dir
        );
        println!("{}", build_script);
        run_script::run_script!(&build_script)
    }
    pub fn run(execute_program_path: &str) -> ScriptResult<(i32, String, String)> {
        let run_script = format!("{}", execute_program_path);
        run_script::run_script!(run_script)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_build_success() {
//         const file_name: &str = "./playground/main.rs";
//         let working_dir = ".";
//         const execute_program_path: &str = "./playground/a.out";
//         let build_script = format!(
//             "cargo build --bin {} --release --offline --quiet --manifest-path={}/Cargo.toml",
//             file_name, working_dir
//         );
//         let run_script = format!("{}", execute_program_path);
//         executor.code_build(&build_script).unwrap();
//         let (status, content, _) = executor.run(&run_script).unwrap();
//         assert_eq!(0, status);
//         assert_eq!("Hello, world!\n", content)
//     }
//     #[test]
//     fn test_compile_error() {
//         let executor = Executor::new(source_program_path, execute_program_path);
//         let (status, _, error) = executor.code_build(build_script).unwrap();
//         assert_eq!(1, status);
//     }
//     #[test]
//     fn test_runtime_error() {
//         let build_script = r#"
//         rustc ./test_resources/case3_runtime_error.rs --out-dir ./test_resources/
//          "#;
//         let run_script = r#"
//          ./test_resources/case3_runtime_error
//           "#;
//         let executor = Executor::new(source_program_path, execute_program_path);
//         executor.code_build(build_script).unwrap();
//         let (status, _, error) = executor.run(run_script).unwrap();
//         assert_eq!(101, status);
//     }
// }
