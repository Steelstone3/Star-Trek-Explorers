use std::fmt::{Display, Formatter};

pub enum StarClassification {
    ClassA,
    ClassB,
    ClassF,
    ClassG,
    ClassK,
    ClassM,
    ClassO,
}

impl Display for StarClassification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StarClassification::ClassA => {
                write!(f, "Class A: .")
            }
            StarClassification::ClassB => {
                write!(f, "Class B: .")
            }
            StarClassification::ClassF => {
                write!(f, "Class F: .")
            }
            StarClassification::ClassG => {
                write!(f, "Class G: .")
            }
            StarClassification::ClassK => {
                write!(f, "Class K: .")
            }
            StarClassification::ClassM => {
                write!(f, "Class M: .")
            }
            StarClassification::ClassO => {
                write!(f, "Class O: .")
            }
        }
    }
}

#[cfg(test)]
mod star_classification_should {
    use crate::world::star_classification::StarClassification::{ClassA, ClassB, ClassF, ClassG, ClassK, ClassM, ClassO};

    #[test]
    fn describe_a_star_classification() {
        assert_eq!("Class A: .", ClassA.to_string());
        assert_eq!("Class B: .", ClassB.to_string());
        assert_eq!("Class F: .", ClassF.to_string());
        assert_eq!("Class G: .", ClassG.to_string());
        assert_eq!("Class K: .", ClassK.to_string());
        assert_eq!("Class M: .", ClassM.to_string());
        assert_eq!("Class O: .", ClassO.to_string());
    }
}