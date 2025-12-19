#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // 1. โหลด Environment Variables จากไฟล์ .env เป็นสิ่งแรก
    // เพื่อให้ api.rs สามารถอ่านค่า GOOGLE_API_URL ได้ทันที
    use dotenvy::dotenv;
    dotenv().ok();

    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    // Import App และ shell จาก module app
    use rust_herb::app::*;

    // โหลด Configuration ของ Leptos
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // สร้าง Route List จาก Component หลัก (App)
    let routes = generate_route_list(App);

    // สร้าง Router ของ Axum
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // เริ่มต้น Server
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
}
