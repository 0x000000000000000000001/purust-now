use std::time::{SystemTime, UNIX_EPOCH};

fn purust_now_milliseconds(time: SystemTime) -> f64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(elapsed) => elapsed.as_millis() as f64,
        Err(error) => -(error.duration().as_millis() as f64),
    }
}

pub fn Effect_Now_now() -> purust_core::UnknownType {
    purust_core::Value::Func1(purust_core::Func1::Static(|_| {
        // Read the clock on every execution of the Effect, including replays.
        purust_core::mk_number(purust_now_milliseconds(SystemTime::now()))
    }))
}
