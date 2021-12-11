use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    one();
}

fn one(){
    let mut count = 0;
    let mut previous = -1;

    if let Ok(lines) = read_lines("values.txt") {
        for line in lines{
            if let Ok(ip) = line {
                let val: i32 = ip.parse().unwrap();
                if previous == -1{
                    previous = val;
                } else {
                    if val > previous{
                        count += 1;
                        previous = val;
                    } else {
                        previous = val;
                    }
                }
            }
        }
    }
    println!("{}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}