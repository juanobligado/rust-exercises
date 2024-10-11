
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    email: String,
    age: u8,
    active: bool
}

fn main() {
    let mut a_person = Person::new("John", "Doe", "john@doe", 30);
    println!("{:?}", a_person);
    println!("{}",a_person.full_name());
    a_person.deactivate();
    println!("{:?}", a_person);
}
impl Person {
    pub fn new(first_name: &str, last_name: &str, email: &str, age: u8) -> Self {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            age,
            active: true
        }
    }

    pub fn from_email(email: &str) -> Self {
        let user_name = email.split('@').collect::<Vec<&str>>();
        Person {
            first_name: user_name[0].to_string(),
            last_name: "Doe".to_string(),
            email: email.to_string(),
            age: 30,
            active: true
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

