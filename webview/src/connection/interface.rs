use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Connector {
    fn add<T>(&self, item: T)
    where
        T: Serialize;

    fn get<T>(&self) -> Vec<T>
    where
        T: Serialize;

    fn update<T>(&self, item: T)
    where
        T: Serialize;

    fn delete<T>(&self, item: T)
    where
        T: Serialize;
}
