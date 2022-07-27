use crate::db;
use crate::error_handler::CustomError;
use crate::schema::items;
use crate::inventories::Inventory;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Serialize, Deserialize,  Insertable)]
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Inventory, foreign_key: "inventory_id")]
#[table_name = "items"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub quantity: i32,
    pub category: String,
    pub inventory_id: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Items {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub quantity: i32,
    pub category: String,
    pub inventory_id: i32,
}

impl Items {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let items = items::table.load::<Items>(&conn)?;
        Ok(items)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let item = items::table.filter(items::id.eq(id)).first(&conn)?;
        Ok(item)
    }
    pub fn create(item: Item) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let item = Item::from(item);
        let item = diesel::insert_into(items::table)
            .values(item)
            .get_result(&conn)?;
        Ok(item)
    }
    pub fn update(id: i32, item: Item) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let item = diesel::update(items::table)
            .filter(items::id.eq(id))
            .set(item)
            .get_result(&conn)?;
        Ok(item)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(items::table.filter(items::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Item {
    fn from(item: Item) -> Item {
        Item {
            id: item.id,
            name: item.name,
            description: item.description,
            quantity: item.quantity,
            category: item.category,
            inventory_id: item.inventory_id
        }
    }
}