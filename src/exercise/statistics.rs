use std::collections::HashMap;

pub fn get_median_and_mode(xs: &Vec<i32>) -> (f32, i32) {
    (get_median(xs.clone()), get_mode(&xs))
}

fn get_median(mut xs: Vec<i32>) -> f32 {
    let len = xs.len();
    if len == 0 {
        return 0.0;
    }

    xs.sort_by(|a, b| b.cmp(a));
    let middle = len / 2;

    if len % 2 == 0 {
        return ((xs[middle - 1] + xs[middle]) as f32) / 2.0;
    }

    return xs[middle] as f32;
}

fn get_mode(xs: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut mode: i32 = 0;
    let mut max = 0;
    for x in xs {
        let count = map.entry(x).or_insert(0);
        *count += 1;

        if *count > max {
            max = *count;
            mode = *x;
        }
    }

    return mode;
}
