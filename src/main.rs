use rAstCoder::executor::Executor;
use rAstCoder::sample_preparator::{self, SamplePreparator};
use rAstCoder::tester::Tester;
use rAstCoder::types::Problem;

fn main() {
    // let problem = Problem::new("abc".to_string(), 249, "a".to_string());
    // let preparator = SamplePreparator { problem: problem };
    // let dir = preparator.prepare();
    let executor = Executor::new();
    executor.build();
    // let tester = Tester::new(executor, "./testcase/abc249_a/input".to_string());
    // tester.test();
}
