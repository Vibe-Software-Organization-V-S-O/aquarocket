use glam::{Quat, Vec3};
use std::f32::consts::PI;
use crate::physics::aerodynamics::{BarrowmanSolver};
use crate::physics::flight6dof::{State6DOF, OpenRocketStyleSim, MassProperties, AeroCoefficients};

#[repr(C)]
pub struct CTelemetry6DOF {
    pub apogee: f32,
    pub max_velocity: f32,
    pub max_acceleration: f32,
    pub cp_location: f32,
    pub cg_location: f32,
    pub static_margin: f32,
    pub drift_distance: f32,
    pub flight_time: f32,
}

#[repr(C)]
pub struct CRocketDesignConfig {
    pub caliber: f32,           // ব্যাস (m)
    pub total_length: f32,      // মোট দৈর্ঘ্য (m)
    pub dry_mass: f32,          // স্ট্রাকচারাল ভর (kg)
    pub cg_dry: f32,            // সিজি অবস্থান (m)
    pub fin_count: u32,
    pub fin_root_chord: f32,
    pub fin_tip_chord: f32,
    pub fin_span: f32,
    pub fin_sweep_length: f32,
    pub fin_position: f32,
    pub motor_impulse: f32,     // মোট ইমপালস (N-s)
    pub motor_burn_time: f32,   // বার্ন টাইম (s)
    pub motor_propellant_mass: f32,
    pub wind_speed_ms: f32,     // ক্রসউইন্ড গতি (m/s)
}

#[no_mangle]
pub extern "C" fn aquarocket_compute_flight_6dof(cfg: CRocketDesignConfig) -> CTelemetry6DOF {
    let r = cfg.caliber * 0.5;
    let ref_area = PI * r * r;

    let nose_comp = BarrowmanSolver::solve_nose_cone(cfg.caliber * 3.5, r);
    let fin_comp = BarrowmanSolver::solve_trapezoidal_fins(
        cfg.fin_count as usize,
        cfg.fin_root_chord,
        cfg.fin_tip_chord,
        cfg.fin_span,
        cfg.fin_sweep_length,
        r,
        cfg.fin_position,
    );

    let components = vec![nose_comp, fin_comp];
    let (cp, static_margin) = BarrowmanSolver::compute_static_margin(&components, cfg.cg_dry, cfg.caliber);

    let sim = OpenRocketStyleSim {
        mass_props: MassProperties {
            dry_mass: cfg.dry_mass,
            cg_distance: cfg.cg_dry,
            inertia_tensor: Vec3::new(
                0.5 * cfg.dry_mass * r.powi(2),
                (1.0 / 12.0) * cfg.dry_mass * cfg.total_length.powi(2),
                (1.0 / 12.0) * cfg.dry_mass * cfg.total_length.powi(2),
            ),
        },
        aero_props: AeroCoefficients {
            cd0: 0.42,
            reference_area: ref_area,
            reference_length: cfg.caliber,
        },
        components,
    };

    let mut state = State6DOF {
        position: Vec3::new(0.0, 0.0, 0.0),
        velocity: Vec3::new(0.0, 0.0, 0.0),
        orientation: Quat::from_rotation_y(-PI / 2.0), // উল্লম্বভাবে লঞ্চপ্যাডে সেট
        angular_velocity: Vec3::ZERO,
    };

    let wind = Vec3::new(cfg.wind_speed_ms, 0.0, 0.0);
    let dt = 0.005;
    let mut t = 0.0;
    let mut max_alt = 0.0;
    let mut max_vel = 0.0;
    let mut max_acc = 0.0;

    let avg_thrust = cfg.motor_impulse / cfg.motor_burn_time;

    while (-state.position.z >= 0.0 || t == 0.0) && t < 120.0 {
        let (thrust, prop_m) = if t <= cfg.motor_burn_time {
            let p_rem = cfg.motor_propellant_mass * (1.0 - t / cfg.motor_burn_time);
            (avg_thrust, p_rem)
        } else {
            (0.0, 0.0)
        };

        let next_state = sim.rk4_step(&state, thrust, prop_m, wind, dt);
        let acc = (next_state.velocity - state.velocity).length() / dt;

        state = next_state;
        t += dt;

        let alt = -state.position.z;
        let vel = state.velocity.length();

        if alt > max_alt { max_alt = alt; }
        if vel > max_vel { max_vel = vel; }
        if acc > max_acc { max_acc = acc; }

        if state.velocity.z > 0.0 && t > cfg.motor_burn_time {
            break; // অ্যাপোজি হিট
        }
    }

    CTelemetry6DOF {
        apogee: max_alt,
        max_velocity: max_vel,
        max_acceleration: max_acc,
        cp_location: cp,
        cg_location: cfg.cg_dry,
        static_margin,
        drift_distance: state.position.x.hypot(state.position.y),
        flight_time: t,
    }
}

use crate::geometry::builder::DynamicRocketCAD;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn aquarocket_generate_cad_mesh(
    output_path: *const c_char,
    caliber: f32,
    nose_len: f32,
    body_len: f32,
    fin_root: f32,
    fin_tip: f32,
    fin_span: f32,
    fin_sweep: f32,
    fin_count: u32,
) -> bool {
    if output_path.is_null() {
        return false;
    }

    let c_str = unsafe { CStr::from_ptr(output_path) };
    let path = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    DynamicRocketCAD::generate_obj_file(
        path,
        caliber,
        nose_len,
        body_len,
        fin_root,
        fin_tip,
        fin_span,
        fin_sweep,
        fin_count,
    ).is_ok()
}
