use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc, time::Duration};

type Registry = HashMap<String, Callback>;
struct Callback(Arc<dyn Fn() + Send + Sync>);

// lazy_static! {
//     REGISTY:

// struct ExecutionTimeRegistry {}

// impl ExecutionTimeRegistry {
//     pub fn new() -> Self {
//         Self {}
//     }

pub fn add_callback<N, F>(name: N, callback: F)
where
    N: Into<String>,
    F: FnMut(Duration) + Send + Sync,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn it_() {
        let mut total = Duration::from_secs(0);
        add_callback("foo", |duration| total += duration);

        let demo = Demo;
        demo.foo("bar");

        assert!(total > Duration::from_secs(0));
    }

    struct Demo;

    impl Demo {
        #[execution_time]
        fn foo(&self, _data: &str) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
