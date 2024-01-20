use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use askama_actix::Template;
use serde::Deserialize;
use sqlx::{prelude::FromRow, SqlitePool};

mod db;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "post_list.html")]
struct PostListTemplate {
    posts: Vec<Post>,
}

#[derive(Deserialize, FromRow)]
struct Post {
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = db::Database::connect().await.migrate().await.connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(fs::Files::new("/static", "assets"))
            .service(index)
            .service(create_post)
            .service(show_posts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    IndexTemplate {}
}

#[post("/posts")]
async fn create_post(db: web::Data<SqlitePool>, post: web::Form<Post>) -> impl Responder {
    let title = &post.title;
    let body = &post.body;

    sqlx::query("INSERT INTO posts (title, body) VALUES (?, ?)")
        .bind(title)
        .bind(body)
        .execute(&**db)
        .await
        .expect("Failed to insert post");

    HttpResponse::Created().finish()
}

#[get("/posts")]
async fn show_posts(db: web::Data<SqlitePool>) -> impl Responder {
    let post_results = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&**db)
        .await
        .expect("Failed to fetch posts");

    PostListTemplate {
        posts: post_results,
    }
}
