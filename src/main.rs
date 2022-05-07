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
 "#;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let working_dir = env::current_dir().unwrap().display().to_string();
    let parsed = ArgParser::build(args, &working_dir).unwrap();
    let preparator = SamplePreparator::new(parsed.problem);
    let (input_dir, output_dir) = preparator.prepare().unwrap();
    let build_result = Executor::code_build(&parsed.target, working_dir.as_str());
    let mut tester = Tester::new(input_dir, output_dir);
    let test_result = tester.test(&parsed.binary_path);
    let mut presenter = Presenter {};
    presenter.validate(&tester.results);
}
