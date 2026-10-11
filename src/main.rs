mod physics;

use physics::aerodynamics::{NoseCone, TrapezoidalFinSet};
use physics::motor::SolidMotor;
use physics::recovery::Parachute;
use physics::trajectory::RocketVehicle;

fn main() {
    println!("==================================================");
    println!("     AquaRocket 100% Pure Rust Mission Engine     ");
    println!("==================================================");

    let radius = 0.025;
    let area = std::f32::consts::PI * radius * radius;
    let nose = NoseCone { length: 0.30, base_radius: radius };
    let fins = TrapezoidalFinSet {
        fin_count: 4,
        root_chord: 0.12,
        tip_chord: 0.06,
        span: 0.08,
        mid_chord_sweep: 0.04,
        body_radius: radius,
        fin_root_position: 0.85,
    };

    let motor = SolidMotor {
        burn_time: 1.6,
        total_impulse: 16.8,
        propellant_mass: 0.025,
    };

    let chute = Parachute::new(0.35); // ৩৫ সেমি প্যারাসুট

    let mut rocket = RocketVehicle {
        dry_mass: 0.120,
        cross_section_area: area,
        drag_coefficient: 0.45,
        motor,
        parachute: chute,
    };

    println!("[Vehicle Info]");
    println!("  -> Body Radius    : {:.3} m", radius);
    println!("  -> Fin Span       : {:.3} m", fins.span);
    println!("  -> Parachute Area : {:.4} m^2", rocket.parachute.area());
    println!("  -> Total Dry Mass : {:.3} kg", rocket.dry_mass);

    println!("\n[Simulating Full Mission Profile...]");
    let telem = rocket.simulate(0.01);

    println!("\n================ Flight Telemetry ================");
    println!("  ★ Apogee (সর্বোচ্চ উচ্চতা)  : {:.2} meters", telem.apogee);
    println!("  ★ Max Ascent Velocity       : {:.2} m/s ({:.2} km/h)", 
             telem.max_velocity, telem.max_velocity * 3.6);
    println!("  ★ Burnout Altitude          : {:.2} meters", telem.burnout_altitude);
    println!("  ★ Time to Apogee            : {:.2} seconds", telem.time_to_apogee);
    println!("  ★ Total Mission Duration    : {:.2} seconds", telem.total_flight_time);
    println!("  ★ Touchdown / Impact Speed  : {:.2} m/s ({:.2} km/h)", 
             telem.landing_velocity, telem.landing_velocity * 3.6);
    println!("==================================================");

    if telem.landing_velocity < 6.0 {
        println!("🚀 মিশন সফল! নিরাপদ ল্যান্ডিং বেগ (< 6.0 m/s)।");
    } else {
        println!("⚠️ সতর্কবার্তা: প্যারাসুট ছোট হওয়ায় ল্যান্ডিং বেশ শক্ত হবে!");
    }
}
