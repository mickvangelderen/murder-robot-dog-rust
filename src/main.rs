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

trait Bark {
    fn name(&self) -> &String;

    fn bark(&self) -> String {
        format!("{}: Woof!", self.name())
    }
}

macro_rules! impl_bark {
    ($Type:ty) => {
        impl Bark for $Type {
            fn name(&self) -> &String {
                &self.name
            }
        }
    }
}

struct Dog {
    pub name: String,
}

impl_bark!(Dog);
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

trait Killer {
    fn name(&self) -> &String;

    fn kill(&self) -> String {
        format!("{} is on a rampage!", self.name())
    }
}

macro_rules! impl_killer {
    ($Type:ty) => {
        impl Killer for $Type {
            fn name(&self) -> &String {
                &self.name
            }
        }
    }
}

struct KillingRobot {
    pub name: String,
}

impl_killer!(KillingRobot);
impl_drive!(KillingRobot);

// Here it comes...

struct MurderRobotDog {
    pub name: String,
}

impl_bark!(MurderRobotDog);
impl_drive!(MurderRobotDog);
impl_killer!(MurderRobotDog);

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

    {
        let mpjme = MurderRobotDog { name: String::from("Matthias") };
        println!("{}", mpjme.bark());
        println!("{}", mpjme.drive());
        println!("{}", mpjme.kill());
    }
}
