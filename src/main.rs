use rAstCoder::arg_parser::ArgParser;
use rAstCoder::executor::Executor;
use rAstCoder::presenter::Presenter;
use rAstCoder::sample_preparator::{self, SamplePreparator};
use rAstCoder::tester::Tester;
use rAstCoder::types::Problem;

const source_program_path: &str = "./playground/main.rs";
const execute_program_path: &str = "./playground/a.out";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parsed = ArgParser::build(args).unwrap();
    let preparator = SamplePreparator {
        problem: parsed.problem,
    };
    let (input_dir, output_dir) = preparator.prepare();
    let executor = Executor::new(source_program_path, execute_program_path);
    let result = executor.code_build().unwrap();
    let mut tester = Tester::new(executor, input_dir, output_dir);
    tester.test();
    let mut presenter = Presenter {};
    presenter.validate(&tester.results);
}
