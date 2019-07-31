#[derive(Queryable)]
pub struct FirstPersons {
    pub id: i32,
    pub value: String,
    pub ruby: Option<String>,
    pub comment: Option<String>,
}
