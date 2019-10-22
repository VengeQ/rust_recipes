use std::thread;
use std::time::Duration;
use std::thread::Thread;
use std::sync::{Arc, Mutex};

fn simple_threads_example() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Create thread");
    })
}

fn shared_data_thread(a_number: i32, a_vec: Vec<i32>) -> thread::JoinHandle<Vec<i32>> {
    thread::spawn(move || {
        for i in &a_vec {
            println!("{}", i)
        }
        a_vec
    })
}

fn parallel_map(data: Vec<Vec<i32>>, f: fn(i32) -> i32) -> Vec<thread::JoinHandle<Vec<i32>>> {
    data.into_iter()
        .map(|th| thread::spawn(move ||
            th.into_iter().map(|c|
                f(c)).collect()))
        .collect()
}

fn shared_date(data: Arc<Mutex<String>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for x in 0..10 {
            let d = data.lock().unwrap();
            //d.push(std::char::from_digit(x, 10).unwrap_or_else(|| '0'));
            println!("{:?}", d);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_threads_example_test() {
        for _ in 0..10 {
            simple_threads_example();
        }
    }

    #[test]
    fn shared_data_thread_test() {
        let a_vec = vec![1, 2, 3, 4, 5];

        shared_data_thread(0, a_vec);
    }

    #[test]
    fn parallel_map_test() {
        let data = vec![vec![1, 2, 3], vec![4, 4, 5], vec![6, 7, 7]];
        let results: Vec<i32> = parallel_map(data.clone(), |x| x + 1)
            .into_iter()
            .flat_map(|thread| thread.join().unwrap())
            .collect();

        let data: Vec<i32> = data.into_iter().flat_map(|e| e).map(|v| v + 1).collect();
        assert_eq!(results, data);
    }

    #[test]
    fn shared_data_arc_test() {
        let mutex = Arc::new(Mutex::new("hello".to_owned()));
        for i in 1..5 {
            shared_date(mutex.clone());
        }
        thread::sleep(Duration::from_secs(2));
        println!("{:?}",mutex);

    }
}