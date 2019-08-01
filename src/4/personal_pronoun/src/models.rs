#[derive(Queryable)]
pub struct FirstPersons {
    pub id: i32,
    pub value: String,
    pub ruby: String,
    pub comment: String,
}
#[derive(Queryable)]
pub struct SecondPersons {
    pub id: i32,
    pub value: String,
    pub ruby: String,
    pub comment: String,
}
#[derive(Queryable)]
pub struct ThirdPersons {
    pub id: i32,
    pub value: String,
    pub ruby: String,
    pub comment: String,
}
// https://docs.diesel.rs/diesel/deserialize/trait.QueryableByName.html
// https://docs.diesel.rs/diesel/sql_types/
use diesel::sql_types::{Integer,Text};
#[derive(QueryableByName, Debug)]
#[table_name = "FirstPersons"]
pub struct FirstPersonsRaw {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub value: String,
    #[sql_type = "Text"]
    pub ruby: String,
    #[sql_type = "Text"]
    pub comment: String,
}
#[derive(QueryableByName)]
#[table_name = "SecondPersons"]
pub struct SecondPersonsRaw {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub value: String,
    #[sql_type = "Text"]
    pub ruby: String,
    #[sql_type = "Text"]
    pub comment: String,
}
#[derive(QueryableByName)]
#[table_name = "ThirdPersons"]
pub struct ThirdPersonsRaw {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub value: String,
    #[sql_type = "Text"]
    pub ruby: String,
    #[sql_type = "Text"]
    pub comment: String,
}
/*
#[derive(QueryableByName)]
#[table_name = "FirstPersons"]
pub struct FirstPersonsRaw {
    pub id: i32,
    pub value: String,
    pub ruby: String,
    pub comment: String,
}
#[derive(QueryableByName)]
#[table_name = "SecondPersons"]
pub struct SecondPersonsRaw {
    pub id: i32,
    pub value: String,
    pub ruby: String,
    pub comment: String,
}
#[derive(QueryableByName)]
#[table_name = "ThirdPersons"]
pub struct ThirdPersonsRaw {
    pub id: i32,
    pub value: String,
    pub ruby: String,
    pub comment: String,
}
*/
