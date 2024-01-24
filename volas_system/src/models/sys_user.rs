use std::default;

use salvo::oapi::{Object, Schema, ToSchema};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
pub const USER_TABLE: &str = "sys_user";
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SysUser {
    pub id: Option<MyThing>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
struct MyThing(surrealdb::sql::Thing);

impl ToSchema for MyThing {
    fn to_schema(
        _components: &mut salvo::oapi::Components,
    ) -> salvo::oapi::RefOr<salvo::oapi::schema::Schema> {
        Schema::Object(Object::default()).into()
    }
}

impl From<surrealdb::sql::Thing> for MyThing {
    fn from(thing: surrealdb::sql::Thing) -> Self {
        MyThing(thing)
    }
}

impl Into<surrealdb::sql::Thing> for MyThing {
    fn into(self) -> surrealdb::sql::Thing {
        self.0
    }
}
