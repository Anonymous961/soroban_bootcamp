fn main() {
    let x: i32= 16;
    println!("{}",x);

    let z: String = String::from("Hello,Soroban"); //mutable string
    let y: &str= "Hello, Stellar!"; //immutable string

    println!("{y}");
    println!("{z}");

    pub fn event(name: String){
        let name: String= String::from("James");
        println!("{}",name);
    }
    let e: EventsForKids= EventsForKids{
        name: String::from("Kidsco"),
        date: String::from("04.03.2024"),
        number_of_participants:1000,
        place: String::from("NY, USA")
    };
}

struct EventsForKids{
    name: String,
    date: String,
    number_of_participants: u32,
    place:String
}

enum ErrorForEvent{
    NoEvent,
    CancelledEvent,
    EventType
}