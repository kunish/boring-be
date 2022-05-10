use diesel::prelude::*;

use crate::models;

type DBError = Box<dyn std::error::Error + Send + Sync>;

pub fn create_post(
    new_post: models::Post,
    conn: &MysqlConnection,
) -> Result<models::Post, DBError> {
    use crate::schema::posts::dsl::*;

    diesel::insert_into(posts).values(&new_post).execute(conn)?;

    Ok(new_post)
}

pub fn retrieve_posts(conn: &MysqlConnection) -> Result<Vec<models::Post>, DBError> {
    use crate::schema::posts::dsl::*;

    let all_posts = posts.load::<models::Post>(conn)?;

    Ok(all_posts)
}

pub fn update_post_title(
    post_id: String,
    new_title: String,
    conn: &MysqlConnection,
) -> Result<(), DBError> {
    use crate::schema::posts::dsl::*;

    diesel::update(posts.find(post_id))
        .set(title.eq(new_title))
        .execute(conn)?;

    Ok(())
}

pub fn update_post_body(
    post_id: String,
    new_body: String,
    conn: &MysqlConnection,
) -> Result<(), DBError> {
    use crate::schema::posts::dsl::*;

    diesel::update(posts.find(post_id))
        .set(body.eq(new_body))
        .execute(conn)?;

    Ok(())
}

pub fn set_published(
    post_id: String,
    new_published: bool,
    conn: &MysqlConnection,
) -> Result<(), DBError> {
    use crate::schema::posts::dsl::*;

    diesel::update(posts.find(post_id))
        .set(published.eq(new_published))
        .execute(conn)?;

    Ok(())
}

pub fn delete_post(post_id: String, conn: &MysqlConnection) -> Result<(), DBError> {
    use crate::schema::posts::dsl::*;

    diesel::delete(posts.filter(id.eq(post_id))).execute(conn)?;

    Ok(())
}
