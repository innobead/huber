#[macro_export]
macro_rules! di {
    ($struct:ident $($attr:ident=$expr:expr )+) => {
        {
            let mut obj = $struct::new();
            $(obj.$attr = $expr;)+

            let mrc = DIContainer::new();
            let mut container = mrc.try_borrow_mut().unwrap();
            let obj = container.add(obj).expect(&format!("failed to add {} to DI container", std::any::type_name::<$struct>()));
        }
    };

    ($struct:ident $($t:tt)+) => {
        {
            let obj = $struct::new();
            let result = obj$($t)+;

            let mrc = DIContainer::new();
            let mut container = mrc.try_borrow_mut().unwrap();
            container.add(obj).expect(&format!("failed to add {} to DI container", std::any::type_name::<$struct>()));

            result
        }
    };

    ($struct:ident) => {
        {
            let obj = $struct::new();
            let mrc = DIContainer::new();
            let mut container = mrc.try_borrow_mut().unwrap();
            container.add(obj).expect(&format!("failed to add {} to DI container", std::any::type_name::<$struct>()))
        }
    };
}

#[macro_export]
macro_rules! output {
    ($output:expr, $($t:tt)*) => {
        FactoryConsole::new($output)$($t)*
    }
}
