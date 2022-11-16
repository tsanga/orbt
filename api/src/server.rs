use actix_web::{web, HttpResponse};
use async_graphql::{Schema, EmptySubscription};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::schema::{Query, Mutation};
use crate::store::DataStore;

#[derive(Clone)]
pub struct OrbtData {
    pub schema: Schema<Query, Mutation, EmptySubscription>,
}

impl OrbtData {
    pub fn new(data_store: DataStore) -> Self {
        Self {
            schema: Schema::build(
                Query::default(),
                Mutation::default(),
                EmptySubscription
            )
            .data(data_store)
            .finish()
        }
    }
}

pub async fn graphql_root(
    data: web::Data<OrbtData>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let inner = req.into_inner();
    data.schema.execute(inner).await.into()
}

pub async fn graphql_playground() -> actix_web::Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/")))
    )
}