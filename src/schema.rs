table! {
    entries (id) {
        id -> Integer,
        title -> Nullable<Varchar>,
        question -> Varchar,
        answer -> Text,
        morning -> Bool,
        entry_date -> Datetime,
    }
}
