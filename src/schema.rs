table! {
    names (chord) {
        chord -> Text,
        alternative_name -> Text,
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
