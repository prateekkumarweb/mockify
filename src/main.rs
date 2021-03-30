use std::path::PathBuf;

use tokio::{fs::File, io::AsyncReadExt};
use warp::{hyper::Method, path::Tail, reply, Filter, Rejection};

#[tokio::main]
async fn main() {
    let filter = warp::path::tail()
        .and(warp::method())
        .and_then(|tail: Tail, method: Method| async move {
            let mut path = PathBuf::new();
            path.push("server");
            path.push(tail.as_str());
            path.push(method.as_str().to_lowercase() + ".json");
            let mut file = File::open(path)
                .await
                .map_err(|_| warp::reject::not_found())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .await
                .map_err(|_| warp::reject::reject())?;
            Ok::<String, Rejection>(contents)
        })
        .map(|reply| reply::with_header(reply, "Content-Type", "application/json"));
    warp::serve(filter).run(([127, 0, 0, 1], 3030)).await;
}
