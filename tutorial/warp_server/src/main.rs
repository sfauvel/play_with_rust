use std::error::Error;

use warp::Filter;
mod test_http_server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log = warp::log::custom(|info| {  
        eprintln!(
            "{} {} {}",
            info.method(),
            info.path(),
            info.status(),
        );
    });
    
    let routes = warp::path!("hello" / String)
        .map(move |name| format!("Hello {}!", name))
        .with(log);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
    
}

// https://github.com/seanmonstar/warp/blob/master/examples/todos.rs