fn main() {
    let principal: f64 = 510_000.0;
    let rate: f64 = 5.0;
    let years: f64 = 3.0;

    let value = principal * (1.0 - rate / 100.0).powf(years);

    println!("Value of the TV after {} years: ₦{:.2}", years, value);
}
