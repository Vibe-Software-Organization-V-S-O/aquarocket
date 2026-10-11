use crate::physics::atmosphere::Atmosphere;
use crate::physics::motor::SolidMotor;
use crate::physics::recovery::Parachute;

pub struct FlightTelemetry {
    pub apogee: f32,
    pub max_velocity: f32,
    pub burnout_altitude: f32,
    pub time_to_apogee: f32,
    pub total_flight_time: f32,
    pub landing_velocity: f32,
}

pub struct RocketVehicle {
    pub dry_mass: f32,
    pub cross_section_area: f32,
    pub drag_coefficient: f32,
    pub motor: SolidMotor,
    pub parachute: Parachute,
}

impl RocketVehicle {
    fn acceleration(&self, t: f32, alt: f32, v: f32, parachute_active: bool) -> f32 {
        let air = Atmosphere::at_altitude(alt);
        let mass = self.dry_mass + self.motor.current_propellant_mass(t);
        let thrust = self.motor.thrust_at(t);
        
        let (cd, area) = if parachute_active {
            (self.parachute.drag_coefficient, self.parachute.area())
        } else {
            (self.drag_coefficient, self.cross_section_area)
        };

        let drag_sign = if v >= 0.0 { 1.0 } else { -1.0 };
        let drag = 0.5 * air.density * v.powi(2) * cd * area * drag_sign;
        let gravity = mass * 9.80665;

        (thrust - drag - gravity) / mass
    }

    pub fn simulate(&mut self, dt: f32) -> FlightTelemetry {
        let mut t = 0.0;
        let mut alt = 0.0;
        let mut v = 0.0;

        let mut max_alt = 0.0;
        let mut max_vel = 0.0;
        let mut burnout_alt = 0.0;
        let mut apogee_time = 0.0;
        let mut apogee_reached = false;

        while alt >= 0.0 || t == 0.0 {
            if (t - self.motor.burn_time).abs() < dt {
                burnout_alt = alt;
            }

            // অ্যাপোজি ডিটেকশন ও প্যারাসুট ডিপ্লয়মেন্ট
            if !apogee_reached && v <= 0.0 && t > self.motor.burn_time {
                apogee_reached = true;
                apogee_time = t;
                self.parachute.deployed = true;
            }

            // RK4 স্টেপ
            let is_chute = self.parachute.deployed;
            let a1 = self.acceleration(t, alt, v, is_chute);
            let v1 = v;

            let a2 = self.acceleration(t + 0.5 * dt, alt + 0.5 * v1 * dt, v + 0.5 * a1 * dt, is_chute);
            let v2 = v + 0.5 * a1 * dt;

            let a3 = self.acceleration(t + 0.5 * dt, alt + 0.5 * v2 * dt, v + 0.5 * a2 * dt, is_chute);
            let v3 = v + 0.5 * a2 * dt;

            let a4 = self.acceleration(t + dt, alt + v3 * dt, v + a3 * dt, is_chute);
            let v4 = v + a3 * dt;

            alt += (dt / 6.0) * (v1 + 2.0 * v2 + 2.0 * v3 + v4);
            v += (dt / 6.0) * (a1 + 2.0 * a2 + 2.0 * a3 + a4);
            t += dt;

            if alt > max_alt {
                max_alt = alt;
            }
            if v > max_vel {
                max_vel = v;
            }

            // ল্যান্ডিং চেকিং
            if apogee_reached && alt <= 0.0 {
                alt = 0.0;
                break;
            }
        }

        FlightTelemetry {
            apogee: max_alt,
            max_velocity: max_vel,
            burnout_altitude: burnout_alt,
            time_to_apogee: apogee_time,
            total_flight_time: t,
            landing_velocity: v.abs(),
        }
    }
}
