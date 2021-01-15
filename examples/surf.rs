use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Ip {
    ip: String
}

#[async_std::main]
async fn main() -> surf::Result<()> {
    let uri = "https://httpbin.org/post";
    let data = &Ip { ip: "129.0.0.1".into() };
    let res = surf::post(uri).body(surf::Body::from_json(data)?).await?;
    println!("{:?}", &res);
    assert_eq!(res.status(), 200);

    let uri = "https://api.ipify.org?format=json";
    let Ip { ip } = surf::get(uri).recv_json().await?;
    println!("{:?}", &ip);
    assert!(ip.len() > 10);

    Ok(())
}
