#[macro_export]
macro_rules! create_dep {
    ($struct:ident, $container:expr) => {{
        let obj = $struct::new();
        $container.add(obj).unwrap()
    }};

    ($struct:ident, $container:expr, $($t:tt)+) => {{
        let obj = $struct::new();
        $container.add(obj).unwrap()$($t)+
    }};
}

#[macro_export]
macro_rules! inject_dep {
    ($struct:ident, $config:expr, $container:expr) => {{
        unsafe {
            let ptr = $container.get::<$struct>().unwrap() as *const $struct;
            let ptr = ptr as *mut $struct;
            let s = &mut *ptr;

            s.set_shared_properties($config, $container);
        }
    }};
}

#[macro_export]
macro_rules! output {
    ($output:expr, $($t:tt)*) => {
        FactoryConsole::new($output)$($t)*
    }
}
