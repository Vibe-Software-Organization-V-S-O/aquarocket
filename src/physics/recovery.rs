pub struct Parachute {
    pub diameter: f32,       // প্যারাসুটের ব্যাস (মিটার)
    pub drag_coefficient: f32, // সাধারণত ১.৫
    pub deployed: bool,
}

impl Parachute {
    pub fn new(diameter: f32) -> Self {
        Self {
            diameter,
            drag_coefficient: 1.5,
            deployed: false,
        }
    }

    pub fn area(&self) -> f32 {
        let radius = self.diameter / 2.0;
        std::f32::consts::PI * radius * radius
    }
}
