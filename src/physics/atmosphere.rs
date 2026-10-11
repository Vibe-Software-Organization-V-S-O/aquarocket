/// ইন্টারন্যাশনাল স্ট্যান্ডার্ড অ্যাটমোস্ফিয়ার (ISA) মডেল
pub struct Atmosphere;

pub struct AirState {
    pub temperature: f32, // কেলভিন (K)
    pub pressure: f32,    // প্যাসকেল (Pa)
    pub density: f32,     // কেজি/মি^৩ (kg/m^3)
    pub speed_of_sound: f32, // মি/সে (m/s)
}

impl Atmosphere {
    // সমুদ্রপৃষ্ঠের আদর্শ মান
    const T0: f32 = 288.15;
    const P0: f32 = 101325.0;
    const _RHO0: f32 = 1.225;
    const L: f32 = 0.0065;   // ট্রপোস্ফিয়ারের ল্যাপস রেট (K/m)
    const G: f32 = 9.80665;
    const R: f32 = 287.05;   // বাতাসের গ্যাস কনস্ট্যান্ট
    const GAMMA: f32 = 1.4;  // অ্যাডিয়াবেটিক ইনডেক্স

    pub fn at_altitude(altitude: f32) -> AirState {
        let alt = altitude.clamp(0.0, 11000.0); // ট্রপোস্ফিয়ার রেঞ্জ
        
        let t = Self::T0 - Self::L * alt;
        let p = Self::P0 * (1.0 - (Self::L * alt) / Self::T0).powf((Self::G) / (Self::R * Self::L));
        let rho = p / (Self::R * t);
        let sound_speed = (Self::GAMMA * Self::R * t).sqrt();

        AirState {
            temperature: t,
            pressure: p,
            density: rho,
            speed_of_sound: sound_speed,
        }
    }
}
