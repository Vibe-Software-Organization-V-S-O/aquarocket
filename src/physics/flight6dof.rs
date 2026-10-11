use glam::{Mat3, Quat, Vec3};
use crate::physics::atmosphere::Atmosphere;
use crate::physics::aerodynamics::{BarrowmanSolver, BarrowmanComponent};

#[derive(Clone, Copy, Debug)]
pub struct State6DOF {
    pub position: Vec3,
    pub velocity: Vec3,
    pub orientation: Quat,
    pub angular_velocity: Vec3,
}

pub struct AeroCoefficients {
    pub cd0: f32,
    pub reference_area: f32,
    pub reference_length: f32,
}

pub struct MassProperties {
    pub dry_mass: f32,
    pub cg_distance: f32,
    pub inertia_tensor: Vec3,
}

pub struct OpenRocketStyleSim {
    pub mass_props: MassProperties,
    pub aero_props: AeroCoefficients,
    pub components: Vec<BarrowmanComponent>,
}

impl OpenRocketStyleSim {
    pub fn compute_derivatives(
        &self,
        state: &State6DOF,
        thrust: f32,
        propellant_mass: f32,
        wind_velocity: Vec3,
    ) -> (Vec3, Vec3, Quat, Vec3) {
        let current_mass = self.mass_props.dry_mass + propellant_mass;
        let altitude = -state.position.z;
        let air = Atmosphere::at_altitude(altitude.max(0.0));

        let v_rel = state.velocity - wind_velocity;
        let rot_matrix = Mat3::from_quat(state.orientation);
        let v_body = rot_matrix.transpose() * v_rel;
        let v_mag = v_body.length();

        let f_thrust_body = Vec3::new(thrust, 0.0, 0.0);

        let alpha = if v_mag > 1e-3 {
            (v_body.y.hypot(v_body.z) / v_body.x.abs().max(1e-3)).atan()
        } else {
            0.0
        };

        let q = 0.5 * air.density * v_mag.powi(2);
        let drag_force = q * self.aero_props.reference_area * (self.aero_props.cd0 + 1.2 * alpha.powi(2));

        let (cp, _) = BarrowmanSolver::compute_static_margin(
            &self.components,
            self.mass_props.cg_distance,
            self.aero_props.reference_length,
        );

        let mut normal_force = 0.0;
        for c in &self.components {
            normal_force += q * self.aero_props.reference_area * c.c_na * alpha;
        }

        let f_aero_body = Vec3::new(
            -drag_force,
            -normal_force * (v_body.y / (v_mag + 1e-4)),
            -normal_force * (v_body.z / (v_mag + 1e-4)),
        );
        let total_force_body = f_thrust_body + f_aero_body;
        let total_force_inertial = rot_matrix * total_force_body + Vec3::new(0.0, 0.0, current_mass * 9.80665);

        let d_vel = total_force_inertial / current_mass;
        let d_pos = state.velocity;

        let restoring_arm = cp - self.mass_props.cg_distance;
        let pitch_torque = normal_force * restoring_arm;
        let damping_factor = 0.55 * q * self.aero_props.reference_area * self.aero_props.reference_length.powi(2) / (v_mag + 1.0);

        let total_torque_body = Vec3::new(
            -state.angular_velocity.x * damping_factor * 0.1,
            pitch_torque - state.angular_velocity.y * damping_factor,
            pitch_torque - state.angular_velocity.z * damping_factor,
        );

        let d_omega = Vec3::new(
            total_torque_body.x / self.mass_props.inertia_tensor.x,
            total_torque_body.y / self.mass_props.inertia_tensor.y,
            total_torque_body.z / self.mass_props.inertia_tensor.z,
        );

        let omega_quat = Quat::from_xyzw(
            state.angular_velocity.x,
            state.angular_velocity.y,
            state.angular_velocity.z,
            0.0,
        );
        let d_quat = (state.orientation * omega_quat) * 0.5;

        (d_pos, d_vel, d_quat, d_omega)
    }

    pub fn rk4_step(
        &self,
        state: &State6DOF,
        thrust: f32,
        propellant_mass: f32,
        wind: Vec3,
        dt: f32,
    ) -> State6DOF {
        let (k1_dp, k1_dv, k1_dq, k1_dw) = self.compute_derivatives(state, thrust, propellant_mass, wind);

        let s2 = State6DOF {
            position: state.position + k1_dp * (0.5 * dt),
            velocity: state.velocity + k1_dv * (0.5 * dt),
            orientation: (state.orientation + k1_dq * (0.5 * dt)).normalize(),
            angular_velocity: state.angular_velocity + k1_dw * (0.5 * dt),
        };
        let (k2_dp, k2_dv, k2_dq, k2_dw) = self.compute_derivatives(&s2, thrust, propellant_mass, wind);

        let s3 = State6DOF {
            position: state.position + k2_dp * (0.5 * dt),
            velocity: state.velocity + k2_dv * (0.5 * dt),
            orientation: (state.orientation + k2_dq * (0.5 * dt)).normalize(),
            angular_velocity: state.angular_velocity + k2_dw * (0.5 * dt),
        };
        let (k3_dp, k3_dv, k3_dq, k3_dw) = self.compute_derivatives(&s3, thrust, propellant_mass, wind);

        let s4 = State6DOF {
            position: state.position + k3_dp * dt,
            velocity: state.velocity + k3_dv * dt,
            orientation: (state.orientation + k3_dq * dt).normalize(),
            angular_velocity: state.angular_velocity + k3_dw * dt,
        };
        let (k4_dp, k4_dv, k4_dq, k4_dw) = self.compute_derivatives(&s4, thrust, propellant_mass, wind);

        State6DOF {
            position: state.position + (k1_dp + k2_dp * 2.0 + k3_dp * 2.0 + k4_dp) * (dt / 6.0),
            velocity: state.velocity + (k1_dv + k2_dv * 2.0 + k3_dv * 2.0 + k4_dv) * (dt / 6.0),
            orientation: (state.orientation + (k1_dq + k2_dq * 2.0 + k3_dq * 2.0 + k4_dq) * (dt / 6.0)).normalize(),
            angular_velocity: state.angular_velocity + (k1_dw + k2_dw * 2.0 + k3_dw * 2.0 + k4_dw) * (dt / 6.0),
        }
    }
}
