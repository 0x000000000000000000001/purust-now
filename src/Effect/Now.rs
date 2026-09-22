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

// Minimal `struct tm` for the C library's local-time entry point.
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct CTm {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    tm_wday: i32,
    tm_yday: i32,
    tm_isdst: i32,
    tm_gmtoff: i64,
    tm_zone: *const i8,
}

extern "C" {
    fn localtime_r(time: *const i64, tm: *mut CTm) -> *mut CTm;
}

pub fn Effect_Now_getTimezoneOffset() -> purust_core::UnknownType {
    purust_core::Value::Func1(purust_core::Func1::Static(|_| {
        // Like `new Date().getTimezoneOffset()`: minutes behind UTC.
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|elapsed| elapsed.as_secs() as i64)
            .unwrap_or(0);
        let mut tm = CTm::default();
        let offset = unsafe {
            if localtime_r(&seconds, &mut tm).is_null() {
                0.0
            } else {
                -(tm.tm_gmtoff as f64) / 60.0
            }
        };
        purust_core::mk_number(offset)
    }))
}
