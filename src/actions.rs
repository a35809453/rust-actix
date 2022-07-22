use crate::models::{self, NewUser};
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    new_user: NewUser,
    conn: &MysqlConnection,
) -> Result<models::NewUser, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::tb_user::dsl::*;

    // let new_user = models::NewUser {
    //     // id: Uuid::new_v4().to_string(),
    //     username: nm.to_owned(),
    //     password: "".to_string(),
    //     email: ""
    // };
    println!("{:#?}",new_user);
    diesel::insert_into(tb_user).values(&new_user).execute(conn)?;

    Ok(new_user)
}
