fn main() {
    println!("Hello, world!");
    
    println!("Hello, broo!");
}


#[test]
fn imutable() {
    let name = "Mr Yan";
    println!("Hello {}", name);
}

#[test]
fn mutable() {
    let mut name = "Mr Yan";
    println!("Hello {}", name);

    name = "Mr Ran";
    println!("Hello {}", name);
}

#[test]
fn operation() {
    let a = 10;
    let b = 20;
    let c = a * b + a;
    println!("{}", c);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 20;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}


