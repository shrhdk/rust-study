use std::collections::BTreeMap;
use std::io::Result;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use crate::http::Connection;

pub mod http;

pub trait Handler: Send + Sync + 'static {
    fn handle(&self, conn: &mut Connection) -> Result<()>;
}

impl<F> Handler for F
where
    F: Send + Sync + Fn(&mut Connection) -> Result<()> + 'static,
{
    fn handle(&self, conn: &mut Connection) -> Result<()> {
        (*self)(conn)
    }
}

pub struct Router {
    routes: BTreeMap<String, Arc<dyn Handler>>,
    not_found_handler: Arc<dyn Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: BTreeMap::new(),
            not_found_handler: Arc::new(not_found_handler),
        }
    }

    pub fn add_route<H: Handler>(&mut self, path: &str, handler: H) {
        self.routes.insert(path.to_string(), Arc::new(handler));
    }

    pub fn listen(&mut self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            self.handle_client(stream?)?;
        }
        Ok(())
    }

    fn handle_client(&self, stream: TcpStream) -> Result<()> {
        let mut conn = Connection::new(stream)?;
        let handler = self
            .routes
            .get(&conn.path)
            .unwrap_or(&self.not_found_handler)
            .clone();
        std::thread::spawn(move || {
            if let Err(e) = handler.handle(&mut conn) {
                eprintln!("{}", e);
            }
        });
        Ok(())
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

fn not_found_handler(conn: &mut Connection) -> Result<()> {
    conn.write_status(404, "Not Found")
}
