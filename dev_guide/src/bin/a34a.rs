struct Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state: state,
        }
    }
}

struct Agreement;
struct Signature;
struct Training;
struct FailedTraining {
    score: u8,
}
struct OnboardingComplete {
    score: u8,
}

impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            state: Agreement,
        }
    }
    fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}

impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
        if score >= 7 {
            Ok(self.transition(OnboardingComplete { score }))
        } else {
            Err(self.transition(FailedTraining { score }))
        }
    }
}

fn main() {
    let employee = Employee::new("Pit");
    let onboarded = employee.read_agreement().sign().train(9);
    match onboarded {
        Ok(employee) => println!("Onboarding complete with score: {}", employee.state.score),
        Err(employee) => println!("training failed, score: {}", employee.state.score),
    }
}
