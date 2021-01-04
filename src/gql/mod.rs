pub mod queries;
pub mod mutations;

use crate::State;

use tide::http::mime;
use tide::{Request, Response, Body, StatusCode};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, receive_json};
use async_graphql::{Schema, EmptySubscription};

use crate::constant::ENV;
use crate::dbs::mongo;

use crate::gql::queries::QueryRoot;
use crate::gql::mutations::MutationRoot;

use crate::users::services::Users;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    // get mongodb datasource. It can be added to:
    // 1. As global data for async-graphql.
    // 2. As application scope state of Tide
    // 3. In product, I recommend lazy-static.rs.
    let mongo_ds = mongo::DataSource::init().await;

    // The root object for the query and Mutatio, and use EmptySubscription.
    // Add global mongodb datasource  in the schema object.
    // let mut schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription)
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mongo_ds)
        .data(Users::default())
        .finish()
}

pub async fn graphql(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let schema = req.state().0.clone();
    let gql_resp = schema.execute(receive_json(req).await?).await;

    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(Body::from_json(&gql_resp)?);

    Ok(resp)
}

pub async fn graphiql(_: Request<State>) -> tide::Result<impl Into<Response>> {
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(playground_source(GraphQLPlaygroundConfig::new(
        ENV.get("GRAPHQL_PATH").unwrap(),
    )));
    resp.set_content_type(mime::HTML);

    Ok(resp)
}
