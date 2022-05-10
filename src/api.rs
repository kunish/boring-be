use crate::{actions, models};
use actix_web::{delete, get, post, web, Error, HttpResponse};
use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use uuid::Uuid;

type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/post")]
pub async fn index(pool: web::Data<DBPool>) -> Result<HttpResponse, Error> {
    let posts = web::block(move || {
        let conn = pool.get()?;
        actions::get_all_posts(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
}

#[post("/post")]
pub async fn create(
    pool: web::Data<DBPool>,
    form: web::Json<models::NewPost>,
) -> Result<HttpResponse, Error> {
    let new_post = models::Post {
        id: Uuid::new_v4().to_string(),
        body: form.body.to_owned(),
        title: form.title.to_owned(),
        published: false,
    };

    let post = web::block(move || {
        let conn = pool.get()?;
        actions::insert_new_post(new_post, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
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
