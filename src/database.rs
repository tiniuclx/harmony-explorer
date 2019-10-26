use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::*;

embed_migrations!("migrations/");

table! {
    names (chord) {
        chord -> Text,
        alternative_name -> Nullable<Text>,
    }
}

table! {
    notes (chord, interval) {
        chord -> Text,
        degree -> Integer,
        interval -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(names, notes,);

pub fn initialise_database() -> Result<SqliteConnection, Box<dyn std::error::Error>> {
    let connection = SqliteConnection::establish(":memory:")?;
    embedded_migrations::run(&connection)?;
    Ok(connection)
}
