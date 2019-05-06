extern crate redis;
use redis::Commands;

fn main() {
    let res = redis_test().expect("redis_test failed.");
    println!("res = {}", res);
}

fn redis_test() -> redis::RedisResult<i32> {
    let cli = redis::Client::open("redis://:password@192.168.99.105:31150")?;
    let con = cli.get_connection()?;
    let _ = con.set("wow", 234)?;
    con.get("wow")
}
