use std::fmt::{Display, Formatter};

pub enum PlanetClassification {
    ClassD,
    ClassH,
    ClassJ,
    ClassK,
    ClassL,
    ClassM,
    ClassN,
    ClassR,
    ClassT,
    ClassY
}

impl Display for PlanetClassification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetClassification::ClassD => {
                write!(f, "Class D: An uninhabitable planetoid, moon, or small planet with little to no atmosphere. Some are viable candidates for terra-forming.")
            }
            PlanetClassification::ClassH => {
                write!(f, "Class H: Generally uninhabitable for Humans, though viable for Sheliak.")
            }
            PlanetClassification::ClassJ => {
                write!(f, "Class J: A type of gas giant.")
            }
            PlanetClassification::ClassK => {
                write!(f, "Class K: Adaptable for Humans by use of artificial biospheres.")
            }
            PlanetClassification::ClassL=> {
                write!(f, "Class L: Marginally habitable, with vegetation but usually no animal life.")
            }
            PlanetClassification::ClassM=> {
                write!(f, "Class M: Earth-like, with atmospheres containing oxygen and, typically, nucleogenic particles. Largely habitable for humanoid life forms.")
            }
            PlanetClassification::ClassN=> {
                write!(f, "Class N: Classified.")
            }
            PlanetClassification::ClassR=> {
                write!(f, "Class R: Unknown Stellar Body.")
            }
            PlanetClassification::ClassT=> {
                write!(f, "Class T: A type of gas giant.")
            }
            PlanetClassification::ClassY=> {
                write!(f, "Class Y: A world with a toxic atmosphere and surface temperatures exceeding 500 Kelvin. Prone to thermionic radiation discharges.")
            }
        }
    }
}

#[cfg(test)]
mod planet_classification_should {
    use crate::world::planet_classification::PlanetClassification::{ClassD, ClassH, ClassJ, ClassK, ClassL, ClassM, ClassN, ClassR, ClassT, ClassY};

    #[test]
    fn describe_a_planet_classification() {
        assert_eq!("Class D: An uninhabitable planetoid, moon, or small planet with little to no atmosphere. Some are viable candidates for terra-forming.", ClassD.to_string());
        assert_eq!("Class H: Generally uninhabitable for Humans, though viable for Sheliak.", ClassH.to_string());
        assert_eq!("Class J: A type of gas giant.", ClassJ.to_string());
        assert_eq!("Class K: Adaptable for Humans by use of artificial biospheres.", ClassK.to_string());
        assert_eq!("Class L: Marginally habitable, with vegetation but usually no animal life.", ClassL.to_string());
        assert_eq!("Class M: Earth-like, with atmospheres containing oxygen and, typically, nucleogenic particles. Largely habitable for humanoid life forms.", ClassM.to_string());
        assert_eq!("Class N: Classified.", ClassN.to_string());
        assert_eq!("Class R: Unknown Stellar Body.", ClassR.to_string());
        assert_eq!("Class T: A type of gas giant.", ClassT.to_string());
        assert_eq!("Class Y: A world with a toxic atmosphere and surface temperatures exceeding 500 Kelvin. Prone to thermionic radiation discharges.", ClassY.to_string());
    }
}