trait Poop {
    fn name(&self) -> &String;

    fn poop(&self) -> String {
        format!("{} pooped.", self.name())
    }
}

macro_rules! impl_poop {
    ($Type:ty) => {
        impl Poop for $Type {
            fn name(&self) -> &String {
                &self.name
            }
        }
    }
}

struct Dog {
    pub name: String,
}

impl Dog {
    fn bark(&self) -> String {
        format!("{}: Woof!", self.name)
    }
}

impl_poop!(Dog);

struct Cat {
    pub name: String,
}

impl Cat {
    fn meow(&self) -> String {
        format!("{}: Meow!", self.name)
    }
}

impl_poop!(Cat);

fn main() {
    {
        let mick = Dog { name: String::from("Mick") };
        println!("{}", mick.bark());
        println!("{}", mick.poop());
    }

    {
        let oliv = Cat { name: String::from("Oliver") };
        println!("{}", oliv.meow());
        println!("{}", oliv.poop());
    }
}
