use crate::music_theory::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::*;

embed_migrations!("migrations/");

#[derive(Debug, PartialEq, Eq, Queryable, Insertable)]
#[table_name = "notes"]
pub struct ChordNote {
    pub chord: String,
    pub degree: Degree,
    pub interval: Interval,
}

impl ChordNote {
    pub fn note(chord: &str, degree: Degree, interval: Interval) -> ChordNote {
        ChordNote {
            chord: chord.to_string(),
            degree: degree,
            interval: interval,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Queryable, Insertable)]
#[table_name = "names"]
pub struct ChordName {
    pub chord: String,
    pub alternative_name: String,
}

impl ChordName {
    pub fn name(chord: &str, alternative_name: &str) -> ChordName {
        ChordName {
            chord: chord.to_string(),
            alternative_name: alternative_name.to_string(),
        }
    }
}

pub fn initialise_database() -> Result<SqliteConnection, Box<dyn std::error::Error>> {
    let connection = SqliteConnection::establish(":memory:")?;
    embedded_migrations::run(&connection)?;
    Ok(connection)
}

pub fn get_quality(name: &str, conn: &SqliteConnection) -> Option<Quality> {
    let quality = notes::table
        .filter(notes::chord.eq(name.trim()))
        .load::<ChordNote>(conn)
        .ok()
        .map(|ns| ns.into_iter().map(|n| (n.degree, n.interval)).collect());

    // If the query returns no notes, the chord may still be found in the
    // alternative name table.
    if quality == Some(vec![]) {
        None
    } else {
        quality
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        use degrees::*;
        use intervals::*;
        let conn = initialise_database().unwrap();

        let inserted_notes = vec![
            ChordNote {
                chord: "maj".to_string(),
                degree: III,
                interval: Maj3rd,
            },
            ChordNote {
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
        use degrees::*;
        use intervals::*;
        let conn = initialise_database().unwrap();

        let new_note = ChordNote {
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
            .load::<ChordNote>(&conn)
            .expect("Could not retrieve note");

        assert_eq!(retrieved_notes.len(), 1);
        assert_eq!(retrieved_notes[0], new_note);
    }
}
