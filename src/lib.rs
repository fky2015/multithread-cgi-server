use std::error::Error;

pub struct ThreadPool;

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> Result<ThreadPool, String> {
        if size == 0 {
            Err("size must larger than 0.".to_string())
        } else {
            Ok(ThreadPool)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f()
    }
}
