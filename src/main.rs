use std::env;

use rAstCoder::arg_parser::ArgParser;
use rAstCoder::executor::Executor;
use rAstCoder::presenter::Presenter;
use rAstCoder::sample_preparator::SamplePreparator;
use rAstCoder::tester::Tester;

const SOURCE_PROGRAM_PATH: &str = "./playground/main.rs";
const EXECUTE_PROGRAM_PATH: &str = "./playground/a.out";
const BUILD_SCRIPT: &str = r#"
cargo build --release --offline --quiet --manifest-path=./playground/Cargo.toml
cp ./playground/target/release/main ./playground/a.out
 "#;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let working_dir = env::current_dir().unwrap().display().to_string();
    let parsed = ArgParser::build(args, working_dir).unwrap();
    let preparator = SamplePreparator::new(parsed.problem);
    let (input_dir, output_dir) = preparator.prepare().unwrap();
    let executor = Executor::new(SOURCE_PROGRAM_PATH, EXECUTE_PROGRAM_PATH);
    let result = executor.code_build(BUILD_SCRIPT).unwrap();
    let mut tester = Tester::new(executor, input_dir, output_dir);
    tester.test();
    let mut presenter = Presenter {};
    presenter.validate(&tester.results);
}
