use tokio::net::{TcpListener,TcpStream};
use tokio::task;
use mini_redis::{Connection,Frame};






pub async fn test_redis(){
    //Bind the listener to the address
    //监听指定地址，等待TCP连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop{
        //第二个被忽略的项中包含新连接的“IP” 和端口信息
        let (socket,_) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }

}



async fn process(socket:TcpStream){
    use mini_redis::Command::{self,Get,Set};
    use std::collections::HashMap;

    // 使用hashmap来存储redis的数据
    let mut db = HashMap::new();

    //Connection 对于redis的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame（数据帧=redis命令+数据），而不是字节流
    //`mini-redis` 提供的便利函数，使用返回的 `connection` 可以用于从 socket 中读取数据并解析为数据帧
    let mut connection: Connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap(){
        println!("Got {:?}",frame);

        let response = match Command::from_frame(frame).unwrap(){
            Set(cmd) => {
                db.insert(cmd.key().to_string(),cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()){
                    Frame::Bulk(value.clone().into())                    
                }else{
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}",cmd),
        };


        //将请求响应返回客户端
        connection.write_frame(&response).await.unwrap();

    }
}