fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n-1) + fib(n-2)
}

fn cycle1() {
    cycle2()
}

fn cycle2() {
    cycle1()
}

fn main() {
    fib(5);
    cycle1();
}
