use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub enum StarClass {
    ClassA,
    ClassM,
}

impl Display for StarClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarClass::ClassA => {
                write!(f, "Class A")
            }
            StarClass::ClassM => {
                write!(f, "Class M")
            }
        }
    }
}
