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
    routes: BTreeMap<String, BTreeMap<String, Arc<dyn Handler>>>,
    not_found_handler: Arc<dyn Handler>,
    method_not_allowed_handler: Arc<dyn Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: BTreeMap::new(),
            not_found_handler: Arc::new(default_not_found_handler),
            method_not_allowed_handler: Arc::new(default_method_not_allowed_handler),
        }
    }

    pub fn add_handler<H: Handler>(&mut self, method: &str, path: &str, handler: H) {
        if !self.routes.contains_key(path) {
            self.routes.insert(path.to_string(), BTreeMap::new());
        }
        self.routes
            .get_mut(path)
            .unwrap()
            .insert(method.to_string(), Arc::new(handler));
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
        let handler = self.get_handler(&conn.method, &conn.path);

        std::thread::spawn(move || {
            if let Err(e) = handler.handle(&mut conn) {
                eprintln!("{}", e);
            }
        });
        Ok(())
    }

    fn get_handler(&self, method: &str, path: &str) -> Arc<dyn Handler> {
        if let Some(handlers) = self.routes.get(path) {
            if let Some(handler) = handlers.get(method) {
                handler.clone()
            } else {
                self.method_not_allowed_handler.clone()
            }
        } else {
            self.not_found_handler.clone()
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

fn default_not_found_handler(conn: &mut Connection) -> Result<()> {
    conn.write_status(404, "Not Found")
}

fn default_method_not_allowed_handler(conn: &mut Connection) -> Result<()> {
    conn.write_status(405, "Method Not Allowed")
}
