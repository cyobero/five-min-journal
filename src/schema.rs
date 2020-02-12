table! {
    entries (id) {
        id -> Integer,
        title -> Nullable<Varchar>,
        question -> Enum,
        answer -> Enum,
        entry_date -> Nullable<Datetime>,
    }
}
