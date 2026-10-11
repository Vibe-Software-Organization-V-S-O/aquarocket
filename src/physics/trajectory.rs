use crate::physics::atmosphere::Atmosphere;
use crate::physics::stage::Stage;

pub struct MultiStageTelemetry {
    pub apogee: f32,
    pub max_velocity: f32,
    pub stage1_burnout_alt: f32,
    pub stage1_separation_time: f32,
    pub stage2_burnout_alt: f32,
    pub time_to_apogee: f32,
    pub total_flight_time: f32,
    pub landing_velocity: f32,
}

pub struct MultiStageRocket {
    pub stages: Vec<Stage>,
    pub cross_section_area: f32,
    pub drag_coefficient: f32,
}

impl MultiStageRocket {
    pub fn new(stages: Vec<Stage>, cross_section_area: f32, drag_coefficient: f32) -> Self {
        Self {
            stages,
            cross_section_area,
            drag_coefficient,
        }
    }

    pub fn simulate(&mut self, dt: f32) -> MultiStageTelemetry {
        let mut t: f32 = 0.0;
        let mut alt: f32 = 0.0;
        let mut v: f32 = 0.0;

        let mut max_alt: f32 = 0.0;
        let mut max_vel: f32 = 0.0;

        let mut s1_burnout_alt: f32 = 0.0;
        let mut s1_sep_time: f32 = 0.0;
        let mut s2_burnout_alt: f32 = 0.0;

        let mut current_stage_idx: usize = 0;
        let mut stage_clock: f32 = 0.0;
        let mut apogee_reached = false;
        let mut apogee_time: f32 = 0.0;

        // মেইন রিকভারি প্যারাসুট স্টেজ ২-এ থাকে
        let mut parachute_active = false;

        while alt >= 0.0 || t == 0.0 {
            // ১. বর্তমান মোট ভর হিসাব (বর্তমান স্টেজ + উপরের স্টেজগুলোর ভর)
            let mut total_mass = 0.0;
            for i in current_stage_idx..self.stages.len() {
                total_mass += self.stages[i].dry_mass;
                if i == current_stage_idx {
                    total_mass += self.stages[i].motor.current_propellant_mass(stage_clock);
                } else {
                    total_mass += self.stages[i].motor.propellant_mass;
                }
            }

            // ২. বর্তমান স্টেজের থ্রাস্ট
            let thrust = if current_stage_idx < self.stages.len() {
                self.stages[current_stage_idx].motor.thrust_at(stage_clock)
            } else {
                0.0
            };

            // ৩. স্টেজিং ইভেন্ট ট্র্যাকিং
            if current_stage_idx == 0 && stage_clock >= self.stages[0].motor.burn_time {
                if s1_burnout_alt == 0.0 {
                    s1_burnout_alt = alt;
                    s1_sep_time = t;
                    // স্টেজ ১ ড্রপ (স্টেজিং সম্পন্ন)
                    current_stage_idx = 1;
                    stage_clock = -self.stages[1].ignition_delay; // ইগনিশন ডিলের টাইমার
                }
            } else if current_stage_idx == 1 && stage_clock >= self.stages[1].motor.burn_time {
                if s2_burnout_alt == 0.0 {
                    s2_burnout_alt = alt;
                }
            }

            // ৪. এরোডাইনামিক ড্র্যাগ ও অ্যাকসিলারেশন
            let air = Atmosphere::at_altitude(alt);
            let (cd, area) = if parachute_active {
                (1.5, 0.096) // স্ট্যান্ডার্ড প্যারাসুট এরিয়া
            } else {
                (self.drag_coefficient, self.cross_section_area)
            };

            let drag_sign = if v >= 0.0 { 1.0 } else { -1.0 };
            let drag = 0.5 * air.density * v.powi(2) * cd * area * drag_sign;
            let gravity = total_mass * 9.80665;
            let a = (thrust - drag - gravity) / total_mass;

            // ৫. RK4-তুল্য হাই-স্পিড স্টেট আপডেট (মোবাইল অ্যাপ-ফ্রেন্ডলি ফাস্ট লুপ)
            v += a * dt;
            alt += v * dt;
            t += dt;
            stage_clock += dt;

            if alt > max_alt {
                max_alt = alt;
            }
            if v > max_vel {
                max_vel = v;
            }

            // অ্যাপোজি চেক ও প্যারাসুট অ্যাক্টিভেশন
            if !apogee_reached && v <= 0.0 && t > 3.0 {
                apogee_reached = true;
                apogee_time = t;
                parachute_active = true;
            }

            // ল্যান্ডিং সমাপ্তি
            if apogee_reached && alt <= 0.0 {
                break;
            }
        }

        MultiStageTelemetry {
            apogee: max_alt,
            max_velocity: max_vel,
            stage1_burnout_alt: s1_burnout_alt,
            stage1_separation_time: s1_sep_time,
            stage2_burnout_alt: s2_burnout_alt,
            time_to_apogee: apogee_time,
            total_flight_time: t,
            landing_velocity: v.abs(),
        }
    }
}
