struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T>
where
    T: Convey,
{
    pub items: Vec<T>,
}

impl<T> ConveyorBelt<T>
where
    T: Convey,
{
    pub fn add(&mut self, item: T) {
        self.items.push(item)
    }
}

struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 1.0,
            depth: 2.0,
            weight: 3.0,
            part_number: "abc".to_owned(),
        }
    }
}

impl Convey for CarPart {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
    fn weight(&self) -> f64 {
        self.weight
    }
}

struct MotorcyclePart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for MotorcyclePart {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 1.0,
            depth: 2.0,
            weight: 3.0,
            part_number: "def".to_owned(),
        }
    }
}

impl Convey for MotorcyclePart {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
    fn weight(&self) -> f64 {
        self.weight
    }
}

fn main() {
    let mut belt_car: ConveyorBelt<CarPart> = ConveyorBelt { items: vec![] };
    belt_car.add(CarPart::default());
    let mut belt_moto: ConveyorBelt<MotorcyclePart> = ConveyorBelt { items: vec![] };
    belt_moto.add(MotorcyclePart::default());
}
