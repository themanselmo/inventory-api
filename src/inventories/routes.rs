use crate::inventories::{Inventory, Inventories};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/inventories")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let inventories = web::block(|| Inventories::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(inventories))
}

#[get("/inventories/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let inventory = Inventories::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(inventory))
}

#[get("/inventory_items/{id}")]
async fn get_items(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let inventory_items = Inventories::get_items(id.into_inner())?;
    Ok(HttpResponse::Ok().json(inventory_items))
}

#[post("/inventories")]
async fn create(inventory: web::Json<Inventory>) -> Result<HttpResponse, CustomError> {
    let inventory = Inventories::create(inventory.into_inner())?;
    Ok(HttpResponse::Ok().json(inventory))
}

#[put("/inventories/{id}")]
async fn update(
    id: web::Path<i32>,
    inventory: web::Json<Inventory>,
) -> Result<HttpResponse, CustomError> {
    let inventory = Inventories::update(id.into_inner(), inventory.into_inner())?;
    Ok(HttpResponse::Ok().json(inventory))
}

#[delete("/inventories/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_inventory = Inventories::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_inventory })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(get_items);
    config.service(create);
    config.service(update);
    config.service(delete);
}