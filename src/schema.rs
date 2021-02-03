table! {
    cast_votes (id) {
        id -> Text,
        user -> Text,
        poll -> Text,
        ranking -> Text,
        timestamp -> Text,
    }
}

table! {
    polls (id) {
        id -> Text,
        title -> Text,
        moderators -> Text,
        choices -> Text,
        timestamp -> Text,
    }
}

joinable!(cast_votes -> polls (poll));

allow_tables_to_appear_in_same_query!(
    cast_votes,
    polls,
);
