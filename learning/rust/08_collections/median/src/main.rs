use std::collections::HashMap;

fn fibonacci_seq(n: u16) -> Vec<u128> {
    if n == 1 {
        return vec![0]
    }

    let mut seq = vec![0, 1];
    for _ in 0..n-2 {
        let last_two = seq.last_chunk::<2>().unwrap();
        seq.push(last_two.iter().sum());
    }
    seq
}

fn median(seq: &Vec<u128>) -> f64 {
    let mut seq = seq.clone();
    seq.sort_unstable();
    let mid = seq.len() / 2;
    
    if seq.len() % 2 == 0 {
        (seq[mid - 1] as f64 + seq[mid] as f64) / 2.0
    }
    else {
        seq[mid] as f64
    }
}

fn mode(seq: &Vec<u128>) -> u128 {
    let mut map = HashMap::new();
    for num in seq {
        map.entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    **map.iter().max_by_key(|&(_, count)| count).unwrap().0
}

fn main() {
    let n= 6;
    let seq = fibonacci_seq(n);
    println!("Sequence: {:?}", seq);
    println!("Median {}", median(&seq));
    println!("Mode of the sequence: {}", mode(&seq))
}
