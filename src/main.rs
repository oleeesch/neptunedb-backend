use std::{fs, sync::Arc};

use actix_web::{web::{self}, App, HttpServer};
use storage::database::Database;
use tokio::sync::Mutex;

use crate::{services::{admin::{self}, display::{home, info, invalid_auth}, openidconnect, user::{self}}, structs::configuration::{Authorization, Configuration}};


mod storage;
mod structs;
mod services;

pub const TOKEN_VALID_LENGTH: u64 = 86400;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config_str = match fs::read_to_string("config.toml") {
        Ok(file) => file,
        Err(err) => {
            println!("please Populate the config.toml config!");
            let config_default = toml::to_string(&Configuration::default()).expect("Failed to Serialize Default Configuration!");
            fs::write("config.toml", config_default).expect("Failed to write config file!");
            return Result::Err(err)
        },
    };

    let configuration = toml::from_str::<Configuration>(&config_str).expect("Failed to deserialize Configuration.");

    let _ = fs::create_dir_all("neptunedb/");

    println!();

    println!("\n\nStarting API!\n");

    let movable_config = configuration.clone();//ToDo: Make this less strange...

    HttpServer::new(move || {
        let mov_config = movable_config.clone();
        let app = App::new()
            .app_data(web::Data::new(Arc::new(Mutex::new(Database::new(None)))))
            .app_data(web::Data::new(mov_config))
            .service(invalid_auth)
            .service(home)
            .service(info)
            .service(admin::save_protocol)
            .service(admin::create)
            .service(admin::add_admin)
            .service(admin::remove_admin)
            .service(admin::list_admins)
            .service(user::get_selection_identifiers)
            .service(user::search_for_protocol);


        match movable_config.authorization {
            Authorization::OpenIdConnect { .. } => {
                app
                    .service(openidconnect::login)
                    .service(openidconnect::redirect)
                    .service(openidconnect::finish)
            },
            Authorization::None => {
                app
            },
        }
    })
        .bind((configuration.api.bind_addr, configuration.api.bind_port))?
        .run()
        .await
}





#[macro_export]
macro_rules! expose_error {
    ($err:expr) => {
        return HttpResponse::InternalServerError().content_type(ContentType::json()).body("{\"error\":\"<E>\"}".to_string().replace("<E>", $err))
    };
}


#[macro_export]
macro_rules! invalid_input {
    ($err:expr) => {
        return HttpResponse::InternalServerError().content_type(ContentType::json()).body("{\"error\":\"<E>\"}".to_string().replace("<E>", $err))
    };
}
