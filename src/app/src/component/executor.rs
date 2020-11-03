use huber_common::result::Result;

pub(crate) trait ExecutorTrait {
    fn run(exec: &str, args: &Vec<&str>) -> Result<String>;
}

#[derive(Debug)]
pub(crate) struct ShellExecutor;

#[derive(Debug)]
pub(crate) struct HelmExecutor;

impl ExecutorTrait for ShellExecutor {
    fn run(_exec: &str, _args: &Vec<&str>) -> Result<String> {
        unimplemented!()
    }
}

impl ExecutorTrait for HelmExecutor {
    fn run(_exec: &str, _args: &Vec<&str>) -> Result<String> {
        unimplemented!()
    }
}
