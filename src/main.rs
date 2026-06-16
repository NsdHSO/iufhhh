use std::{num, result};

fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    fn foo<'a, 'b>(a: &'a str, b: &'b str) -> &'b str {
        b
    }
    fn foo2<'a>(a: &'a str, b: &'a str) -> &'a str {
        b
    }
    {
        let e = String::from("Iancu");
        let result2;
        let x = String::from("X Third");
        {
            let result = foo(r1, &x);
            println!("{}", result);
            result2 = foo2(r1, &x);
        }
        println!("test foo2{}", result2);
        let result = foo(r1, &e);
        println!("{}", result)
    }
    let result = foo(r1, r2);
    println!("{}", result);

    fn first(v: &[i32]) -> &i32 {
        &v[0]
    }
    let num = vec![32, 1, 2, 3];
    let n = first(&num);
    print!("{:?}", n);
}
