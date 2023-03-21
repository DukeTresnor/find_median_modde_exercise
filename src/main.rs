use std::collections::HashMap;

fn main() {
    let mut integer_list = [2, 1, 5, 6, 5, 5, 5, 6, 6, 6, 6, 6];

    println!("Unsorted: {:?}", integer_list);

    integer_list.sort_unstable();

    // Creating mutable vector to add integer_list to
    let mut my_vector: Vec<usize> = Vec::new();

    for number in &mut integer_list {
        my_vector.push(*number);
    }

    let median = {
        if my_vector.len()%2 == 0 {
            // Even number of elements
            println!("Even number of elements");
            let high_median = my_vector[my_vector.len()/2];
            let low_median = my_vector[my_vector.len()/2 - 1];
            (high_median + low_median) / 2 
        } else {
            // Odd number of elements
            println!("Odd number of elements");
            my_vector[my_vector.len()/2]
        }
    };

    println!("Sorted: {:?}", my_vector);

    println!("Median: {:?}", median);

    // ------- Finding the mode ------- //

    let mut list_hash = HashMap::new();

    for number in &integer_list {
        // Inserts number into the list_hash if it's not there. If it is, increaase its count by 1.
        let count = list_hash.entry(number).or_insert(0);
        *count += 1;
    }

    println!("List_hash, with (key: value) --> (integer: frequency): {:?}", list_hash);

    let mode = {
        let mut greatest_frequency = 0;
        let mut greatest_key: usize = 0;
        for (key, value) in &list_hash {
            if value > &greatest_frequency {
                greatest_frequency = *value;
                greatest_key = **key;
            } else {
                ();
            }
        }

        greatest_key
    };

    println!("Mode: {mode}");

}