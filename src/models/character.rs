pub struct CrewMember {
    name: String,
    rank: String,
    role: String,
}

pub fn create(the_name: String, the_rank: String, the_role: String) -> CrewMember {
    CrewMember {
        name: the_name,
        rank: the_rank,
        role: the_role,
    }
}

pub fn details(crew_member: CrewMember) -> String {
    return [crew_member.name.as_str(), crew_member.rank.as_str(), crew_member.role.as_str()].join(" ");
}

#[cfg(test)]
mod model_character_should {
    use super::*;

    #[test]
    fn create_a_character() {
        let a_crew_member = create("Bob".to_string(),
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

        let details = details(dave);

        assert_eq!(details, "Dave Captain Captain");
    }
}