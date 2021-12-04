use std::fs;

const RADIX: u32 = 10;

fn main() {
    let csv_file_path = "./data.csv";
    let data = fs::read_to_string(csv_file_path).expect("Unable to read file");
    let num_len = data.lines().next().unwrap().len();

    println!("{}", num_len);

    let mut gamma = vec![0; num_len];
    let mut epilson = vec![1; num_len];
    let mut epilson2 = vec!["0"; num_len];

    let result = data.lines().enumerate().fold([[0; 12]; 5], |mut acc, (i1, line)| {
        for (i2, c) in line.chars().enumerate() {
            acc[i2][i1] = c.to_digit(RADIX).unwrap();
        }
        acc
    });


    dbg!(result);

}

// const WIDTH: usize = 12;
// const COUNT: usize = 1000;

// pub fn main() {
//     let gamma = include_str!("../input.txt")
//         .lines()
//         .map(|l| usize::from_str_radix(l, 2).unwrap())
//         .fold(vec![0; WIDTH], |count, bits| {
//             count
//                 .into_iter()
//                 .enumerate()
//                 .map(|(i, n)| n + ((bits & 1 << i) >> i))
//                 .collect()
//         })
//         .into_iter()
//         .enumerate()
//         .map(|(i, b)| ((b >= COUNT / 2) as u32) << i)
//         .sum::<u32>();

//     println!("{}", gamma * (!gamma & ((1 << WIDTH) - 1)));
// }
