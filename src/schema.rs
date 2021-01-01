use async_std::sync::{Arc, RwLock};

use tide::http::mime;
use tide::{Request, Response, Body, StatusCode};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, receive_json};
use async_graphql::{Schema, Context, EmptySubscription};

use crate::constant::ENV;
use crate::dbs::mongo;

#[derive(Clone)]
struct User {
    id: Option<u16>,
    first_name: String,
}

#[async_graphql::Object]
impl User {
    async fn id(&self) -> i32 {
        self.id.unwrap_or(0) as i32
    }

    async fn first_name(&self) -> &str {
        &self.first_name
    }
}

#[derive(async_graphql::InputObject)]
struct NewUser {
    first_name: String,
}

impl NewUser {
    fn into_internal(self) -> User {
        User { id: None, first_name: self.first_name }
    }
}

#[derive(Default)]
pub struct Users(Arc<RwLock<Vec<User>>>);

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    /// Get all Users,
    async fn all_users1(&self) -> Vec<User> {
        let user1 = User { id: Some(12), first_name: "Alice".to_string() };
        let user2 = User { id: Some(22), first_name: "Jack".to_string() };
        let user3 = User { id: Some(32), first_name: "Tom".to_string() };

        vec![user1, user2, user3]
    }

    /// Get all Users
    async fn all_users2(&self, ctx: &Context<'_>) -> Vec<User> {
        let users = ctx.data_unchecked::<Users>().0.read().await;

        users.iter().cloned().collect()
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    /// Add new user
    async fn add_user(&self, ctx: &Context<'_>, user: NewUser) -> User {
        let mut users = ctx.data_unchecked::<Users>().0.write().await;
        let mut user = user.into_internal();

        user.id = Some((users.len() + 1) as u16);
        users.push(user.clone());

        user
    }
}

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    // let client = mongodb::Client::with_uri_str(ENV.get("MONGODB_URI").unwrap())
    //     .await
    //     .expect("Failed to initialize database!");
    // let db_budshome = client.database(ENV.get("DB_BUDSHOME").unwrap());
    let mongo_ds = mongo::ds().await;
    // let _db_budshome = mongo_ds.db_budshome;
    // let _db_yazhijia = mongo_ds.db_yazhijia;

    // let mut schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription)
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mongo_ds)
        .data(Users::default())
        .finish()
}

#[derive(Clone)]
pub struct State(pub Schema<QueryRoot, MutationRoot, EmptySubscription>);

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
