use axum::{Router, routing::get};
use rust_web_store_rahmananin::model::{
    server_queries::AppState,
    sql::handlers::{add_category, add_product, delete_category, delete_product_by_id},
    templates::{
        handlers::{category_page_handler, products_page_handler},
        index_handler,
    },
};
use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, SessionManagerLayer, cookie::time::Duration};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Настройки для логирования
    let subscrider = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .compact()
        .without_time()
        .finish();
    // Инициализация логирования.
    tracing::subscriber::set_global_default(subscrider).unwrap();

    // Создаем хранилище для сессий.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::days(7)));

    // Подключаем базу данных
    let options = sqlx::mysql::MySqlConnectOptions::new()
        .host("localhost")
        .username("root")
        .password("1234")
        .database("crud-shop");

    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    tracing::info!("Успешное подключение к базе данных {:#?}", pool);

    let app_state = AppState { db: pool };

    // Настройка web-приложения: роуты, раздача статики, мидлвары
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/categories", get(category_page_handler).post(add_category))
        .route("/categories/delete/{id}", get(delete_category))
        .route("/products", get(products_page_handler).post(add_product))
        .route("/products/delete/{id}", get(delete_product_by_id))
        // Раздаем статику. css / js / медиа. Когда в .html мы будет обращаться через тот же href
        // к роуту /static сервер поймет что нам нужно отдать все файлы из директории static
        .nest_service("/static", ServeDir::new("static"))
        // Добавляем сессии. Теперь если нуцжно будет что-то записать / прочитать
        // нужно в функции обработки добавить аргумент с типом 'Session'
        .layer(session_layer)
        .with_state(app_state);

    // Настйрока адреса и порта на котором будет запущен наш сервер
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();

    // Пишем сообщение в логи - есть несколько уровней логирования:
    // info - просто информация
    // warn - предупреждения, не критичное поведение но требующее анализа
    // error - ошибки
    // debug - более подробные логи
    // trace - низкроуровневые логи, показывает каждый шаг программы
    tracing::info!("Server has open on http://0.0.0.0:3030");

    // Запуск сервера с настройками которые мы записали в listener и app
    axum::serve(listener, app).await?;

    Ok(())
}
