// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

struct Luggage(LuggageId);
#[derive(Debug)]
struct LuggageId(isize);
struct CheckIn(LuggageId);
struct OnLoading(LuggageId);
struct Offloading(LuggageId);
struct AwaitingPickup(LuggageId);
struct EndCustody(LuggageId);
struct LostAndFound(LuggageId);

impl Luggage {
    fn new(id: LuggageId) -> Self {
        Luggage(id)
    }
    fn check_in(self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn onload(self) -> OnLoading {
        OnLoading(self.0)
    }
}

impl OnLoading {
    fn offload(self) -> Offloading {
        Offloading(self.0)
    }
}

impl Offloading {
    fn carousel(self) -> AwaitingPickup {
        AwaitingPickup(self.0)
    }
}

impl AwaitingPickup {
    fn pick_up(self, picked_up: bool) -> Result<EndCustody, LostAndFound> {
        if picked_up {
            Ok(EndCustody(self.0))
        } else {
            Err(LostAndFound(self.0))
        }
    }
}

fn main() {
    let id = LuggageId(111);
    let luggage = Luggage::new(id);
    let luggage_picked = luggage
        .check_in()
        .onload()
        .offload()
        .carousel()
        .pick_up(true);
    match luggage_picked {
        Ok(lugg) => println!("{:?} picked up!", lugg.0),
        Err(lugg) => println!("{:?} went to lost-and-found section..", lugg.0),
    }
}
