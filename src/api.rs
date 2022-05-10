use crate::{actions, models};
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use uuid::Uuid;

type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[put("/post")]
pub async fn create(
    pool: web::Data<DBPool>,
    form: web::Json<models::CreatePost>,
) -> Result<HttpResponse, Error> {
    let new_post = models::Post {
        id: Uuid::new_v4().to_string(),
        body: form.body.to_owned(),
        title: form.title.to_owned(),
        published: false,
    };

    let post = web::block(move || {
        let conn = pool.get()?;
        actions::create_post(new_post, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
}

#[get("/post")]
pub async fn retrieve(pool: web::Data<DBPool>) -> Result<HttpResponse, Error> {
    let posts = web::block(move || {
        let conn = pool.get()?;
        actions::retrieve_posts(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
}

#[post("/post/update-title/{post_id}")]
pub async fn update_title(
    pool: web::Data<DBPool>,
    form: web::Json<models::UpdatePostTitle>,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        actions::update_post_title(post_id.to_string(), form.title.to_owned(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/post/update-body/{post_id}")]
pub async fn update_body(
    pool: web::Data<DBPool>,
    form: web::Json<models::UpdatePostBody>,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        actions::update_post_body(post_id.to_string(), form.body.to_owned(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/post/publish/{post_id}")]
pub async fn publish(
    pool: web::Data<DBPool>,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        actions::set_published(post_id.to_string(), true, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/post/unpublish/{post_id}")]
pub async fn unpublish(
    pool: web::Data<DBPool>,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        actions::set_published(post_id.to_string(), false, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/post/{post_id}")]
pub async fn delete(
    pool: web::Data<DBPool>,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        actions::delete_post(post_id.to_string(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
