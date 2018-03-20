// struct2.rs

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn copy(&self) -> Self {
        Self::new(&self.first_name,&self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }
}

fn main() {
    let p = Person::new("John","Smith");
    let mut q = p.copy();
    q.set_first_name("Bob");
    println!("person {} {}", p.first_name,p.last_name);
    println!("fullname {}", p.full_name());
    println!("fullname {}", q.full_name());
    println!("{:?}", q.to_tuple());
    println!("{:?}", p);
}
