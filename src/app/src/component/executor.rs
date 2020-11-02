use huber_common::result::Result;

pub(crate) trait ExecutorTrait {
    fn run(exec: &str, args: &Vec<&str>) -> Result<String>;
}

pub(crate) struct ShellExecutor;

pub(crate) struct HelmExecutor;

impl ExecutorTrait for ShellExecutor {
    fn run(exec: &str, args: &Vec<&str>) -> Result<String> {
        unimplemented!()
    }
}

impl ExecutorTrait for HelmExecutor {
    fn run(exec: &str, args: &Vec<&str>) -> Result<String> {
        unimplemented!()
    }
}
