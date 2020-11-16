use std::any::{type_name, Any};

use std::collections::HashMap;
use std::sync::Mutex;

use crate::result::Result;

#[derive(Debug)]
pub struct DIContainer {
    objects: HashMap<String, Box<dyn Any>>,
    mutex: Mutex<u8>,
}
unsafe impl Send for DIContainer {}
unsafe impl Sync for DIContainer {}

impl DIContainer {
    pub fn new() -> Self {
        DIContainer {
            objects: Default::default(),
            mutex: Mutex::new(0),
        }
    }

    pub fn add<T>(&mut self, obj: T) -> Result<&mut T>
    where
        T: 'static,
    {
        let _ = self.mutex.lock();
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
