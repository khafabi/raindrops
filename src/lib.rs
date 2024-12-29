pub fn raindrops(n: u32) -> String {
    fn factor_to_sound(n: u32, factor: u32, sound: &str) -> Option<&str> {
        if n % factor == 0 {
            return Some(sound);
        }
        None
    }

    let factors = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let sounds: Vec<&str> = factors
        .iter()
        .filter_map(|&(factor, sound)| factor_to_sound(n, factor, sound))
        .collect();

    if sounds.is_empty() {
        return n.to_string();
    }

    sounds.concat()
}
