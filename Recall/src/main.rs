use test::foo;
mod foo;
mod test;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let num: [i8; 4] = [10, 20, 30, 127];
    println!("{:#?}", num);
    let mut x: i32 = 0;
    (-5..=5).for_each(|i: i32| {
        x += i;
    });
    let mut _a: usize = 0;
    for i in 0..num.len() {
        print!("{} ", num[i]);
        _a += i;
        x += i as i32;
    }
    println!("\nx = {}", x);
    println!("a = {}", _a);
    for v in num {
        print!("{} ", v);
    }
    let resp = reqwest::blocking::get("https://github.com/akash2061")?;
    println!("\n{:#?}", resp.url());
    println!("Status => {:#?}", resp.status());

    foo();
    foo::main();

    Ok(())
}
