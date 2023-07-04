use std::{
  env,
  sync::{mpsc::channel, mpsc::Sender, Arc, Mutex},
  thread,
  time::Instant
};

use rand::{Rng, thread_rng};
use sha2::{Digest, Sha256};

fn run_thread(sender: Sender<(Vec<u8>, String)>, max_leading_bytes: Arc<Mutex<u32>>, byte_to_search_for: u8) {
  let mut rng = thread_rng();
  let mut previous_instant = Instant::now();

  loop {
    // Generate a random alphanumeric string of length 30
    let random_string: String = (0..30)
      .map(|_| {
        let rand_char = rng.gen_range(0..62);
        let character = match rand_char {
          0..=25 => (rand_char + 'a' as u8) as char,
          26..=51 => (rand_char - 26 + 'A' as u8) as char,
          _ => (rand_char - 52 + '0' as u8) as char,
        };
        character
      })
      .collect();

    let hash = Sha256::digest(random_string.as_bytes());

    let mut count = 0;
    for byte in hash.iter() {
      if *byte == byte_to_search_for {
        count += 1;
      } else {
        break;
      }
    }

    let mut max = max_leading_bytes.lock().unwrap();
    if count > *max {
      let elapsed_time = previous_instant.elapsed();
      *max = count;
      sender.send((hash.to_vec(), random_string)).unwrap();
      println!(
        "Time elapsed since last record: {:.2?}",
        elapsed_time
      );
      previous_instant = Instant::now();
    }
  }
}

fn main() {
  if env::args().len() < 3 {
    println!("Usage: {} <byte_to_search_for> <thread_amount>", env::args().nth(0).unwrap());
    return;
  }
  let byte_to_search_for = env::args().nth(1).unwrap().parse::<u8>().unwrap();
  let thread_amount = env::args().nth(2).unwrap().parse::<u32>().unwrap();

  println!("Thread amount: {}", thread_amount);

  let (sender, receiver) = channel();
  let max_leading_bytes = Arc::new(Mutex::new(0));
  let mut threads = Vec::new();
  for i in 0..thread_amount {
    println!("Starting thread {}", i);
    let new_sender = sender.clone();
    let max_leading_bytes = Arc::clone(&max_leading_bytes);
    threads.push(thread::spawn(move || {
        run_thread(new_sender, max_leading_bytes, byte_to_search_for);
    }));
  }

  // Get the results and print them
  loop {
    let new_result = receiver.recv();
    match new_result {
      Ok((hash, random_string)) => {
        let max = max_leading_bytes.lock().unwrap();
        println!(
          "New record! Count of leading {:02x} bytes: {}. Hash: {} Input string: {}",
          byte_to_search_for,
          max,
          hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>(),
          random_string
        );
      }
      Err(_) => {}
    }
  }
}