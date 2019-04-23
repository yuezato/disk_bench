use std::time::SystemTime;

pub struct Timer {
    start: SystemTime,
    message: String,
}
impl Timer {
    pub fn new(message: &str) -> Self {
        println!("[{}] start", message);
        Timer {
            start: SystemTime::now(),
            message: message.to_owned(),
        }
    }

    fn secs_to_readable(_secs: u64) -> String {
        let mut secs = _secs;
        let mut result: String = String::from("");
        if secs > 60 * 60 {
            let hour = secs / (60 * 60);
            secs %= 60 * 60;
            result.push_str(&format!("{}h ", hour));
        }
        if secs > 60 {
            let minutes = secs / 60;
            secs %= 60;
            result.push_str(&format!("{}m ", minutes));
        }
        result.push_str(&format!("{}s", secs));
        result
    }
}
impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().expect("should be succeeded");
        println!(
            "[{}] finish @ {} {}ms",
            self.message,
            Timer::secs_to_readable(elapsed.as_secs()),
            elapsed.subsec_millis()
        );
    }
}
