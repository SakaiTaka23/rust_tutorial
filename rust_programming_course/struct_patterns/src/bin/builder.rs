use crate::MembershipType::{Loyal, New};

#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: MembershipType,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Clone)]
enum MembershipType {
    New,
    Casual,
    Loyal,
}

impl Default for MembershipType {
    fn default() -> Self {
        New
    }
}

impl Customer {
    fn new_builder(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name,
            username: None,
            membership: None,
            gender: None,
            country: None,
            age: None,
        }
    }

    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    fn new_2(name: String, username: String) -> Self {
        Self {
            name,
            username,
            ..Default::default()
        }
    }

    fn new_3(name: String, username: String, membership_type: MembershipType) -> Self {
        Self {
            name,
            username,
            membership: membership_type,
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<MembershipType>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: MembershipType) -> &mut Self {
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }

    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.clone().unwrap_or_default(),
            country: self.country.clone().unwrap_or_default(),
            age: self.age.clone().unwrap_or_default(),
        }
    }
}

fn main() {
    let new_user = Customer::new(String::from("Nouman"));
    let user_with_login = Customer::new_2(String::from("Joseph"), String::from("joe123"));
    let user_with_membership = Customer::new_3(String::from("Micheal"), String::from("micheal123"), Loyal);
    let user_with_membership = Customer::new_builder(String::from("Micheal"))
        .username(String::from("micheal123"))
        .membership(Loyal)
        .build();
}
