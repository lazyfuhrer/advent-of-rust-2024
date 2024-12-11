const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl From<SnowKg> for Snowball {
    fn from(snow_kg: SnowKg) -> Self {
        let snowballs = (snow_kg.0 / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball(snowballs)
    }
}

impl From<SnowLb> for Snowball {
    fn from(snow_lb: SnowLb) -> Self {
        let snowballs = (snow_lb.0 / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball(snowballs)
    }
}

// 2. Implement the same for SnowLb

fn main() {
    let snow_kg = SnowKg::new(5.0);
    let snowballs_from_kg: Snowball = snow_kg.into();
    println!("{} kg of snow can make {} snowballs", snow_kg.0, snowballs_from_kg.0);

    let snow_lb = SnowLb::new(10.0);
    let snowballs_from_lb: Snowball = snow_lb.into();
    println!("{} lb of snow can make {} snowballs", snow_lb.0, snowballs_from_lb.0);
}