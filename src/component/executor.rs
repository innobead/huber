use crate::base::result::Result;

trait ExecutorTrait {
    fn run(exec: &str, args: &Vec<&str>) -> Result<String>;
}

struct ShellExecutor;

struct HelmExecutor;

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
