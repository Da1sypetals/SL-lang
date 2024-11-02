model Person {
    name, age, salary,
}

model Book {
    name, isbn, _n_page,
}

let e = 2.71828;

func main() {

    let b1 = new Book;
    let b2 = new Book;

    print b1;
    print b2;

    # return "Success";

    b1.name = b2;
    b2.name = b1;

    print "ok";
    print b1;
    print b2;

}