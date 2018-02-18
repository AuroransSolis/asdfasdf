extern crate asdfasdf;
use std::thread;
use std::sync::{Arc, Mutex};

use asdfasdf::TripFetcher;

fn main() {
    let fetcher = TripFetcher::new(32_768, (0, 0, 0));
    let fetcher = Arc::new(Mutex::new(fetcher));
    let mut threads = vec![];
    for no in 0..3 {
        let fetcher = Arc::clone(&fetcher);
        let request_len = 1000;
        let checker = thread::spawn(move || {
            loop {
                let mut m_fetcher = fetcher.try_lock();
                let mut data = Vec::new();
                match m_fetcher {
                    Ok(ref mut t_fetcher) => {
                        if !t_fetcher.active {
                            break;
                        }
                        let trips = t_fetcher.get_triplets_vec(request_len);
                        if let Err(tferr) = trips {
                            println!("{}", tferr);
                            continue;
                        }
                        data = trips.unwrap();
                        println!("Thread {} got {} trips | Start: {:?}, end: {:?}", no, data.len(), data[0], data[data.len() - 1]);
                    },
                    _ => continue
                }
                drop(m_fetcher);
                for trip in data.into_iter() {
                    if asdfasdf::test_squares(trip) {
                        println!("Hory shet! Solution: {:?}", trip);
                    }
                }
            }
            println!("Thread {} finished execution.", no);
        });
        threads.push(checker);
    }
    for a in threads {
        a.join().unwrap();
    }
    println!("Done.");
}