use rand::Rng;
const ARRAY_SIZE: usize = 100000;
const ARRAY_MIN_GEN: i32 = -1000000000;
const ARRAY_MAX_GEN: i32 = 1000000000;

//The main function is really just printing the array
fn main() {
    let random_array = generate_random_array();
    let sorted_array = sort_array(random_array);
    print_array(sorted_array);
    print!("\n Sorted {ARRAY_SIZE} items in total");
}

fn sort_array(unsorted_array: [i32; ARRAY_SIZE]) -> [i32; ARRAY_SIZE] {
    let mut sorted_array = unsorted_array.clone();
    let mut swapped;
    loop {
        swapped = false;
        for i in 0..ARRAY_SIZE - 1 {
            if sorted_array[i] > sorted_array[i + 1] {
                sorted_array.swap(i, i + 1);
                swapped = true;
            }
        }
        if swapped == false {
            break;
        }
    }
    sorted_array
}

fn print_array(array_to_print: [i32; ARRAY_SIZE]) {
    for i in array_to_print {
        print!(", {i}");
    }
}

fn generate_random_array() -> [i32; ARRAY_SIZE] {
    let mut random_array = [0; ARRAY_SIZE];
    for i in 0..random_array.len() - 1 {
        random_array[i] = rand::thread_rng().gen_range(ARRAY_MIN_GEN..=ARRAY_MAX_GEN);
    }
    random_array
}
