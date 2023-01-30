use std::cmp::min;
use std::collections::HashMap;
use std::iter::Iterator;
use std::sync::mpsc::channel;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut ans: HashMap<char, usize> = HashMap::new();
    if worker_count == 0 {
        return ans;
    }

    // Split the input into chunks for all workers -> best to do so by chars
    let flattened_input: String = input
        .iter()
        .flat_map(|c| c.chars())
        .filter(|c| c.is_alphabetic())
        .collect();
    if flattened_input.is_empty() {
        return ans;
    }
    let input_size = flattened_input.len();
    let mut worker_chunk = input_size / worker_count;

    // If chunk is integer divisible by worker_count we need to increase the chunk size so we don't
    // miss any char at the end due to rounding error
    if input_size % worker_count != 0 {
        worker_chunk += 1;
    }

    // We create this block in order to drop the sender (tx) and have the channel close, otherwise the
    // program hangs, since channels close when the last sender is dropped
    let rx = {
        let (tx, rx) = channel();
        // Create worker threads
        for i in 0..worker_count {
            let start = i * worker_chunk;
            let end = min((i + 1) * worker_chunk, input_size);
            let chunk = flattened_input.get(start..end);
            if let Some(content) = chunk {
                let content = content.to_lowercase();
                let tx = tx.clone();
                thread::spawn(move || {
                    let temp_map: HashMap<char, usize> =
                        content.chars().fold(HashMap::new(), |mut m, c| {
                            *m.entry(c).or_insert(0) += 1;
                            m
                        });
                    tx.send(temp_map).unwrap();
                });
            }
        }
        rx
    };

    // Accumulate all partial results
    for recv in rx {
        for (k, v) in recv {
            *ans.entry(k).or_insert(0) += v;
        }
    }

    ans
}
