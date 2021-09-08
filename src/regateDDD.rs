#[derive(Debug)]
pub enum Cap {
    N,
    S,
    W,
    E,
}

#[derive(Debug)]
struct Boat {
    enterprise_name: String,
    cap: Cap,
}

struct Regate {
    boats: Vec<Boat>,
}

impl Boat {
    pub fn get_cap(&self) -> &Cap {
        &self.cap
    }

    pub fn change_cap(self, new_cap: Cap) -> Self {
        Boat {
            enterprise_name: self.enterprise_name,
            cap: new_cap,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let boat = Boat {
        enterprise_name: String::from("Cochonou"),
        cap: Cap::N,
    };
    let boat_cap = boat.get_cap();
    let new_boat = boat.change_cap(Cap::S);

    println!("New nboat {:?}", new_boat);
}
