use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / "world")
        .map(|| "Hello, world!");

    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}