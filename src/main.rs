use dotenv::dotenv;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    dotenv().ok();

    let graphql_address = dotenv::var("GRAPHQL_ADDRESS").unwrap_or("0.0.0.0".to_owned());
    let graphql_port = dotenv::var("GRAPHQL_PORT").unwrap_or("8080".to_owned());

    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("Hello, world!") });

    app.listen(format!("{}:{}", graphql_address, graphql_port))
        .await?;

    Ok(())
}
