#[derive(Queryable)]
pub struct FirstPersons {
    pub id: i32,
    pub value: String,
    pub ruby: String,
    pub comment: String,
}
