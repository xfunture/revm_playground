
use serde::{Deserialize,Serialize};
// use serde_json::Result;
use std::num::ParseIntError;
// std::result::Result::Ok;
use core::result::Result::Ok;

use tokio::task::yield_now;
use std::rc::Rc;


mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // 中间函数
    fn op_(x: f64, y: f64) -> MathResult {
        // 如果 `div` “失败” 了，那么返回 `DivisionByZero`
        let ratio = div(x, y)?;

        // 如果 `ln` “失败” 了，那么返回 `NegativeLogarithm`
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}",match why {
                MathError::NegativeLogarithm
                    => "logarithm of negative number",
                MathError::DivisionByZero
                    => "division by zero",
                MathError::NegativeSquareRoot
                    => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

pub fn test_question_mark_operator() {
    checked::op(2.0, 10.0);
}


#[derive(Serialize,Deserialize)]
struct Person{
    name:String,
    age:u8,
    phones:Vec<String>,
}


// 测试unwrap
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}


// 测试match
fn multiply_v1(first_number_str:&str,second_number_str: &str) -> Result<i32,ParseIntError>{
    match first_number_str.parse::<i32>(){
        Ok(first_number) => {
            match second_number_str.parse::<i32>(){
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}


// 测试Result map
fn multiply_v2(first_number_str: &str,second_number_str: &str) -> Result<i32,ParseIntError>{
    first_number_str.parse::<i32>().and_then(|first_number|{
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// 测试问号运算符
fn multiply_v3(first_number_str:&str,second_number_str:&str) -> Result<i32,ParseIntError>{
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}


fn print(result:Result<i32,ParseIntError>){
    match result{
        Ok(n) => println!("n is {}",n),
        Err(e) => println!("Error:{}",e),
    }
}

pub fn test_unwrap(){
    // let twenty = multiply("10", "2");
    // println!("double is {}", twenty);

    // let tt = multiply("t", "2");
    // println!("double is {}", tt);

    let twenty = multiply_v1("10", "2");
    print(twenty);

    let tt = multiply_v1("t", "2");
    print(tt);

    let twenty = multiply_v2("10", "2");
    print(twenty);

    let tt = multiply_v2("t", "2");
    print(tt);

    let twenty = multiply_v3("10", "2");
    print(twenty);

    let tt = multiply_v3("t", "2");
    print(tt);


}




