trait Sale {
    fn amount(&self) -> f64;
}

struct FullSale(f64);
impl Sale for FullSale {
    fn amount(&self) -> f64 {
        self.0
    }
}

struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon {
    fn amount(&self) -> f64 {
        self.0 - 1.0
    }
}

struct TenPercentOffPromo(f64);
impl Sale for TenPercentOffPromo {
    fn amount(&self) -> f64 {
        self.0 * 0.9
    }
}

fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}

fn main() {
    let price: f64 = 20.0;
    let regular = Box::new(FullSale(price));
    let dollar_off = Box::new(OneDollarOffCoupon(price));
    let ten_percent_off = Box::new(TenPercentOffPromo(price));

    let sales: Vec<Box<dyn Sale>> = vec![regular, dollar_off, ten_percent_off];
    let revenue = calculate_revenue(&sales);
    println!("Total revenue: {:.2}", revenue);
}
