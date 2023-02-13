use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, AttributeValue, BillingMode, KeySchemaElement, KeyType,
    ScalarAttributeType,
};
use aws_sdk_dynamodb::Client;
use lambda_http::{run, Error};
use uuid::Uuid;
mod models;
use models::*;

use axum::{
    extract::Path,
    response::Json,
    routing::{get, post, put},
    Router,
};

async fn get_user(Path(uid): Path<String>) -> Json<Response<User, ErrorResponse>> {
    Json(match get_user_internal(uid).await {
        Ok(r) => Response::Result(r),
        Err(e) => Response::Error(ErrorResponse {
            message: format!("{:?}", e),
        }),
    })
}
async fn get_user_internal(uid: String) -> Result<User, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&config);
    let request = client
        .get_item()
        .table_name("users")
        .key("uid", AttributeValue::S(uid));
    let result = request.send().await?;
    let item = result.item().ok_or(anyhow::anyhow!("Not found"))?;
    let mut user_builder = UserBuilder::default();
    let first_name = item.get("first_name");
    if let Some(AttributeValue::S(first_name)) = first_name {
        user_builder.first_name(first_name.clone());
    }
    let last_name = item.get("last_name");
    if let Some(AttributeValue::S(last_name)) = last_name {
        user_builder.last_name(last_name.clone());
    }
    let user = user_builder.build()?;

    Ok(user)
}
async fn create_user(Json(user): Json<CreateUser>) -> Json<Response<Uuid, ErrorResponse>> {
    let uid = Uuid::new_v4();
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&config);
    let request = client
        .put_item()
        .table_name("users")
        .item("uid", AttributeValue::S(uid.to_string()))
        .item("first_name", AttributeValue::S(user.first_name))
        .item("last_name", AttributeValue::S(user.last_name));
    let result = request.send().await;

    Json(match result {
        Ok(_) => Response::Result(uid),
        Err(e) => Response::Error(ErrorResponse {
            message: format!("{:?}", e),
        }),
    })
}

async fn update_user(
    Path(uid): Path<String>,
    Json(update): Json<UpdateUser>,
) -> Json<Response<String, ErrorResponse>> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&config);
    let mut request = client
        .update_item()
        .table_name("users")
        .key("uid", AttributeValue::S(uid));
    if let Some(first_name) = update.first_name {
        request = request
            .update_expression("set first_name = :first_name")
            .expression_attribute_values(":first_name", AttributeValue::S(first_name));
    }
    if let Some(last_name) = update.last_name {
        request = request
            .update_expression("set last_name = :last_name")
            .expression_attribute_values(":last_name", AttributeValue::S(last_name));
    }
    Json(match request.send().await {
        Ok(_) => Response::Result("Updated".to_string()),
        Err(e) => Response::Error(ErrorResponse {
            message: format!("{:?}", e),
        }),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();
    create_table().await?;
    let app = Router::new()
        .route("/:uid", get(get_user))
        .route("/create", post(create_user))
        .route("/update", put(update_user));

    run(app).await
}

async fn create_table() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let key = "id";
    let pk = AttributeDefinition::builder()
        .attribute_name(key)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(key)
        .key_type(KeyType::Hash)
        .build();

    client
        .create_table()
        .table_name(String::from("todos"))
        .key_schema(ks)
        .attribute_definitions(pk)
        .billing_mode(BillingMode::PayPerRequest)
        .send()
        .await?;
    Ok(())
}
#[macro_use]
extern crate derive_builder;
