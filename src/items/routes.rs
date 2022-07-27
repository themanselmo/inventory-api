use crate::items::{Item, Items};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;


#[get("/items")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let items = web::block(|| Items::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(items))
}

#[get("/items/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let item = Items::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(item))
}

#[post("/items")]
async fn create(item: web::Json<Item>) -> Result<HttpResponse, CustomError> {
    let item = Items::create(item.into_inner())?;
    Ok(HttpResponse::Ok().json(item))
}

#[put("/items/{id}")]
async fn update(
    id: web::Path<i32>,
    item: web::Json<Item>,
) -> Result<HttpResponse, CustomError> {
    let item = Items::update(id.into_inner(), item.into_inner())?;
    Ok(HttpResponse::Ok().json(item))
}

#[delete("/items/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_inventory = Items::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_inventory })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}