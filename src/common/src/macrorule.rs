#[macro_export]
macro_rules! output {
    ($output:expr, $($t:tt)*) => {
        FactoryConsole::new($output)$($t)*
    }
}

#[macro_export]
macro_rules! progress {
    ($message:expr, $($t:tt)*) => {
        {
            let pb = ProgressBar::new();

            pb.update(&$message)?;
            let result = $($t)*;
            pb.done_without_indicator("")?;
            result
        }
    };
}
