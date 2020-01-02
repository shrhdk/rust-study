use std::io;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

use waf::http::{Connection, METHOD_GET};
use waf::Router;

fn main() -> io::Result<()> {
    let counter = AtomicUsize::new(1);

    let mut router = Router::new();

    router.add_handler(METHOD_GET, "/", move |conn: &mut Connection| {
        let _headers = conn.read_headers()?;

        // Create Body
        let count = counter.fetch_add(1, Ordering::SeqCst);
        let body = format!(
            r###"
            <!DOCTYPE html>
            <html lang="ja">
                <head>
                    <title>ほーむぺーじ</title>
                </head>
                <body>
                    <p>あなたは {} 人目の訪問者です。</p>
                </body>
            </html>
        "###,
            count
        );

        // Write Headers
        conn.write_status(200, "OK")?;
        conn.write_header("Content-Type", "text/html; charset=UTF-8")?;
        conn.write_header("Content-Length", &format!("{}", body.len()))?;
        conn.finish_header()?;

        // Write Body
        conn.write_all(body.as_bytes())?;

        Ok(())
    });

    router.add_handler(METHOD_GET, "/json", |conn: &mut Connection| {
        let _headers = conn.read_headers()?;

        // Create Body
        let body = "{\"message\":\"Hello, World!\"}\n";

        // Write Headers
        conn.write_status(200, "OK")?;
        conn.write_header("Content-Type", "application/json; charset=UTF-8")?;
        conn.write_header("Content-Length", &format!("{}", body.len()))?;
        conn.write_header("Connection", "close")?;
        conn.write_header("Server", "Example")?;
        conn.write_date_header()?;
        conn.finish_header()?;

        // Write Body
        conn.write_all(body.as_bytes())?;

        Ok(())
    });

    router.listen("localhost:8080")
}
