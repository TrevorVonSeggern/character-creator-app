use std::rc::Rc;
use yewdux::prelude::*;

use crate::entities::Thing;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct Counter {
    count: i32,
}

//#[derive(Default, Clone, PartialEq, Eq, Store)]
#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct AppState {
    count: i32,
    loading: bool,
    loading_count: i32,
    things: Vec<Thing>,
}

pub enum Action {
    AddOne,
    Minus,
    Clear,
}

impl Reducer<Counter> for Action {
    fn apply(self, mut counter: Rc<Counter>) -> Rc<Counter> {
        let state = Rc::make_mut(&mut counter);
        match self {
            Action::AddOne => state.count += 1,
            Action::Minus => state.count -= 1,
            Action::Clear => state.count = 0,
        };

        counter
    }
}


