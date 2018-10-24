use std::{borrow::Cow, collections::HashMap};

use http::{self, method::Method as HttpMethod};
use hyper::{self, rt::Future, service::service_fn_ok};

type HttpBody = hyper::Body;
type HttpRequest = hyper::Request<HttpBody>;
type HttpResponse = hyper::Response<HttpBody>;

type HttpUri = http::uri::Uri;
type HttpHeaders = http::HeaderMap;

pub fn run() -> Result<(), Box<::std::error::Error>> {
    let addr = ::cfg::hyper_server_addr().map_err(Box::new)?;
    let server = hyper::Server::bind(&addr)
        .serve(|| service_fn_ok(root_handler))
        .map_err(|e| eprintln!("server error: {}", e));
    Ok(hyper::rt::run(server))
}

fn root_handler(req: HttpRequest) -> HttpResponse {
    let (parts, body) = req.into_parts();

    match parts.method {
        HttpMethod::GET => route_get(parts.uri, parts.headers),
        HttpMethod::POST => route_post(parts.uri, parts.headers, body),
        _ => mk_not_found(),
    }
}

fn route_get(uri: HttpUri, _headers: HttpHeaders) -> HttpResponse {
    match uri.path() {
        "/hello" => HttpResponse::new(HttpBody::from("Hello World")),
        "/echo" => match parse_uri_query(&uri).get("text") {
            Some(v) => HttpResponse::new(HttpBody::from(v.to_string())),
            None => HttpResponse::new(HttpBody::from("")),
        },
        "/boom" => panic!(()),
        _ => mk_not_found(),
    }
}

fn route_post(uri: HttpUri, _headers: HttpHeaders, _body: HttpBody) -> HttpResponse {
    match uri.path() {
        "/incr" => HttpResponse::new(HttpBody::from(format!("{}", ::counter::incr()))),
        _ => mk_not_found(),
    }
}

fn parse_uri_query<'a>(uri: &'a HttpUri) -> HashMap<Cow<'a, str>, Cow<'a, str>> {
    let mut params = HashMap::new();
    if let Some(query) = uri.query() {
        params.extend(::url::form_urlencoded::parse(query.as_bytes()));
    }
    params
}

fn mk_not_found() -> HttpResponse {
    hyper::Response::builder()
        .status(404)
        .body(HttpBody::from(""))
        .unwrap()
}
