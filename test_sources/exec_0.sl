model Person {
    name, age, salary,
}

func main() {
    let print_ra = false;
    let a = 3;
    let __ra = -1242.1222;

    print __ra;
    
    func print_abn(a, na, b, nb) {
        for i: na {
            print a;
            # cannot find in call scope
            # print __ra;
        }
        for i: nb {
            print b;
        }
    }

    let sal = 5999;
    let p0 = new Person;
    p0.age = -19.6;
    p0.salary = sal;
    print p0;
    sal = 8888;
    print p0;
    

    let prx = print_abn;

    let res = prx(-114.514, 3, exile, 6);
    print res;

    print empty;
    for i: a {
        let z = exile;
        {
            let z = false;
            if print_ra {
                print __ra;
            } else {
                # this is hello world comment
                print z; print "Hello world!";
            }
        }
        print a;
    }
    # for i: 10 {
    #     for j: 5 {
    #         print i;
    #         print j;
    #     }
    # }

    # let cond = true;
    # while cond {
    #     print "fancy words";
    #     # inf loop
    #     # let cond = false;
    # }

    let b = 2;
    print "B = ";
    print b;
    print z;
    print b;
}