use crate::controller::auth_controller::{callback, login, logout, signup, validator};
use crate::controller::book_controller::{
    get_book_details, list_books, recent_books, top_rated_books, upload_book_info
};
use crate::controller::user_controller::{
    add_user, delete_user, get_user, get_user_list, user_count,
};
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/libre/v1/books")
            .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                validator,
            ))
            .route("/list", web::post().to(list_books))
            .route("/details/{book_id}", web::get().to(get_book_details))
            .route("/recent", web::get().to(recent_books))
            .route("/top-rated", web::get().to(top_rated_books))
            .route("/upload", web::post().to(upload_book_info))
    );

    cfg.service(
        web::scope("/api/libre/v1")
            .service(login)
            .service(signup)
            .service(callback)
            .service(logout),
    );

    cfg.service(
        web::scope("/api/libre/v1/users")
            .service(user_count)
            .service(get_user)
            .service(get_user_list)
            .service(delete_user)
            .service(add_user),
    );
}
