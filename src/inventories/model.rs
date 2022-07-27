use crate::db;
use crate::error_handler::CustomError;
use crate::schema::{inventories, items};
use crate::items::Item;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::BelongingToDsl;

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Identifiable, Queryable)]
#[table_name = "inventories"]
pub struct Inventory {
    pub id: i32,
    pub name: String,
    pub item_count: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Inventories {
    pub id: i32,
    pub name: String,
    pub item_count: i32,
}

impl Inventories {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let inventories = inventories::table.load::<Inventories>(&conn)?;
        Ok(inventories)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let inventory = inventories::table.filter(inventories::id.eq(id)).first(&conn)?;
        Ok(inventory)
    }
    pub fn create(inventory: Inventory) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let inventory = Inventory::from(inventory);
        let inventory = diesel::insert_into(inventories::table)
            .values(inventory)
            .get_result(&conn)?;
        Ok(inventory)
    }
    pub fn update(id: i32, inventory: Inventory) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let inventory = diesel::update(inventories::table)
            .filter(inventories::id.eq(id))
            .set(inventory)
            .get_result(&conn)?;
        Ok(inventory)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(inventories::table.filter(inventories::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
    pub fn get_items(id: i32) -> Result<Vec<Item>, CustomError> {
        let conn = db::connection()?;
        let inventory: Inventories = inventories::table.filter(inventories::id.eq(id)).first(&conn)?;
        // let item_list = Item::belonging_to(&inventory)
        //     .load::<Item>(conn)
        //     .expect("Error loading items");
        let item_list = items::table.filter(items::inventory_id.eq(inventory.id)).load::<Item>(&conn)?;
        Ok(item_list)
    }
}
impl Inventory {
    fn from(inventory: Inventory) -> Inventory {
        Inventory {
            id: inventory.id,
            name: inventory.name,
            item_count: inventory.item_count,
        }
    }
}