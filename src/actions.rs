use diesel::prelude::*;

use crate::models;

type DBError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all_posts(conn: &MysqlConnection) -> Result<Vec<models::Post>, DBError> {
    use crate::schema::posts::dsl::*;
    let all_posts = posts.load::<models::Post>(conn)?;

    Ok(all_posts)
}

pub fn insert_new_post(
    new_post: models::Post,
    conn: &MysqlConnection,
) -> Result<models::Post, DBError> {
    use crate::schema::posts::dsl::*;

    diesel::insert_into(posts).values(&new_post).execute(conn)?;

    Ok(new_post)
}

pub fn delete_post(post_id: String, conn: &MysqlConnection) -> Result<(), DBError> {
    use crate::schema::posts::dsl::*;

    diesel::delete(posts.filter(id.eq(post_id))).execute(conn)?;

    Ok(())
}
