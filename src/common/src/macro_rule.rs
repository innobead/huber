#[macro_export]
macro_rules! di {
    ($struct:ident $($attr:ident=$expr:expr )*) => {
        {
            let mrc = DIContainer::new();
            let mut container = mrc.try_borrow_mut().unwrap();
            let obj = container.add($struct::new()).expect(&format!("failed to add {} to DI container", std::any::type_name::<$struct>()));

            $(obj.$attr = $expr;)*
        }
    };

    ($struct:ident $($t:tt)*) => {
        {
            let mrc = DIContainer::new();
            let mut container = mrc.try_borrow_mut().unwrap();
            container.add($struct::new()).expect(&format!("failed to add {} to DI container", std::any::type_name::<$struct>()))$($t)*
        }
    };
}

#[macro_export]
macro_rules! output {
    ($output:expr, $($t:tt)*) => {
        FactoryConsole::new($output)$($t)*
    }
}
