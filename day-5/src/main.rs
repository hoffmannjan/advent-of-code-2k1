use std::fs;
use std::cmp;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Segment {
    x1: u32,
    y1: u32,

    x2: u32,
    y2: u32
}

fn run() {
    let data = fs::read_to_string("./data.txt").expect("Unable to read file");

    let segments = Segment::parse_data(data).unwrap();

    // 1. get max width and max height of a vector of vectors
    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;

    for s in segments.iter() {
        let w = cmp::max(s.x1, s.x2);
        let h = cmp::max(s.y1, s.y2);

        if w > max_width {
            max_width = w + 1;
        }
        if h > max_height {
            max_height = h + 1;
        }
    }

    dbg!(max_width, max_height);

    // 2. the structure will look like
    //    [[0; max_width]; max_height]
    let mut matrix = vec![vec![0; (max_width + 1) as usize]; (max_height + 1) as usize];

    // 3. I will +1 on matrix for every segment coordinates
    for s in segments.iter() {
        // dbg!(s);
        let x1 = cmp::min(s.x1, s.x2);
        let y1 = cmp::min(s.y1, s.y2);

        let x2 = cmp::max(s.x1, s.x2);
        let y2 = cmp::max(s.y1, s.y2);

        let w = x2 - x1;
        let h = y2 - y1;

        dbg!(x1, y1, x2, y2);
        if (w > 0) {
            // println!("this is horizontal line");
            // dbg!(s);
            for i in x1..x2+1 {
                // dbg!(&matrix[y1 as usize]);

                matrix[y1 as usize][i as usize] += 1;
            }
        }
        if (h > 0) {
            // println!("this is vertical line");
            // dbg!(s);
            for i in y1..y2+1 {
                // dbg!(i);
                matrix[i as usize][x1 as usize] += 1;
            }
        }
    }

    // 4. I will check the num of ticks > 1
    let num_of_dangerous_fields = matrix.into_iter().flatten().fold(0, |acc, x| {
        if x > 1 {
            return acc + 1
        }
        return acc
    });

    dbg!(num_of_dangerous_fields);
}

impl Segment {
    fn parse_data(input: String) -> Option<Vec<Segment>> {
        let mut out = Vec::new();
            // println!("HALO {}", input.lines().len());

        for line in input.lines() {
            let mut parts = line.split(" -> ");
            let mut one = parts.next()?.split(",").filter(|x| !x.is_empty());
            let mut two = parts.next()?.split(",").filter(|x| !x.is_empty());

            let x1: u32 = one.next().unwrap().parse().unwrap();
            let y1: u32 = one.next().unwrap().parse().unwrap();
            let x2: u32 = two.next().unwrap().parse().unwrap();
            let y2: u32 = two.next().unwrap().parse().unwrap();

            if x1 != x2 && y1 != y2 {
                continue;
            }

            out.push(Segment { x1, y1, x2, y2 });
        }


        Some(out)
    }
}

fn main() {
    run();
}
