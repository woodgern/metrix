table! {
    metrics (id) {
        id -> Int4,
        metric_name -> Varchar,
        data -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
