fn main() {
    let principal: f64 = 520_000_000.0;
    let rate: f64 = 10.0;
    let years: f64 = 5.0;

    let amount = principal * (1.0 + rate / 100.0).powf(years);
    let compound_interest = amount - principal;

    println!("Principal (P): ₦{:.2}", principal);
    println!("Amount after {} years (A): ₦{:.2}", years, amount);
    println!("Compound Interest (CI): ₦{:.2}", compound_interest);
}
