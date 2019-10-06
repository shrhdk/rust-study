use std::collections::HashMap;
use std::io::Result;
use std::net::{TcpListener, TcpStream};

use crate::http::Connection;

pub mod http;

pub struct Router {
    routes: HashMap<String, Box<dyn Fn(&mut Connection) -> Result<()>>>,
    not_found_handler: Box<dyn Fn(&mut Connection) -> Result<()>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
            not_found_handler: Box::new(not_found_handler),
        }
    }

    pub fn add_route<H>(&mut self, path: &str, handler: H)
    where
        H: Fn(&mut Connection) -> Result<()> + 'static,
    {
        self.routes.insert(path.to_string(), Box::new(handler));
    }

    pub fn serve(&mut self, addr: &str) -> Result<()> {
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
            .unwrap_or(&self.not_found_handler);
        handler(&mut conn)?;
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
