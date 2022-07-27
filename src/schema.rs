table! {
    inventories (id) {
        id -> Int4,
        name -> Varchar,
        item_count -> Int4,
    }
}

table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        quantity -> Int4,
        category -> Varchar,
        inventory_id -> Int4,
    }
}

joinable!(items -> inventories (inventory_id));

allow_tables_to_appear_in_same_query!(
    inventories,
    items,
);
