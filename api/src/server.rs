use actix_web::{web, HttpRequest, HttpResponse};
use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Data, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

use crate::Database;
use crate::auth::actor::Actor;
use crate::model::room::Room;
use crate::model::user::User;
use crate::schema::{Mutation, Query, Subscription};
use crate::stream::StreamControl;
use musty::prelude::Backend;

#[derive(Clone)]
pub struct OrbtData {
    pub schema: Schema<Query, Mutation, Subscription>,
}

impl OrbtData {
    pub fn new(
        db: Database,
        stream_user_room_ctl: StreamControl<User, Room>,
    ) -> Self {
        Self {
            schema: Schema::build(
                Query::default(),
                Mutation::default(),
                Subscription::default(),
            )
            .data(db)
            .data(stream_user_room_ctl)
            .extension(ApolloTracing)
            .finish(),
        }
    }
}

pub async fn graphql_root(
    data: web::Data<OrbtData>,
    db: web::Data<Database>,
    http: HttpRequest,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut inner = req.into_inner();

    let actor = Actor::identify(&db, http).await;
    inner.data.insert(actor);

    data.schema.execute(inner).await.into()
}

pub async fn graphql_playground() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

pub async fn graphql_ws(
    orbt_data: web::Data<OrbtData>,
    db: web::Data<Database>,
    http: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(orbt_data.schema.clone())
        .on_connection_init(|value| on_connection_init(db, value))
        .start(&http, payload)
}

/*
{
    payload: { Authorization: "<token>"  }
}
*/
async fn on_connection_init(
    db: web::Data<Database>,
    value: serde_json::Value,
) -> async_graphql::Result<Data> {
    let Some(value) = value.as_object() else { return Err("connection_init payload must be an object".into()) };
    let Some(authorization) = value.get("Authorization") else { return Err("connection_init payload must have 'Authorization' key".into()) };
    let Some(token) = authorization.as_str() else { return Err("connection_init payload key 'Authorization' must have a string value".into()) };
    let actor = Actor::identify_with_token(&db, token).await;
    let mut data = Data::default();
    data.insert(actor);
    Ok(data)
}
