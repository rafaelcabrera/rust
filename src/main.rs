#[macro_use]
extern crate diesel;

use dotenvy::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;
pub mod models;

fn main() {

    use self::models::{Post, NewPost};
    use self::schema::posts;
    use self::schema::posts::dsl::*; //todas las funciones internas de dsl

    

    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("de_url variable not found");

    let conn = PgConnection::establish(&db_url).expect("Failed to establish");

    let new_post = NewPost{
        title: "Mi primer blogpost",
        body: "lorem ipsum dolor sit amet",
        slug: "primer-post",
    };
    let post: Post = diesel::insert_into(posts::table).values(new_post).get_result(&conn).expect("Failed to insert");
    //Select * FROM posts
    let posts_result = posts.load::<Post>(&conn).expect("Failed to load posts");

    for post in posts_result {
        println!("{}", post.title);
    }
}
