use schema::feeds;

#[derive(Queryable)]
pub struct Feed {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name="feeds"]
pub struct NewFeed<'a> {
    pub title: &'a str,
    pub link: &'a str,
}
