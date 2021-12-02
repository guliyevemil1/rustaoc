pub fn run1(input : Vec<String>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for w in input {
        let ps : Vec<&str> = w.split(" ").collect();
        let d : i32 = ps[1].parse::<i32>().unwrap();
        match ps[0] {
            "forward" => x = x+d,
            "down" => y = y+d,
            "up" => y = y-d,
            _ => (),
        }
    }
    x*y
}

pub fn run2(input : Vec<String>) -> i32 {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for w in input {
        let ps : Vec<&str> = w.split(" ").collect();
        let d : i32 = ps[1].parse::<i32>().unwrap();
        match ps[0] {
            "forward" => {
                x += d;
                y += aim * d;
            },
            "down" => aim += d,
            "up" => aim -= d,
            _ => (),
        }
    }
    x*y
}
