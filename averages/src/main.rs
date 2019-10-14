/*
Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data. Here are some exercises you should now be equipped to solve:

Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!

We’re getting into more complex programs in which operations can fail, so, it’s a perfect time to discuss error handling. We’ll do that next!
*/

mod mean;
mod median;
mod mode;

fn main() {
    let v = vec![
        5, 2, 7, 5, 4, 6, 2, 3, 8, 1, 9, 9, 1, 8, 3, 2, 9, 6, 2, 8, 8, 3, 1, 1,
    ];
    let averages = get_averages(&v);
    println!("{:?}", averages);
}

#[derive(Debug)]
struct Averages {
    mean: f64,
    median: median::Values,
    mode: Vec<i32>,
}

fn get_averages(vec: &Vec<i32>) -> Averages {
    let mean = mean::get_mean(vec);
    let median = median::get_median(vec);
    let mode = mode::get_mode(vec);

    Averages { mean, median, mode }
}
