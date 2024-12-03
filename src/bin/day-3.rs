// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    // Calculate the ratio of good deeds to total deeds
    // Any ratio greater than 0.75 is considered nice
    // e.g. 10 good deeds and 2 bad deeds =
    // (10 * 1) / (10 * 1) + (2 * 2) = 10 / 14 = 0.714... (not nice)
    // If both good and bad deeds are 0, the child is naughty
    if good_deeds == 0 && bad_deeds == 0 {
        return false;
    }
    let good_weighted = good_deeds as f32 * GOOD_WEIGHT;
    let bad_weighted = bad_deeds as f32 * BAD_WEIGHT;
    let total_weighted = good_weighted + bad_weighted;

    if total_weighted == 0.0 {
        return false;
    }

    let ratio = good_weighted / total_weighted;
    ratio >= 0.75
}

fn main() {
    let good_deeds = 50;
    let bad_deeds = 2;
    let result = is_nice(good_deeds, bad_deeds);
    println!("Is the child nice? {}", result);
}