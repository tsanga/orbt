use actix_web::{web, HttpResponse, HttpRequest};
use async_graphql::{Schema, Data};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

use crate::auth::actor::Actor;
use crate::model::room::Room;
use crate::model::user::User;
use crate::schema::{Query, Mutation, Subscription};
use crate::store::DataStore;
use crate::stream::StreamController;

#[derive(Clone)]
pub struct OrbtData {
    pub schema: Schema<Query, Mutation, Subscription>,
}

impl OrbtData {
    pub fn new(user_store: DataStore<User>, room_store: DataStore<Room>, stream_ctl: StreamController) -> Self {
        Self {
            schema: Schema::build(
                Query::default(),
                Mutation::default(),
                Subscription::default()
            )
            .data(user_store)
            .data(room_store)
            .data(stream_ctl)
            .finish()
        }
    }
}

pub async fn graphql_root(
    data: web::Data<OrbtData>,
    user_store: web::Data<DataStore<User>>,
    http: HttpRequest,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut inner = req.into_inner();

    let actor = Actor::identify(user_store, http);
    inner.data.insert(actor);

    data.schema.execute(inner).await.into()
}

pub async fn graphql_playground() -> actix_web::Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/")))
    )
}

pub async fn graphql_ws(
    orbt_data: web::Data<OrbtData>,
    user_store: web::Data<DataStore<User>>,
    http: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(orbt_data.schema.clone())
        .on_connection_init(|value| { on_connection_init(user_store, value) })
        .start(&http, payload)
}

/*
{
    payload: { Authorization: "<token>"  }
}
*/
async fn on_connection_init(user_store: web::Data<DataStore<User>>, value: serde_json::Value) -> async_graphql::Result<Data> {
    let Some(value) = value.as_object() else { return Err("connection_init payload must be an object".into()) };
    let Some(authorization) = value.get("Authorization") else { return Err("connection_init payload must have 'Authorization' key".into()) };
    let Some(token) = authorization.as_str() else { return Err("connection_init payload key 'Authorization' must have a string value".into()) };
    let actor = Actor::identify_with_token(user_store, token);
    let mut data = Data::default();
    data.insert(actor);
    Ok(data)
}