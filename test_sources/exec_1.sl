model Person {
    name, age, salary,
}

model Book {
    name, isbn, _n_page,
}

let e = 2.71828;

func main() {

    let p = new Person;
    
    {
        let b = new Book;
        b.name = "jane eyre";
        p.name = b;
        print p.name.name;
    }

    print p.name.name;
    print p.age;
    print p.name._n_page;
    
    # print p.age + p.name;

    let z =  (1.3+4.0)*14.12;

    let a = 2;
    print z;
    # print a;

    # for i: 600000 {
    #     let a = 1;
    #     let q = 1;
    #     let w = 1;
    #     # let e = 1;
    # }

    # print "a=";
    # print a;

    print "e=";
    print e;
    
    # print p.name.name;

}