use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use lettris_sdk::{FromCustomRow, GoodProperty, GoodPropertyRow};
use sqlx::MySqlPool;

mod tracer;

#[tokio::main]
async fn main() {
    tracer::local_time_tracer();
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    let app = Router::new()
        .route("/user/list", get(list_users))
        // .route("/user/create", post(create_user))
        .with_state(pool);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8088));
    tracing::info!("application running on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_users(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<GoodProperty>>, StatusCode> {
    let properties = 
        sqlx::query_as!(GoodPropertyRow,
            "SELECT name, price, volumes, discount_type, discount_value FROM good_property"
        ).fetch_all(&pool)
        .await
        .unwrap();

    
    let properties = 
        properties.into_iter()
            .map(|row| GoodProperty::from_custom_row(row))
            .collect();
    Ok(Json(properties))
}
