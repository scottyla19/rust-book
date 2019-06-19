fn main() {
    let c: f32 = convert_f_to_c(37.0);
    println!("The temp in C is {}", c);

    let n = 9;
    let fibs: i32 = fib(n);
    println!("The {}th term is {}", n, fibs);

    xmas();
}

fn convert_f_to_c(f: f32) -> f32 {
    let c: f32 = (f - 32.0) * (5.0 / 9.0);
    c
}

fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;

    if (n == 0) {
        return a;
    }
    for _val in (1..n) {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}

fn xmas() {
    let a = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
    ];
    let v = ["first", "second", "third"];
    for (i, item) in a.iter().enumerate() {
        if i == 0 {
            println!(
                "On the {} day of Christmas my true love sent to me {}",
                v[i], item
            );
        } else {
            let sliced_a = &a[0..i + 1];
            let mystr = sliced_a.join(", ");
            println!(
                "On the {} day of Christmas my true love sent to me {}",
                v[i], mystr
            );
        }
    }
}
