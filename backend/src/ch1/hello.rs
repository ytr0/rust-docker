fn g(t: &str) {
    println!("g: {t}");
}

fn f() {
    let mut s = String::from("Hey"); 
    let t = &s;
    g(&s);
    s.push_str(" there!");
    // println!("f: {s} {t}");
}

fn main() {
    f();
    let x = 5 * 5;
    println!("Hello, World! {}", x);
}