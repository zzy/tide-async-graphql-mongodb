#[async_std::main]
async fn main() -> surf::Result<()> {
    let uri = "https://budshome.com";
    
    let string: String = surf::get(uri).recv_string().await?;
    println!("{}", string);

    Ok(())
}
