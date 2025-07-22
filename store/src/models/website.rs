use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]

struct User{
    id: String,
    url: String,
    user_id : String,
    time_added: chrono::NaiveDateTime,
}

impl Store{
    pub fn create_website(&self){
        print!("Create user called");

    }
    pub fn get_website(&self) -> String {
        String::from("1")
    }
}