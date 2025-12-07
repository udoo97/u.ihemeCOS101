struct Brand {
    name: String,
    unit_price: u64,
}

impl Brand {
    fn total_cost(&self, qty: u32) -> u64 {
        self.unit_price * qty as u64
    }
}

fn main() {
    let brands = vec![
        Brand { name: "HP".to_string(), unit_price: 650_000 },
        Brand { name: "IBM".to_string(), unit_price: 755_000 },
        Brand { name: "Toshiba".to_string(), unit_price: 550_000 },
        Brand { name: "Dell".to_string(), unit_price: 850_000 },
    ];

    let qty = 3;
    let mut grand_total = 0;

    for b in &brands {
        let cost = b.total_cost(qty);
        println!("{}: {}", b.name, cost);
        grand_total += cost;
    }

    println!("Total cost: {}", grand_total);
}