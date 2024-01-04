

// use anyhow::{Ok,Result};
use fern::colors::{Color,ColoredLevelConfig};
use log::LevelFilter;
use revm_playground::trace::mempool_watching;


use tokio::net::TcpListener;
use revm_playground::test::test_question_mark_operator;
use revm_playground::test::test_unwrap;
use revm_playground::single_thread_web::test_thread_pool_web;


// mini_redis
use mini_redis::{client,Result};

//test redis
use revm_playground::redis::test_redis;

// pub fn setup_logger() -> Result<()>{
//     let colors = ColoredLevelConfig{
//         trace:Color::Cyan,
//         debug:Color::Magenta,
//         info: Color::Green,
//         warn: Color::Red,
//         error: Color::BrightRed,
//         ..ColoredLevelConfig::new()
//     };
//     fern::Dispatch::new()
//         .format(move |out, message, record| {
//             out.finish(format_args!(
//                 "{}[{}] {}",
//                 chrono::Local::now().format("[%H:%M:%S]"),
//                 colors.color(record.level()),
//                 message
//             ))
//         })
//         .chain(std::io::stdout())
//         .level(log::LevelFilter::Error)
//         .level_for("revm_playground", LevelFilter::Info)
//         .apply()?;

//     Ok(())
// }


#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // setup_logger()?;

    // 测试问号运算符
    // test_question_mark_operator();

    // 测试unwrap,问号运算符，Result<T,Err>
    // test_unwrap();

    // let weth = String::from("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    // mempool_watching(weth).await?;
 
    //测试异步框架tokio

    //测试线程池服务器
    // test_thread_pool_web();

    //启动redis服务端
    test_redis().await;

    Ok(())
}