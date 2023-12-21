mod test_http_server;


#[cfg(test)]
mod tests {
    async fn http_request(url: String, route: &str) -> String {
        let result = reqwest::get(format!("{}/{}", url, route))
            .await
            .expect("should run")
            .text()
            .await
            .unwrap();
        result
    }

    async fn http_request_against_server<F>(routes: F, endpoint: &str) -> String
        where
            F: Filter + Clone + Send + Sync + 'static,
            F::Extract: Reply,
    {
        let server = test_http_server_with_socket_address(routes, ([127, 0, 0, 1], 3030).into());

        http_request(server.url(), endpoint).await
    }

    use std::{convert::Infallible, sync::Arc, io::{Read, Write, Error}, fs::File};

    use crate::test_http_server::test_http_server_with_socket_address;
    use serial_test::serial;
    use tokio::sync::Mutex;
    use warp::{Filter, reply::Reply, filters::path::FullPath};

    #[serial] // Execute each test in a serial mode to kill server at the end of each tests.
    #[tokio::test] // To be able to call async function in test
    async fn simple_http_example() {
        let routes = warp::path!("hello" / String)
            .map(move |name| format!("Hello {}!", name));

        // It's equivalent to the line below but in a separate thread to not block the execution.
        // warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
        let server = test_http_server_with_socket_address(routes, ([127, 0, 0, 1], 3030).into());

        let result = http_request(server.url(), "hello/bob").await;
        assert_eq!("Hello bob!", result);
    }

    #[serial]
    #[tokio::test]
    async fn http_route_with_map() {
        let routes = warp::path!("hello" / String)
            .map(move |name| format!("Hello {}!", name));

        let result = http_request_against_server(routes, "hello/bob").await;
        assert_eq!("Hello bob!", result);
    }

    #[serial]
    #[tokio::test]
    async fn http_route_with_and_then() {
        async fn say_hello(name: String) -> Result<impl warp::Reply, Infallible> {
            Ok(format!("Hi {}!", name))
        }

        let routes = warp::path!("hello" / String)
            .and_then(say_hello);

        let result = http_request_against_server(routes, "hello/bob").await;
        assert_eq!("Hi bob!", result);
    }

    type Database = Arc<Mutex<Vec<String>>>;
    async fn add_someone(name: String, database: Database) -> Result<impl warp::Reply, Infallible> {
        database.lock().await.push(name.clone());
        Ok(format!("Welcome {}!", name))
    }

    #[serial]
    #[tokio::test]
    async fn http_route_with_a_mutable_object_clone_in_a_method() {
        let database = Arc::new(Mutex::new(Vec::<String>::new()));

        // Use this method to clone database object outside of the map and then move it.
        fn with_parameter(database: Database) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
            warp::any().map(move || database.clone() )
        }

        let routes = warp::path!("add" / String)
            .and(with_parameter(database.clone()))
            .and_then(add_someone);

        let server = test_http_server_with_socket_address(routes, ([127, 0, 0, 1], 3030).into());

        assert_can_add_someone(database, server.url()).await;
    }

    #[serial]
    #[tokio::test]
    async fn http_route_with_a_mutable_object_used_directly_in_and() {
        let database = Arc::new(Mutex::new(Vec::<String>::new()));


        // To clone object directly in `and`, we need to clone it before.
        let database_to_borrow = database.clone();
        let routes = warp::path!("add" / String)
            .and(warp::any().map(move || database_to_borrow.clone() ))
            .and_then(add_someone);

        let server = test_http_server_with_socket_address(routes, ([127, 0, 0, 1], 3030).into());

        assert_can_add_someone(database, server.url()).await;
    }

    #[serial]
    #[tokio::test]
    async fn http_route_with_a_mutable_object_used_directly_in_and_xxx() {
        let database = Arc::new(Mutex::new(Vec::<String>::new()));

        let routes = warp::path!("add" / String)
            .and({
                // To clone object directly in `and`, we need to clone it before.
                let database = database.clone();
                warp::any().map(move || database.clone() )
            })
            .and_then(add_someone);

        let server = test_http_server_with_socket_address(routes, ([127, 0, 0, 1], 3030).into());

        assert_can_add_someone(database, server.url()).await;
    }

    async fn assert_can_add_someone(database: Database, base_url: String) {
        assert_eq!(0, database.lock().await.len());
        let result = http_request(base_url.clone(), "add/bob").await;
        assert_eq!("Welcome bob!", result);
        assert_eq!(1, database.lock().await.len());
        assert_eq!("bob", database.lock().await[0]);

        let result = http_request(base_url.clone(), "add/john").await;
        assert_eq!("Welcome john!", result);
        assert_eq!(2, database.lock().await.len());
        assert_eq!("bob", database.lock().await[0]);
        assert_eq!("john", database.lock().await[1]);
    }


    #[serial]
    #[tokio::test]
    async fn http_route_using_with_instruction() -> Result<(), Error> {
        const OUTPUT_FILE: &str = "target/tmp.txt";
        File::create(OUTPUT_FILE)?;

        let log = warp::log::custom(|info| {
           let mut file = File::options().append(true).open(OUTPUT_FILE).unwrap();
           let output = format!("{} {} {}\n", info.method(), info.path(), info.status(),);
           let _ = file.write_all(output.as_bytes());
        });

        let routes = warp::path!("hello" / String)
            .map(move |name| format!("Hello {}!", name))
            .with(log);

        let result = http_request_against_server(routes, "hello/bob").await;
        assert_eq!("Hello bob!", result);
        let result = http_request_against_server(routes, "hello/john").await;
        assert_eq!("Hello john!", result);
       
       
        let mut file = File::open(OUTPUT_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!("GET /hello/bob 200 OK\nGET /hello/john 200 OK\n", contents);

        Ok(())
    }


    #[serial]
    #[tokio::test]
    async fn http_route_intercept_fullpath() {
        async fn say_hello(name: String, path: FullPath) -> Result<impl warp::Reply, Infallible> {
            Ok(format!("Hi {} from {}!", name, path.as_str()))
        }

        let routes = warp::path!("hello" / String)
            .and(warp::path::full().map(move |path: FullPath| path))
            .and_then(say_hello);

        let result = http_request_against_server(routes, "hello/bob").await;
        assert_eq!("Hi bob from /hello/bob!", result);
    }



}