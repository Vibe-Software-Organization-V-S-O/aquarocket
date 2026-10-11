use crate::physics::motor::SolidMotor;
use crate::physics::recovery::Parachute;
use crate::physics::stage::Stage;
use crate::physics::trajectory::MultiStageRocket;
use std::f32::consts::PI;

/// C-Compatible Telemetry Output Struct
#[repr(C)]
pub struct CRocketTelemetry {
    pub apogee: f32,
    pub max_velocity: f32,
    pub stage1_burnout_alt: f32,
    pub stage1_separation_time: f32,
    pub stage2_burnout_alt: f32,
    pub time_to_apogee: f32,
    pub total_flight_time: f32,
    pub landing_velocity: f32,
}

/// C-Compatible Input Configuration
#[repr(C)]
pub struct CRocketConfig {
    pub radius: f32,
    pub drag_coeff: f32,
    // Stage 1 (Booster)
    pub s1_dry_mass: f32,
    pub s1_burn_time: f32,
    pub s1_impulse: f32,
    pub s1_propellant_mass: f32,
    // Stage 2 (Sustainer)
    pub s2_dry_mass: f32,
    pub s2_burn_time: f32,
    pub s2_impulse: f32,
    pub s2_propellant_mass: f32,
    pub s2_ignition_delay: f32,
    pub parachute_diameter: f32,
}

/// FFI Export: সম্পূর্ণ মাল্টি-স্টেজ সি-ফাংশন
#[no_mangle]
pub extern "C" fn aquarocket_simulate_flight(config: CRocketConfig) -> CRocketTelemetry {
    let area = PI * config.radius * config.radius;

    let booster_motor = SolidMotor {
        burn_time: config.s1_burn_time,
        total_impulse: config.s1_impulse,
        propellant_mass: config.s1_propellant_mass,
    };
    let stage1 = Stage::new(1, config.s1_dry_mass, booster_motor, None, 0.0);

    let sustainer_motor = SolidMotor {
        burn_time: config.s2_burn_time,
        total_impulse: config.s2_impulse,
        propellant_mass: config.s2_propellant_mass,
    };
    let chute = Parachute::new(config.parachute_diameter);
    let stage2 = Stage::new(2, config.s2_dry_mass, sustainer_motor, Some(chute), config.s2_ignition_delay);

    let mut rocket = MultiStageRocket::new(vec![stage1, stage2], area, config.drag_coeff);
    let telem = rocket.simulate(0.005);

    CRocketTelemetry {
        apogee: telem.apogee,
        max_velocity: telem.max_velocity,
        stage1_burnout_alt: telem.stage1_burnout_alt,
        stage1_separation_time: telem.stage1_separation_time,
        stage2_burnout_alt: telem.stage2_burnout_alt,
        time_to_apogee: telem.time_to_apogee,
        total_flight_time: telem.total_flight_time,
        landing_velocity: telem.landing_velocity,
    }
}
