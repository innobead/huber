use std::any::{type_name, Any};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use crate::result::Result;

pub type MutableRc<T> = Rc<RefCell<T>>;

static mut CONTAINER: Option<MutableRc<DIContainer>> = None;

lazy_static! {
    static ref MUTEX: Mutex<u8> = Mutex::new(0);
}

pub trait DIObjectTrait {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self;
}

#[derive(Debug)]
pub struct DIContainer {
    objects: HashMap<String, Box<dyn Any>>,
}

impl DIContainer {
    pub fn new() -> MutableRc<Self> {
        unsafe {
            let _ = MUTEX.lock();

            if let None = CONTAINER {
                let rc = Rc::new(RefCell::new(Self {
                    objects: Default::default(),
                }));

                CONTAINER = Some(rc.clone());
            }

            CONTAINER.clone().unwrap().clone()
        }
    }

    pub fn add<T>(&mut self, obj: T) -> Result<&mut T>
    where
        T: 'static,
    {
        let key = type_name::<T>().to_string();
        self.objects.insert(key.clone(), Box::new(obj));

        Ok(self.get_mut::<T>().unwrap())
    }

    pub fn get<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        self.objects
            .get(&type_name::<T>().to_string())
            .unwrap()
            .downcast_ref()
    }

    pub fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static,
    {
        self.objects
            .get_mut(&type_name::<T>().to_string())
            .unwrap()
            .downcast_mut()
    }
}

#[macro_export]
macro_rules! di {
    ($struct:ident $($attr:ident=$expr:expr )+) => {
        {
            let container_rc = DIContainer::new();
            let mut container = container_rc.borrow_mut();
            let obj = container.add($struct::new_for_di(container_rc.clone())).expect(&format!("failed to add {} to DI container", std::any::type_name::<$struct>()));

            $(
            obj.$attr = $expr;
            )+
        }
    };

    ($struct:ident $($t:tt)*) => {
        {
            let container_rc = DIContainer::new();
            let mut container = container_rc.borrow_mut();
            container.add($struct::new_for_di(container_rc.clone())).expect(&format!("failed to add {} to DI container", std::any::type_name::<$struct>()))$($t)*
        }
    };
}

#[macro_export]
macro_rules! di_aware {
    ($struct:ident) => {{
        let container_rc = DIContainer::new();
        let container = container_rc.borrow_mut();
        $struct::new_for_di(container_rc.clone())
    }};
}
