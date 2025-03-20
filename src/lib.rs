pub struct ThreadPool{
    size: usize,
}

impl ThreadPool {
    pub fn build(size: usize) -> ThreadPool {
        assert!(size > 0);
        println!("Membangun ThreadPool dengan {} thread!", size);
        ThreadPool { size }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("Menjalankan task di thread pool dengan {} thread", self.size); // log tambahan
        std::thread::spawn(f);
    }
}