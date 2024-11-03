use std::time::Instant;

pub struct Timer {
    start: Instant,
}

impl Timer {
    /// 构造一个新的计时器，并立即开始计时
    pub fn new() -> Self {
        Timer {
            start: Instant::now(),
        }
    }

    /// 重置计时器，重新开始计时
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    /// 获取自计时器启动以来经过的时间，单位为秒，返回值为 f64
    pub fn elapsed(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}
