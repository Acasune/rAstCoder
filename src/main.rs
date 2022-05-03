use rAstCoder::executor::Executor;
use rAstCoder::sample_preparator::{self, SamplePreparator};
use rAstCoder::tester::Tester;
use rAstCoder::types::Problem;

fn main() {
    let problem = Problem::new("abc".to_string(), 249, "a".to_string());
    let preparator = SamplePreparator { problem: problem };
    let (input_dir, output_dir) = preparator.prepare();
    let executor = Executor::new();
    let result = executor.build().unwrap();
    let mut tester = Tester::new(executor, input_dir, output_dir);
    tester.test();
}
