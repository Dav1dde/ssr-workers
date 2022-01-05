use worker::{kv::KvStore, Request, Response};

pub async fn serve_asset(req: Request, store: KvStore) -> worker::Result<Response> {
    let path = req.path();
    let path = path.trim_start_matches('/');
    let value = match store.get(path).bytes().await? {
        Some(value) => value,
        None => return Response::error("Not Found", 404),
    };
    let mut response = Response::from_bytes(value)?;
    response.headers_mut().set("Content-Type", get_mime(path))?;
    Ok(response)
}

fn get_mime(path: &str) -> &'static str {
    let ext = if let Some((_, ext)) = path.rsplit_once(".") {
        ext
    } else {
        ""
    };

    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "ico" => "image/x-icon",
        "wasm" => "application/wasm",
        _ => "text/plain",
    }
}
