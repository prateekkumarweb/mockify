use std::path::PathBuf;

use structopt::StructOpt;
use tokio::{fs::File, io::AsyncReadExt};
use warp::{hyper::Method, path::Tail, reply, Filter, Rejection};

#[derive(StructOpt, Debug, Clone)]
struct Opt {
    /// Path to the folder containing the responses
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// Port to run on
    #[structopt(long, short, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let path = opt.path;
    let filter = warp::path::tail()
        .and(warp::method())
        .and(warp::any().map(move || path.clone()))
        .and_then(|tail: Tail, method: Method, mut path: PathBuf| async move {
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
    println!("Server starting on port {}", opt.port);
    warp::serve(filter).run(([0, 0, 0, 0], opt.port)).await;
}
