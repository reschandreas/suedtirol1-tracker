table! {
    logs (date) {
        date -> Timestamp,
        song -> Int4,
        is_new -> Bool,
    }
}

table! {
    songs (id) {
        id -> Int4,
        title -> Varchar,
        artist -> Varchar,
    }
}

joinable!(logs -> songs (song));

allow_tables_to_appear_in_same_query!(logs, songs,);
