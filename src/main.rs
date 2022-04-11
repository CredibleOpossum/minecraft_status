use minecraft_query::get_server_json;
use rocket::get;
use rocket::http::Status;
use rocket::response::content;
use rocket::routes;

const MAIN_HTML: &str = include_str!("../main/index.html");
const MAIN_JS: &str = include_str!("../main/index.js");
const MAIN_CSS: &str = include_str!("../main/style.css");
const FAVICON: &[u8; 100071] = include_bytes!("../main/favicon.ico");

#[get("/get/<hostname>/<port>")]
async fn query(hostname: String, port: u16) -> Result<String, Status> {
    let json =
        tokio::task::spawn_blocking(move || match get_server_json(hostname.as_str(), port) {
            Ok(json) => Some(json),
            Err(_) => None,
        })
        .await
        .unwrap();
    if json.is_some() {
        return Ok(json.unwrap());
    }
    Err(Status::BadRequest)
}

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(MAIN_HTML)
}

#[get("/index.js")]
fn main_js() -> content::JavaScript<&'static str> {
    content::JavaScript(MAIN_JS)
}

#[get("/style.css")]
fn main_style() -> content::Css<&'static str> {
    content::Css(MAIN_CSS)
}

#[get("/favicon.ico")]
fn favicon() -> Vec<u8> {
    FAVICON.to_vec()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![query, index, main_js, main_style, favicon])
        .launch()
        .await
}
