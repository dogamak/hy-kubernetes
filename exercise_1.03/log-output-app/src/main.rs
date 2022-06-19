use rand::{distributions::Alphanumeric, Rng};
use std::{thread::sleep, time::Duration};

fn main() {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    loop {
        println!("{}", random_string);
        sleep(Duration::from_secs(5));
    }
}
