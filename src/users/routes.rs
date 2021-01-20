use graphql_client::{GraphQLQuery, Response};
use tide::Request;
use bson::oid::ObjectId;

use crate::State;
use crate::util::common::{gql_uri, Tpl};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_users.graphql",
    response_derives = "Debug"
)]
struct AllUsers;

pub async fn user_index(_req: Request<State>) -> tide::Result {
    let project_index: Tpl = Tpl::new("user/index").await;

    // make data and render it
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6InNpZ25pbmdfa2V5In0.eyJlbWFpbCI6Im9rYTIyQGJ1ZHNob21lLmNvbSIsInVzZXJuYW1lIjoi5oiRMjJz6LCBMjRvazMyIiwiZXhwIjoxMDAwMDAwMDAwMH0.mw2OP6A6uW2W0hEHNk3C5Mq8QoAwT-xfaUXZmP0I9qEsaeO26ORZgRIFL3t1C0JtdTNfYoIFiMbBrPRY5nBjKg";
    let build_query = AllUsers::build_query(all_users::Variables { token: token.to_string() });
    let query = serde_json::json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");

    project_index.render(&resp_data).await
}
