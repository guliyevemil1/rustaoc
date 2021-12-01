pub fn run1(input : Vec<String>) -> i32 {
    let mut out : i32 = 0;
    let mut prev : i32 = -1;
    for w in input {
        let cur : i32 = w.parse::<i32>().unwrap();
        if prev != -1 && cur > prev {
            out += 1;
        }
        prev = cur;
    }
    out
}

pub fn run2(input : Vec<String>) -> i32 {
    let mut out : i32 = 0;
    let mut prev : [i32; 3] = [-1, -1, -1];
    for i in 0..input.len() {
        let cur : i32 = input[i].parse::<i32>().unwrap();
        if i < 3 {
            prev[i] = cur;
            continue;
        }
        if prev[0] < cur {
            out += 1;
        }
        prev[0] = prev[1];
        prev[1] = prev[2];
        prev[2] = cur;
    }
    out
}
