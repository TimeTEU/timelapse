mod timelapse;

use crate::timelapse::Timelapse;
use std::io;
use std::io::BufRead;

fn main() {
    let mut vec: Vec<i32> = Vec::new();

    for line in io::stdin().lock().lines() {
        let item = String::from(line.unwrap().trim());
        let val: i32 = item[0..10].parse().expect("int");
        vec.push(val);
    }
    let tl = Timelapse::from_sorted_vector(&vec, 1580599038, 60 * 60 * 24, 60 * 90);

    for (i, x) in tl.enumerate() {
        println!(
            "ln -s /media/timemachine/frames/{}.jpg /media/timemachine/frames/test/img{:08}.jpg",
            x, i
        );
    }
}
