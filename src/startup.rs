use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // 将连接包装在一个智能指针中
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // 为 POST /subscriptions 在请求路由表中添加一个条目
            .route("/subscriptions", web::post().to(subscribe))
            // 将连接注册为应用程序状态的一部分
            .app_data(connection.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}
