use std::collections::HashMap;

use redis::{ Commands, RedisResult };

pub fn get_apis(con: &mut redis::Connection) -> RedisResult<HashMap<String, String>> {
    let apis: HashMap<String, String> = con.hgetall("apis").unwrap_or_default();

    Ok(apis)
}
