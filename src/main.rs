extern crate core;

use env_logger::*;
use log::*;
use ::redis::{Commands, Connection, RedisResult};
use rand::Rng;
use crate::redis::cmd::PoolCommand;
use crate::redis::pool::HPool;

mod redis;
mod signaling;
mod server;

fn redis_set_thingg(key: &str, xconn: &mut Connection) {
    let mut rng = rand::thread_rng();
    let value: u32 = rng.gen();
    let _: () = xconn.zadd(key, value, 0u32).unwrap();
}

fn redis_get_thingg(key: &str, xconn: &mut Connection) {
    let zv: RedisResult<Vec<u32>> = xconn.zrange(key, 0, 1000);
    match zv {
        Ok(v) => info!("{}={:?}", key, v),
        Err(e) => error!("Error {}", e)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if let Some(hpool) = HPool::new("redis://192.168.0.100:49153", 10) {
        let mut cmd = PoolCommand::new(Box::new(hpool), "mycommand");
        cmd.execute(|xconn| {
            redis_set_thingg("myhhkeyymmmmhh", xconn.conn.as_mut());
            redis_get_thingg("myhhkeyymmmmhh", xconn.conn.as_mut());
        });
    } else {
        println!("Could not create redis pool")
    }

    server::start().await
}
