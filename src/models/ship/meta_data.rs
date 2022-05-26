pub struct MetaData {
    prefix: String,
    name: String,
    suffix: String,
    serial_number: String,
    crew_compliment: u16,
}

impl MetaData {
    pub(crate) fn create(
        prefix: String,
        name: String,
        suffix: String,
        serial_number: String,
        crew_compliment: u16) -> MetaData {
        return MetaData {
            prefix,
            name,
            suffix,
            serial_number,
            crew_compliment,
        };
    }

    pub fn details(self) -> String {
        return [self.prefix.as_str(),
            self.name.as_str(),
            self.suffix.as_str(),
            self.serial_number.as_str()].join(" ");
    }
}

#[cfg(test)]
mod model_meta_data_should {
    use super::*;

    #[test]
    fn create_starship_meta_data() {
        let meta_data = given_meta_data();

        assert_eq!(meta_data.prefix, "USS");
        assert_eq!(meta_data.name, "Enterprise");
        assert_eq!(meta_data.suffix, "E");
        assert_eq!(meta_data.serial_number, "NCC-1701");
        assert_eq!(meta_data.crew_compliment, 2000);
    }

    #[test]
    fn get_starship_meta_data_details() {
        let meta_data = given_meta_data();

        assert_eq!(meta_data.details(), "USS Enterprise E NCC-1701");
    }

    fn given_meta_data() -> MetaData {
        MetaData::create("USS".to_string(),
                         "Enterprise".to_string(),
                         "E".to_string(),
                         "NCC-1701".to_string(),
                         2000)
    }
}
