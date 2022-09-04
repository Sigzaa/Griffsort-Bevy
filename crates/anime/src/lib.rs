use async_std::task;
use std::{thread, time};

pub async fn anime(time: i64){
    for i in 1..time{
    let _: i64 = 61829851 * 911521234 + i; 
    }
    println!("hey async {time}");
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
