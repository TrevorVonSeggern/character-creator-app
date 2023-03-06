#[derive(Default, Clone, PartialEq, Eq)]
pub struct Thing {
    id: i32,
    categoryId: i32,
    name: String,
    jsonData: String,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Category {
    id: i32,
    nodePath: String,
    name: String,
    description: String,
    templateId: String,
    userId: i32,
}
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Template {
    id: i32,
    name: String,
    jsonData: String,
}
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Character {
    id: i32,
    name: String,
    edition: i32,
}
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Tag {
    id: i32,
    name: String,
}

// data for things.

#[derive(Default, Clone, PartialEq, Eq)]
pub struct ThingCollection {
}

//#[derive(Default, Clone, PartialEq, Eq)]
//pub enum ThingData {

//}
