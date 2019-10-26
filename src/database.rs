use crate::music_theory::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::*;

embed_migrations!("migrations/");

#[derive(Debug, PartialEq, Eq, Queryable, Insertable)]
#[table_name = "notes"]
pub struct Note {
    pub chord: String,
    pub degree: Degree,
    pub interval: Interval,
}

#[derive(Debug, PartialEq, Eq, Queryable, Insertable)]
#[table_name = "names"]
pub struct ChordName {
    pub chord: String,
    pub alternative_name: String,
}

pub fn initialise_database() -> Result<SqliteConnection, Box<dyn std::error::Error>> {
    let connection = SqliteConnection::establish(":memory:")?;
    embedded_migrations::run(&connection)?;
    Ok(connection)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        use degree::*;
        use interval::*;
        let conn = initialise_database().unwrap();

        let inserted_notes = vec![
            Note {
                chord: "maj".to_string(),
                degree: III,
                interval: Maj3rd,
            },
            Note {
                chord: "maj".to_string(),
                degree: V,
                interval: Per5th,
            },
        ];

        // We are inserting two notes, make sure that is the case
        assert_eq!(
            2,
            diesel::insert_into(notes::table)
                .values(inserted_notes)
                .execute(&conn)
                .expect("Could not insert note")
        );

        let inserted_name = ChordName {
            chord: "maj".to_string(),
            alternative_name: "major".to_string(),
        };

        assert_eq!(
            1,
            diesel::insert_into(names::table)
                .values(inserted_name)
                .execute(&conn)
                .expect("Could not insert name")
        );
    }

    #[test]
    fn retrieve() {
        use super::*;
        use degree::*;
        use interval::*;
        let conn = initialise_database().unwrap();

        let new_note = Note {
            chord: "maj".to_string(),
            degree: III,
            interval: Maj3rd,
        };

        assert_eq!(
            1,
            diesel::insert_into(notes::table)
                .values(&new_note)
                .execute(&conn)
                .expect("Could not insert note")
        );

        let retrieved_notes = notes::table
            .filter(notes::chord.eq("maj"))
            .load::<Note>(&conn)
            .expect("Could not retrieve note");

        assert_eq!(retrieved_notes.len(), 1);
        assert_eq!(retrieved_notes[0], new_note);
    }
}
