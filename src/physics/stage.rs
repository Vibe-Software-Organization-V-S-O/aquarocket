use crate::physics::motor::SolidMotor;
use crate::physics::recovery::Parachute;

pub struct Stage {
    pub stage_number: u8,
    pub dry_mass: f32,       // এই স্টেজের নিজস্ব স্ট্রাকচারাল ভর (kg)
    pub motor: SolidMotor,   // এই স্টেজের মোটর
    pub parachute: Option<Parachute>, // স্টেজের প্যারাসুট (থাকলে)
    pub ignition_delay: f32, // পূর্ববর্তী স্টেজ শেষ হওয়ার পর কত সেকেন্ড পর জ্বলবে
}

impl Stage {
    pub fn new(stage_number: u8, dry_mass: f32, motor: SolidMotor, parachute: Option<Parachute>, ignition_delay: f32) -> Self {
        Self {
            stage_number,
            dry_mass,
            motor,
            parachute,
            ignition_delay,
        }
    }
}
