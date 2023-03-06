#[derive(Default, Clone, PartialEq, Eq)]
pub struct Thing {
    pub id: i32,
    pub categoryId: i32,
    pub name: String,
    pub jsonData: String,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Category {
    pub id: i32,
    pub nodePath: String,
    pub name: String,
    pub description: String,
    pub templateId: String,
    pub userId: i32,
}
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub jsonData: String,
}
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub edition: i32,
}
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

// data for things.

#[derive(Default, Clone, PartialEq, Eq)]
pub struct ThingCollection {
}

//#[derive(Default, Clone, PartialEq, Eq)]
//pub enum ThingData {

//}
