pub struct SolidMotor {
    pub burn_time: f32,       // মোট বার্ন টাইম (সেকেন্ড)
    pub total_impulse: f32,   // টোটাল ইম্পালস (N·s)
    pub propellant_mass: f32, // জ্বালানির ভর (kg)
}

impl SolidMotor {
    // বর্তমান সময়ে মোটর থ্রাস্ট (নিউটনে)
    pub fn thrust_at(&self, t: f32) -> f32 {
        if t >= 0.0 && t <= self.burn_time {
            self.total_impulse / self.burn_time
        } else {
            0.0
        }
    }

    // বর্তমান সময়ে মোটরের অবশিষ্ট জ্বালানির ভর
    pub fn current_propellant_mass(&self, t: f32) -> f32 {
        if t <= 0.0 {
            self.propellant_mass
        } else if t >= self.burn_time {
            0.0
        } else {
            self.propellant_mass * (1.0 - t / self.burn_time)
        }
    }
}
