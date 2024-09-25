use actix_web::{get, http::header::ContentType, HttpResponse, Responder};


#[get("/info")]
pub async fn info() -> impl Responder {
    let banner = "

    ";


    HttpResponse::Ok().body(banner)
}

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::html()).body("
        <html>

            <p>Willkommen auf dem Backend der NeptuneDB</p>
            <p>Wenn du auf der Suche nach der eigentlichen Website bist, dann klicke bitte <a href = \"https://leckere.aprikosenmarmela.de\">hier</a>.</p>
            <p><a href = \"/login\">Hier</a> kannst du dich alternativ auch direkt einloggen.</p>
        </html>
    ")
}



#[get("/invalidauth")]
pub async fn invalid_auth() -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::html()).body("<html><h1>Authentication isn't configured correctly. Please contact your respective Server-Admin</h1></html>'")
}

