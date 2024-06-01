#![allow(dead_code)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

trait Database {
    fn name(&self) -> &'static str;
}

struct MySql;
struct Postgres;

impl Database for MySql {
    fn name(&self) -> &'static str {
        "MySql"
    }
}

impl Database for Postgres {
    fn name(&self) -> &'static str {
        "Postgres"
    }
}

struct WebServer {
    db: Box<dyn Database>,
}

impl WebServer {
    fn run(&self) {
        println!("Db name: {}", self.db.name());
    }
}

trait DIBuilder {
    type Input;
    type Output;

    fn build(input: Self::Input) -> Self::Output;
}

impl DIBuilder for MySql {
    type Input = ();

    type Output = Box<dyn Database>;

    fn build(_: Self::Input) -> Self::Output {
        Box::new(MySql)
    }
}

impl DIBuilder for Postgres {
    type Input = ();

    type Output = Box<dyn Database>;

    fn build(_: Self::Input) -> Self::Output {
        Box::new(Postgres)
    }
}

impl DIBuilder for WebServer {
    type Input = (Box<dyn Database>,);

    type Output = WebServer;

    fn build((db,): Self::Input) -> Self::Output {
        WebServer { db }
    }
}

struct TypeMap(HashMap<TypeId, Box<dyn Any>>);
impl TypeMap {
    fn set<T: Any + 'static>(&mut self, value: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(value));
    }

    fn hash<T: Any + 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }

    fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.0
            .get_mut(&TypeId::of::<T>())
            .map(|it| it.downcast_mut::<T>().unwrap())
    }
}

struct DIManager(TypeMap);
type DIObj<T> = Arc<Mutex<T>>;
impl DIManager {
    fn build<T: DIBuilder>(&mut self) -> Option<DIObj<T::Output>> {
        todo!()
    }
}
