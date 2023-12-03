#[cfg(test)]
pub mod test_controller {
    use std::{env, io, sync::Arc};

    use actix_web::test;
    use sqlx::MySqlPool;
    use serde_json::json;

    use crate::creat_app::create_app;
    use crate::domain::models::my_result::MyResult;

    #[actix_web::test]
    async fn test() {
        init_environment();
        let server_addr = env::var(SERVER_ADDR).expect(SERVER_ADDR_NOT_SET_MSG);
        let database_url = env::var(DATABASE_URL).expect(DATABASE_URL_NOT_SET_MSG);
        let pool = init_database(database_url).await;
        let app = test::init_service(create_app(pool.clone())).await;

        // no mock repo, need use real mysql
        let request_body = json!({
            "encryptCode": "asd"
        });
        let resp = test::TestRequest::post().uri(&format!("/")).set_json(&request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let pick_result: MyResult = test::read_body_json(resp).await;
        assert_eq!(pick_result.team_id, 9);
    }

    fn init_environment() {
        dotenvy::dotenv().ok();
        env_logger::init_from_env(env_logger::Env::new().default_filter_or(LOGGER_FILTER_LEVEL));
    }

    async fn init_database(database_url: String) -> Arc<MySqlPool> {
        let connection = MySqlPool::connect(&database_url).await.unwrap();
        Arc::new(connection)
    }

    const SERVER_ADDR: &str = "SERVER_ADDR";
    const DATABASE_URL: &str = "DATABASE_URL";
    const SERVER_ADDR_NOT_SET_MSG: &str = "SERVER_ADDR is not set in .env file";
    const DATABASE_URL_NOT_SET_MSG: &str = "DATABASE_URL must be set";
    const STARTING_SERVER_LOG: &str = "starting HTTP server at ";
    const LOGGER_FILTER_LEVEL: &str = "DEBUG";
}


