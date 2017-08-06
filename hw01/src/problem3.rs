/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut ans = Vec::new();
    let mut visited = vec![0;n as usize];

    for i in 2..n {
        if visited[i as usize] == 0 {
            ans.push(i);
            let mut j = i*i;
            while j < n {
                visited[j as usize] = 1;
                j+=i;
            }
        }
    }

    ans
}

//
// Problem 3
//

#[test]
fn test_sieve_basic() {
    assert_eq!(vec![2,3,5,7,11], sieve(12));
}

