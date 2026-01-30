fn main() {
    let converted: f64 = f_to_c(100.0);
    println!("Converted temp: {converted}");
}


fn f_to_c(temperature: f64) -> f64{
    let result: f64;

    result = (temperature - 32.0) * 5.0/9.0;

    result
}
