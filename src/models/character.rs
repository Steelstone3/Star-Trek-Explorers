pub struct CrewMember {
    name: String,
    rank: String,
    role: String,
}

impl CrewMember {
    pub fn create_crew_member(the_name: String, the_rank: String, the_role: String) -> CrewMember {
        CrewMember {
            name: the_name,
            rank: the_rank,
            role: the_role,
        }
    }

    pub fn details(self) -> String {
        return [self.name, self.rank, self.role].join(" ");
    }
}

#[cfg(test)]
mod model_character_should {
    use super::*;

    #[test]
    fn create_a_crew_member() {
        let a_crew_member = CrewMember::create_crew_member("Bob".to_string(),
                                                           "Captain".to_string(),
                                                           "Captain".to_string());

        assert_eq!(a_crew_member.name, "Bob");
        assert_eq!(a_crew_member.rank, "Captain");
        assert_eq!(a_crew_member.role, "Captain");
    }

    #[test]
    fn provide_character_details() {
        let dave = CrewMember {
            name: "Dave".to_string(),
            rank: "Captain".to_string(),
            role: "Captain".to_string(),
        };

        let details = dave.details();

        assert_eq!(details, "Dave Captain Captain");
    }
}