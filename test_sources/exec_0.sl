model Person {
    name, age, salary,
}

model Book {
    name, isbn, _n_page,
}

let e = 2.71828;

func sayhi(thing) {
    print "<sayhi> hello::::";
    print thing;
}

func main() {

    func loop(a) {
        let i = 0;
        while true {
            if i >= a + 2 {
                return i;
            }
            i = i + 2;
        }
    }

    print "z=";
    let z = loop(1);
    print z;

    func mult(a, b) {
        print a;
        print b;
        return a * b;
    }

    print mult(1.414, -1.414);
    print mult(12, 13);

    let print_ra = false;
    let a = 3;
    let __ra = -1242.1222;

    let psn = new Person;
    let boy = new Person;

    boy.age = 14.33;
    psn.age = boy;

    let qq = new Person;
    let ww = new Person;
    ww.age = qq;
    let ee = qq;

    print qq==ww;
    print qq==ee;
    print qq;
    print ww;

    ee.age = 1034;
    print qq.age;

    sayhi(__ra);
    sayhi(e * 4.0 * psn.age.age);
    sayhi(a);
    sayhi(e == 2.71828);
    sayhi(e == 2.72);
    
    <//
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
    
    print_abn(true, 2, false, 2);


    1883.1241;
    -1242;
    nil;
    excel;
    false;

    let sal = 5999;
    let p0 = new Person;
    p0.age = -19.6;
    p0.salary = sal;
    print p0;
    sal = 8888;
    print p0;

    let z = new Person;

    p0.name = z;
    z.name = "candice";
    p0.name.salary = 666666;
    # z.salary = p0;

    print "-----------------------";
    print "-----------------------";

    print z;
    print p0;
    
    p0.name.salary = z;
    # print p0;

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
    print __ww__;
    print b;
    //>
}