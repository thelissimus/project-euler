fn solution(n: i32) -> i32 {
    let mut sum = 0;

    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    sum
}

fn solution_fp(n: i32) -> i32 {
    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

fn main() {
    let result = solution_fp(1000);
    println!("{}", result);
}
