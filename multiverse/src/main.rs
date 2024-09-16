// hmm this doesn't look right!!
struct UniverseDetails {
    universe_name : String,
    universe_winner : String,
    universe_population : u32, 
}

fn get_universe_details(universe_id: u32) -> Option<UniverseDetails> {
    // does this even compile??
    /* struct universe_details;
    if universe_id % 3 == 0 {
        return None(universe_details {
            universe_name: "Milky Way",
            universe_winner: "The Galaxy".to_string(),
            universe_population: "six",
        })   
    } */
    let universe_details  = if universe_id % 15 == 0{
        Some(UniverseDetails{
            universe_name: String::from("Stardew Valley"),
            universe_winner: String::from("Jojo Corp"),
            universe_population : 1,
        })
    }else if universe_id % 3 == 0 {
        Some(UniverseDetails{
            universe_name: String::from("Star Wars"),
            universe_winner: String::from("The Rebellion"),
            universe_population : std::u32::MAX,
        })
    }else if universe_id % 5 == 0{
        Some(UniverseDetails{
            universe_name: String::from("Miraculous"),
            universe_winner: String::from("Hawk Moth"),
            universe_population : 22,
        })
    }else{
        None
    };

    universe_details
}


// this main function is fine, except for two gaps
// the print statements could make use of "{variable}" instead of 
// ("{}", variable)
fn main() {
    for id in 1..=15 {
        let universe_details = get_universe_details(id);
        if let Some(details) = universe_details {
            println!("Universe with id {} is called {}, won by {} and has a population of {}", 
                id, 
                details.universe_name, 
                details.universe_winner, 
                details.universe_population
            );
        } else {
            println!("Universe with id {} is unknown", id);
        }
    }
}
