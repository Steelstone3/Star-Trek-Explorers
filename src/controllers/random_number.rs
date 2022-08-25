use rand::Rng;

pub fn get_random_number_from_range(lower_range: u64, upper_range: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower_range..upper_range)
}