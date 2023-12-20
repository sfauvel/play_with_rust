


use std::net::SocketAddr;
use std::sync::mpsc as std_mpsc;
use std::thread;
use std::time::Duration;

use tokio::runtime;
use tokio::sync::oneshot;
use warp::reply::Reply;
use warp::Filter;

pub struct TestHttpServer {
    address: SocketAddr,
    panic_rx: std_mpsc::Receiver<()>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl TestHttpServer {
    /// Get the test server address
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// Get the server url
    pub fn url(&self) -> String {
        format!("http://{}", self.address)
    }
}

impl Drop for TestHttpServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        if !::std::thread::panicking() {
            self.panic_rx
                .recv_timeout(Duration::from_secs(3))
                .expect("test server should not panic");
        }
    }
}
/// Spawn a [TestHttpServer] using the given warp filters
pub fn test_http_server_with_socket_address<F>(
    filters: F,
    socket_addr: SocketAddr,
) -> TestHttpServer
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
{
    //Spawn new runtime in thread to prevent reactor execution context conflict
    thread::spawn(move || {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("new rt");
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (address, server) = rt.block_on(async move {
            warp::serve(filters).bind_with_graceful_shutdown(socket_addr, async {
                shutdown_rx.await.ok();
            })
        });

        let (panic_tx, panic_rx) = std_mpsc::channel();
        let thread_name = format!(
            "test({})-support-server",
            thread::current().name().unwrap_or("<unknown>")
        );
        thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                rt.block_on(server);
                let _ = panic_tx.send(());
            })
            .expect("thread spawn");

        TestHttpServer {
            address,
            panic_rx,
            shutdown_tx: Some(shutdown_tx),
        }
    })
    .join()
    .unwrap()
}
