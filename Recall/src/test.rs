fn datatype<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}

pub fn foo() {
    println!("\nDataTypes:");
    let age: &str = "47";
    println!("{}\t:\t{:?}", age, datatype(&age));
    let age: i32 = age.trim().parse().expect("Not a number");
    println!("{}\t:\t{:?}", age, datatype(&age));
}
