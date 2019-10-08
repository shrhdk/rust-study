use std::io;
use std::sync::{Arc, Mutex};

use waf::http::Connection;
use waf::Router;

fn main() -> io::Result<()> {
    let counter = Arc::new(Mutex::new(0usize));

    let mut router = Router::new();

    router.add_route("/", move |conn: &mut Connection| {
        if &conn.method != "GET" {
            conn.write_status(405, "Method Not Allowed")?;
            return Ok(());
        }

        let count = {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
            *counter
        };

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

        conn.write_status(200, "OK")?;
        conn.write_header("Content-Type", "text/html; charset=UTF-8")?;
        conn.write_header("Content-Length", &format!("{}", body.len()))?;
        conn.println("")?;
        conn.print(&body)?;
        Ok(())
    });
    router.listen("localhost:8080")
}
