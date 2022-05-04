use rAstCoder::arg_parser::ArgParser;
use rAstCoder::executor::Executor;
use rAstCoder::presentor::Presenter;
use rAstCoder::sample_preparator::{self, SamplePreparator};
use rAstCoder::tester::Tester;
use rAstCoder::types::Problem;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let parsed = ArgParser::build(args).unwrap();
    // let problem = Problem::new("abc".to_string(), 249, "a".to_string());
    let preparator = SamplePreparator {
        problem: parsed.problem,
    };
    let (input_dir, output_dir) = preparator.prepare();
    let executor = Executor::new();
    let result = executor.code_build().unwrap();
    let mut tester = Tester::new(executor, input_dir, output_dir);
    tester.test();
    let mut presenter = Presenter {};
    presenter.validate(&tester.results);
}
