use std::{rc::Rc, vec};
use yewdux::prelude::*;

use crate::entities::Thing;

//#[derive(Default, Clone, PartialEq, Eq, Store)]
#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct AppState {
    count: i32,
    loading: bool,
    loading_count: i32,
    pub things: Vec<Thing>,
}

pub enum Action {
    AddThings(Vec<Thing>),
}

impl Reducer<AppState> for Action {
    fn apply(self, mut counter: Rc<AppState>) -> Rc<AppState> {
        let state = Rc::make_mut(&mut counter);
        match self {
            Action::AddThings(t) => {
                for e in t {
                    state.things.push(e);
                }
            },
        };
        counter
    }
}


pub fn CreateManyThings(num: i32) -> Vec<Thing> {
    let mut r: Vec<Thing> = vec![];
    for n in 0..num {
        r.push(Thing { id: n, categoryId:10, name: "Thing 1".to_string(), jsonData: "".to_string() })
    }
    return r;
}
