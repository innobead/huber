#[macro_export]
macro_rules! output {
    ($output:expr, $($t:tt)*) => {
        FactoryConsole::new($output)$($t)*
    }
}
