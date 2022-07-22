use crate::schema::tb_user;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct TbUser {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

// this is to insert users to database
#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "tb_user"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}
