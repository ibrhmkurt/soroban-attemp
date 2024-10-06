fn main() {

    let x: i32 = 16;

    println!("{}", x);

    let z: String = String::from("Hello, Soroban!");
    let y: &str = "Hello, Stellar!";

    println!("{y}");
    println!("{z}");

    let e: EventForKids = EventForKids {
        name:  String::from("KidsCo"),
        date: String:: from("04.03.2024"),
        number_of_particificants: 1000,
        place: String::from("NY, USA")
    };
 
    // add your enum here..
    
}

struct EventForKids {
    name: String,
    date: String,
    number_of_particificants: u32,
    place: String
}

enum ErrorsForEvent {
    NoEvent,
    CancelledEvent,
    EventType
}
