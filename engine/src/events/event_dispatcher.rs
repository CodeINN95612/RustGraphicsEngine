use log::warn;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub type BoxedFn = Box<dyn Fn(&dyn Any) -> bool>;
pub struct EventDispatcher {
    handlers: HashMap<TypeId, Vec<BoxedFn>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn bind<E, F>(&mut self, f: F)
    where
        E: 'static,
        F: Fn(&E) -> bool + 'static,
    {
        let e_type = TypeId::of::<E>();
        let f = Box::new(move |e: &dyn Any| -> bool {
            if let Some(event) = e.downcast_ref() {
                return f(event);
            }

            warn!("Unable to downcast '{:?}'", e);
            true
        });

        self.handlers.entry(e_type).or_insert_with(Vec::new).push(f);
    }

    pub fn trigger<E>(&self, e: E)
    where
        E: 'static,
    {
        let e_type = TypeId::of::<E>();
        if let Some(handlers) = self.handlers.get(&e_type) {
            for h in handlers {
                if !h(&e) {
                    return;
                }
            }
        }
    }
}
