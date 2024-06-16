use axum::{
    routing::get,
    Router,
};
use askama_axum::Template;


#[tokio::main]
async fn main() {
    #[derive(Template)]
    #[template(path = "index.html")]
    struct IndexTemplate {
        strvar: String,
        num: i64,
    }
    // build our application with a single route
    let app = Router::new().route("/", get(get_index));

    async fn get_index() -> IndexTemplate {
        return IndexTemplate {
            strvar : String::from("safty and speeeed"),
            num: 2
        };
    }

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}