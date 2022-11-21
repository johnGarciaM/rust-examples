#[derive(PartialEq, Eq)]
#[allow(dead_code)]
pub enum PersonKind {
    Student,
    Employee,
    Profesional,
}

impl std::fmt::Debug for PersonKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Student => write!(f, "Student"),
            Self::Employee => write!(f, "Employee"),
            Self::Profesional => write!(f, "Profesional"),
        }
    }
}

#[allow(dead_code)]
pub struct Person {
    person_name: &'static str,
    is_active: bool,
    age: i32,
    weight: f32,
    person_kind: PersonKind,
}

#[allow(dead_code)]
pub fn examples_of_variables() -> Person {
    // primitive's kinds
    let person_name: &str = "John García";
    let is_active: bool = true;
    let age: i32 = 30;
    let weight: f32 = 75.5;
    let person_kind: PersonKind = PersonKind::Employee;

    let new_person: Person = Person {
        person_name,
        is_active,
        age,
        weight,
        person_kind,
    };

    println!(
        "Person Data: name {}, age {}, is active {}, weight {}, person_kind {:#?}",
        new_person.person_name,
        new_person.age,
        new_person.is_active,
        new_person.weight,
        new_person.person_kind
    );

    new_person
}

#[allow(dead_code)]
pub fn sub(num1: i8, num2: i8) -> i8 {
    num1 - num2
}
