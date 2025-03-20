use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

/// Runs the specified sorting algorithm using input from the user
pub fn run_sort_algo(algo: &String) {
    let num_items = get_i32("enter num items: ");
    let max_value = get_i32("enter max algo: ");
    let mut random_vec = make_random_vec(num_items, max_value);
    print_vec(&random_vec, num_items);
    match algo.as_str() {
        "bubblesort" => {
            bubble_sort(&mut random_vec);
        },
        "quicksort" => {
            let len = random_vec.len();
            quicksort(&mut random_vec, 0, len - 1);
        },
        _ => {
            println!("Invalid sort algo: {}", algo);
            std::process::exit(1);
        }
    }
    print_vec(&random_vec, num_items);
    check_sorted(&random_vec);
}

// Bubblesort ~ Multiple passes over an array, swapping adjacent elements O(n^2)
pub fn bubble_sort(vec: &mut Vec<i32>) {
    let mut n = vec.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..=n-1 {
            if vec[i-1] > vec[i] {
                vec.swap(i-1, i);
                swapped = true;
            }
        }
        n = n - 1
    }
}

pub fn partition(vec: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
    let pivot = vec[hi];
    let mut i = lo;
    for j in lo..=hi-1 {
        // If the current element is less than or equal to the pivot
        if vec[j] <= pivot {
            vec.swap(i, j);
            i = i + 1
        }
    }
    vec.swap(i, hi);
    i
}

// Quicksort ~ D&C algo with recursion, partition + pivot O(n log n)
pub fn quicksort(vec: &mut Vec<i32>, lo: usize, hi: usize) {
    if vec.len() < 2 {
        return;
    }

    // Ensure indices are in correct order
    if lo >= hi {
        return;
    }

    let pivot = partition(vec, lo, hi);
    // Sort the two partitions
    if pivot > 0 {
        quicksort(vec, lo, pivot - 1); // Left side of pivot
    }
    if pivot < hi {
        quicksort(vec, pivot + 1, hi); // Right side of pivot
    }
}

fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    vec
}

// Print at most num_items items.
fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

// Verify that the Vec is sorted.
fn check_sorted(vec: &Vec<i32>) {
    let n = vec.len();
    for i in 0..n-1 {
        if vec[i] > vec[i+1] {
            println!("The vector is NOT sorted!");
            return;
        }
    }
    println!("The vector is sorted!");
}

fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse::<i32>().expect("Error parsing integer")
}

// ************
// *** Prng ***
// ************
struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        prng
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        self.seed
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        f / (2147483647.0 + 1.0)
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        result as i32
    }
}