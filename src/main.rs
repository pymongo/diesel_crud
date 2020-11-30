/*!
schema of table users:

```sql
CREATE TABLE users (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	email TEXT NOT NULL UNIQUE,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

[SQLite Current Timestamp with Milliseconds?](https://stackoverflow.com/questions/17574784/sqlite-current-timestamp-with-milliseconds)
```
*/
mod schema {
    table! {
        users (id) {
            id -> Integer,
            email -> Text,
            // SQLite's TIMESTAMP type is map to Timestamp in diesel_schema
            created_at -> Timestamp,
        }
    }
}
mod models {
    use super::schema::users;
    #[derive(Queryable, Debug)]
    pub struct User {
        pub id: i32,
        pub email: String,
        /// deisel create must enable chrono feature
        /// Timestamp without timezone, the memory align of Timestamp type in sqlite is same as libc::timeval?
        pub created_at: chrono::NaiveDateTime,
    }

    #[derive(Insertable)]
    #[table_name = "users"]
    pub struct UserInsert {
        pub email: String,
    }
}
#[macro_use]
extern crate diesel;
use diesel::{
    result::Error as DieselError, sql_types::BigInt, sqlite::SqliteConnection, Connection,
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use models::{User, UserInsert};
use schema::users::dsl::{created_at, id, users};

fn create_user(conn: &SqliteConnection, new_user_form: UserInsert) -> Result<User, DieselError> {
    // use sqlite(last_insert_rowid)/mysql(last_insert_id) to get current connection's last_insert_id
    // use .order(id.desc()).last() will get the wrong id when multi db_connections insert at same time
    no_arg_sql_function!(last_insert_rowid, BigInt);
    diesel::insert_into(users)
        .values(&new_user_form)
        .execute(conn)?;
    let new_user_id: i64 = diesel::select(last_insert_rowid).first(conn)?;
    let last_insert_user: User = users.filter(id.eq(new_user_id as i32)).first(conn)?;
    Ok(last_insert_user)
}

fn read_users(conn: &SqliteConnection) -> Result<Vec<User>, DieselError> {
    Ok(users.load::<User>(conn)?)
}

fn update_user_created_at(conn: &SqliteConnection, user_id: i32) -> Result<(), DieselError> {
    diesel::update(users.filter(id.eq(user_id)))
        .set(created_at.eq(chrono::Utc::now().naive_utc()))
        .execute(conn)?;
    Ok(())
}

fn delete_user_by_user_id(conn: &SqliteConnection, user_id: i32) -> Result<(), DieselError> {
    diesel::delete(users.filter(id.eq(user_id))).execute(conn)?;
    Ok(())
}

/// diesel CRUD(Create, Read, Update, Delete) example with datetime
/// optional: use r2d2 db_pool to enhance diesel performance
fn main() -> Result<(), DieselError> {
    let conn = SqliteConnection::establish("file:db.sqlite").unwrap();
    // clear all data before test
    diesel::delete(users).execute(&conn)?;
    let test_user_email = format!(
        "test+{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    // CRUD - Create
    println!("\nCRUD - Create");
    let last_insert_user = create_user(
        &conn,
        UserInsert {
            email: test_user_email,
        },
    )?;
    dbg!(&last_insert_user);
    // CRUD - Read
    println!("\nCRUD - Read");
    dbg!(read_users(&conn)?);
    assert_eq!(read_users(&conn)?[0].id, last_insert_user.id);
    // CRUD - Update
    println!("\nCRUD - Update");
    update_user_created_at(&conn, last_insert_user.id)?;
    dbg!(read_users(&conn)?);
    assert_ne!(read_users(&conn)?[0].created_at, last_insert_user.created_at);
    // CRUD - Delete
    println!("\nCRUD - Delete");
    delete_user_by_user_id(&conn, last_insert_user.id)?;
    dbg!(read_users(&conn)?);
    assert!(read_users(&conn)?.is_empty());
    Ok(())
}
