pub fn sum(slice: &[i32]) -> i32 {
    let mut ans: i32;
    ans = 0;
    for s in slice {
        ans += *s;
    }
    ans
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    
    for v in vs {
        if !ans.contains(v) {
            ans.push(*v);
        }
    }

    ans
}

pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut vec = Vec::new();
    for v in vs {
        if pred(*v) {
            vec.push(*v);
        }
    }
    vec
}

// Part 1

#[test]
fn test_sum_small() {
    let array = [1,2,3,4,5];
    assert_eq!(sum(&array), 15);
}

// Part 2

#[test]
fn test_dedup_small() {
    let vs = vec![1,2,2,3,4,1];
    assert_eq!(dedup(&vs), vec![1,2,3,4]);
}

// Part 3

fn even_predicate(x: i32) -> bool {
    (x % 2) == 0
}

#[test]
fn test_filter_small() {
    let vs = vec![1,2,3,4,5];
    assert_eq!(filter(&vs, &even_predicate), vec![2,4]);
}
