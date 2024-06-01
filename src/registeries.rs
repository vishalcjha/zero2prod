#![allow(dead_code)]

mod string_based {
    use std::{any::Any, collections::HashMap};

    pub type StringEventListener = Box<dyn Fn(&dyn Any)>;

    #[derive(Default)]
    pub struct StringEventRegistry {
        listeners: HashMap<String, Vec<StringEventListener>>,
    }

    impl StringEventRegistry {
        pub fn add_event_listener(&mut self, event: String, f: StringEventListener) {
            self.listeners
                .entry(event)
                .or_insert_with(|| Vec::new())
                .push(f);
        }

        pub fn trigger(&self, event: String, data: &dyn Any) {
            let listeners = self.listeners.get(&event).unwrap();
            for listener in listeners.iter() {
                listener(data);
            }
        }
    }

    pub struct OnClick {
        pub mouse_x: f32,
        pub mouse_y: f32,
    }
}

mod type_based {
    use std::{
        any::{Any, TypeId},
        collections::HashMap,
    };

    struct EventDispatcher(TypeMap);

    struct TypeMap(HashMap<TypeId, Box<dyn Any>>);

    impl TypeMap {
        pub fn set<T: Any + 'static>(&mut self, t: T) {
            self.0.insert(TypeId::of::<T>(), Box::new(t));
        }

        pub fn has<T: Any + 'static>(&self) -> bool {
            self.0.contains_key(&TypeId::of::<T>())
        }

        pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
            self.0
                .get_mut(&TypeId::of::<T>())
                .map(|t| t.downcast_mut::<T>().unwrap())
        }
    }
}
#[cfg(test)]
mod test {
    use super::string_based::{OnClick, StringEventRegistry};

    #[test]
    fn test_string_based_event_listener() {
        let mut events = StringEventRegistry::default();
        events.add_event_listener(
            "click".to_owned(),
            Box::new(|event| {
                let event = event.downcast_ref::<OnClick>().unwrap();
                assert_eq!(event.mouse_x, 1.);
            }),
        );
        let event = OnClick {
            mouse_x: 1.,
            mouse_y: 3.,
        };
        events.trigger("click".to_owned(), &event);
    }
}
