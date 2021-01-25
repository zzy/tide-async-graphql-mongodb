use graphql_client::{GraphQLQuery, Response};
use tide::Request;

// use crate::State;
use crate::{
    State,
    util::common::{gql_uri, rhai_dir, Tpl},
};

type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_users.graphql",
    response_derives = "Debug"
)]
struct AllUsers;

pub async fn user_index(_req: Request<State>) -> tide::Result {
    let mut user_index: Tpl = Tpl::new("users/index").await;
    user_index
        .reg
        .register_script_helper_file(
            "length",
            format!("{}{}", rhai_dir().await, "length.rhai"),
        )
        .unwrap();

    // make data and render it
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6ImlvazJAYnVkc2hvbWUuY29tIiwidXNlcm5hbWUiOiLmiJHmmK9vazIiLCJleHAiOjEwMDAwMDAwMDAwfQ.Gk98TjaFPpyW2Vdunn-pVqSPizP_zzTr89psBTE6zzfLQStUnBEXA2k0yVrS0CHBt9bHLLcFgmo4zYiioRBzBg";
    let build_query = AllUsers::build_query(all_users::Variables {
        token: token.to_string(),
    });
    let query = serde_json::json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");

    user_index.render(&resp_data).await
}
