// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Material {
    fn cost_per_sq_meter(&self) -> f64;
    fn squre_meters(&self) -> f64;
    fn retrieve_cost(&self) -> f64 {
        self.cost_per_sq_meter() * self.squre_meters()
    }
}

struct Carpet(f64);
impl Material for Carpet {
    fn cost_per_sq_meter(&self) -> f64 {
        10.00
    }
    fn squre_meters(&self) -> f64 {
        self.0
    }
}

struct Tile(f64);
impl Material for Tile {
    fn cost_per_sq_meter(&self) -> f64 {
        15.00
    }
    fn squre_meters(&self) -> f64 {
        self.0
    }
}

struct Wood(f64);
impl Material for Wood {
    fn cost_per_sq_meter(&self) -> f64 {
        20.00
    }
    fn squre_meters(&self) -> f64 {
        self.0
    }
}

fn calculate_total_cost(total_cost: &Vec<Box<dyn Material>>) -> f64 {
    total_cost
        .iter()
        .map(|material| material.retrieve_cost())
        .sum()
}

fn main() {
    let living_room_carpet: Box<Carpet> = Box::new(Carpet(6.5));
    let kitchen_tiles: Box<Tile> = Box::new(Tile(5.5));
    let master_bedroom_floor: Box<Wood> = Box::new(Wood(8.0));

    let costs: Vec<Box<dyn Material>> =
        vec![living_room_carpet, kitchen_tiles, master_bedroom_floor];

    let total = calculate_total_cost(&costs);
    println!("Total costs: ${:.2}", total);
}
