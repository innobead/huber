use std::any::{type_name, Any};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::result::Result;

pub type MutableArc<T> = Arc<RefCell<T>>;

static mut CONTAINER: Option<MutableArc<DIContainer>> = None;

lazy_static! {
    static ref MUTEX: Mutex<u8> = Mutex::new(0);
}

pub fn di_container() -> Ref<'static, DIContainer> {
    unsafe { CONTAINER.as_ref().unwrap().try_borrow().unwrap() }
}

#[derive(Debug)]
pub struct DIContainer {
    objects: HashMap<String, Box<dyn Any>>,
}

impl DIContainer {
    pub fn new() -> MutableArc<Self> {
        unsafe {
            let _ = MUTEX.lock();

            if CONTAINER.is_none() {
                CONTAINER = Some(Arc::new(RefCell::new(Self {
                    objects: Default::default(),
                })));
            }

            CONTAINER.as_ref().unwrap().clone()
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
