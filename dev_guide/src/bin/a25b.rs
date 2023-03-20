struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}

fn main() {
    // The idea behind the Default Trait is to define
    // pre-determined data to make your code easier.
    // A suitable scenario of default is when your
    // new() implementation does not require parameters.
    // In this case, its better to implement default()
    // instead of new()
    let _p: Package = Package::default();
}
