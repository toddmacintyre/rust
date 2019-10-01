use std::io;
use std::num::Wrapping;

fn get_input() -> usize {
    loop {
        println!("Please enter a positive number.");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read user input");
        match buffer.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                continue;
            }
        }
    }
}

// fn fib(n: u128) -> u128 {
//     if n < 2 {
//         1
//     } else {
//         let mut memo = [1, 1];
//         let mut n = n - 2;

//         loop {
//             let [a, b] = memo;
//             let c = a + b;
//             // println!("{}", c);

//             if n == 0 {
//                 return c;
//             }

//             memo = [b, c];
//             n -= 1;
//         }
//     }
// }

// fn recursive_fib(n: usize) -> usize {
//     fn fib_memo(n: usize, memo: &mut [usize; 2]) -> usize {
//         let [a, b] = *memo;
//         let c = a + b;
//         if n == 0 {
//             c
//         } else {
//             *memo = [b, c];
//             fib_memo(n - 1, memo)
//         }
//     }

//     if n < 2 {
//         1
//     } else {
//         fib_memo(n - 2, &mut [1, 1])
//     }
// }

fn wrapping_fib(max: usize) {
    // let mut n: usize = 0;
    // let mut i: Wrapping<u128> = Wrapping(1);
    // let mut j: Wrapping<u128> = Wrapping(2);
    // let mut v: Vec<Wrapping<u128>> = vec![];

    // while n < max {
    //     n += 1;
    //     if n == 1 {
    //         v.push(Wrapping(1));
    //         continue;
    //     } else if n == 2 {
    //         v.push(Wrapping(2));
    //         continue;
    //     }
    //     let k = i + j;
    //     v.push(k);
    //     i = j;
    //     j = k;
    // }

    // println!("{:?}", v);

    let mut i: Wrapping<u128> = Wrapping(1);
    let mut j: Wrapping<u128> = Wrapping(2);
    for n in 1..=max {
        if n == 1 {
            print_result(&n, &Wrapping(0));
            continue;
        }
        if n == 2 || n == 3 {
            print_result(&n, &Wrapping(1));
            continue;
        }
        let result = i + j;
        print_result(&n, &result);
        i = j;
        j = result;
    }
}

fn print_result(&n: &usize, &result: &Wrapping<u128>) {
    println!("{n}: {result}", n = n, result = result);
}

fn main() {
    let n = get_input();
    // fib(n);
    wrapping_fib(n);
}
