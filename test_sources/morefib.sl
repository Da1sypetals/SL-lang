func main() {
    func fib(n) {
        if n < 0 {
            return "error";
        }
        if n == 0 {
            return 1;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        return fib(n - 1) + fib(n - 2) + fib(n - 3);
    }

    print 100 % 7;

    # let x = fib(24);
    # print x;

    <//
    func fib(n) {
        let x=1;let y=1;
        for i: n {
            let z = x + y;
            y=x;x=z;
        }
        return x;
    }

    print fib(3);
    //>
}