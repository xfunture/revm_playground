
// mini_redis
use mini_redis::{client,Result};




#[tokio::main]
async fn main() -> Result<()> {

    // 建立与mini-redis服务器的连接
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // println!("client:{:#?}",client);
    // 设置 key: "hello" 和 值: "world"
    let result = client.set("hello", "world".into()).await.unwrap();
    println!("result:{:?}",result);


    // 获取"key=hello"的值
    let result = client.get("hello").await.unwrap();

    println!("从服务器端获取到结果={:?}", result);


    Ok(())
}