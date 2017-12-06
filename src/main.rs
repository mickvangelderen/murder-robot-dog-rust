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

trait Drive {
    fn name(&self) -> &String;

    fn drive(&self) -> String {
        format!("{} is moving.", self.name())
    }
}

macro_rules! impl_drive {
    ($Type:ty) => {
        impl Drive for $Type {
            fn name(&self) -> &String {
                &self.name
            }
        }
    }
}

struct CleaningRobot {
    pub name: String,
}

impl CleaningRobot {
    fn clean(&self) -> String {
        format!("{} is cleaning.", self.name)
    }
}

impl_drive!(CleaningRobot);

struct KillingRobot {
    pub name: String,
}

impl KillingRobot {
    fn kill(&self) -> String {
        format!("{} is on a rampage!", self.name)
    }
}

impl_drive!(KillingRobot);

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

    {
        let r2d2 = CleaningRobot { name: String::from("R2D2") };
        println!("{}", r2d2.clean());
        println!("{}", r2d2.drive());
    }

    {
        let hk47 = KillingRobot { name: String::from("HK47") };
        println!("{}", hk47.kill());
        println!("{}", hk47.drive());
    }
}
