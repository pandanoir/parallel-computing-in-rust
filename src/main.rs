use std::{i32::MAX, thread};

fn calc_ao(v: &[i32]) -> i32 {
    let size = v.len();
    let mut slice = Vec::from_iter(v.iter().cloned());
    let num_to_omit = (size as f64 * 0.05).ceil() as usize;

    slice.sort();
    slice[num_to_omit..size - num_to_omit]
        .iter()
        .fold(0, |acc, x| acc + x)
        / (size as i32 - num_to_omit as i32 * 2)
}
fn calc_rolling_ao(v: &Vec<i32>, size: usize) -> Vec<i32> {
    if (v.len()) < size {
        return vec![0; size];
    }
    let mut average = vec![0; size - 1];
    for i in 0..v.len() - size + 1 {
        average.push(calc_ao(&v[i..i + size]));
    }
    average
}
fn calc_best_ao(v: &Vec<i32>, size: usize) -> Result<i32, String> {
    if (v.len()) < size {
        return Err("hoge".to_string());
    }
    let mut best = MAX;
    for i in 0..v.len() - size + 1 {
        let ao = calc_ao(&v[i..i + size]);
        if ao < best {
            best = ao;
        }
    }
    Ok(best)
}
fn main() {
    let times = vec![
        7, 92, 83, 11, 93, 82, 88, 46, 26, 28, 19, 36, 44, 26, 60, 27, 92, 50, 20, 24, 78, 67, 3,
        76, 17, 25, 55, 92, 26, 53, 68, 19, 30, 59, 13, 23, 31, 75, 33, 32, 32, 15, 8, 97, 22, 62,
        58, 56, 48, 59, 78, 80, 43, 58, 59, 12, 85, 34, 62, 60, 84, 88, 89, 26, 21, 62, 23, 10, 4,
        37, 75, 67, 75, 45, 77, 46, 98, 55, 34, 11, 42, 31, 61, 64, 87, 34, 40, 61, 82, 31, 80, 18,
        1, 14, 71, 77, 0, 42, 1, 18,
    ];

    thread::scope(|scope| {
        // let handle_ao5 = scope.spawn(|| calc_rolling_ao(&times, 5));
        // let handle_ao12 = scope.spawn(|| calc_rolling_ao(&times, 12));
        // let handle_ao25 = scope.spawn(|| calc_rolling_ao(&times, 25));
        // let handle_ao100 = scope.spawn(|| calc_rolling_ao(&times, 100));
        let handle_ao5 = scope.spawn(|| calc_best_ao(&times, 5));
        let handle_ao12 = scope.spawn(|| calc_best_ao(&times, 12));
        let handle_ao25 = scope.spawn(|| calc_best_ao(&times, 25));
        let handle_ao100 = scope.spawn(|| calc_best_ao(&times, 100));

        let ao5 = handle_ao5.join().unwrap();
        let ao12 = handle_ao12.join().unwrap();
        let ao25 = handle_ao25.join().unwrap();
        let ao100 = handle_ao100.join().unwrap();
        println!("Here's ao5: {:?}", ao5);
        println!("Here's ao12: {:?}", ao12);
        println!("Here's ao25: {:?}", ao25);
        println!("Here's ao100: {:?}", ao100);
    });
}
