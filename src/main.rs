#![allow(dead_code)]
mod physics;
mod geometry;

use std::f32::consts::PI;
use physics::motor::SolidMotor;
use physics::recovery::Parachute;
use physics::stage::Stage;
use physics::trajectory::MultiStageRocket;

fn main() {
    println!("==================================================");
    println!("   AquaRocket Mobile-Core: Multi-Stage Sim Engine ");
    println!("==================================================");

    let radius = 0.025;
    let area = PI * radius * radius;

    // ১. স্টেজ ১ (Booster Stage) - বেশি পাওয়ারফুল বুস্টার মোটর
    let booster_motor = SolidMotor {
        burn_time: 1.2,
        total_impulse: 20.0,    // ২০ N·s বুস্টার
        propellant_mass: 0.035, // ৩৫ গ্রাম ফুয়েল
    };
    let stage1 = Stage::new(1, 0.080, booster_motor, None, 0.0);

    // ২. স্টেজ ২ (Sustainer Stage) - হালকা ও দীর্ঘস্থায়ী মোটর
    let sustainer_motor = SolidMotor {
        burn_time: 2.0,
        total_impulse: 18.0,    // ১৮ N·s সাসটেইনার
        propellant_mass: 0.028,
    };
    let chute = Parachute::new(0.35);
    let stage2 = Stage::new(2, 0.090, sustainer_motor, Some(chute), 0.5); // ০.৫ সেকেন্ড পর ইগনিশন

    println!("[Vehicle Config]");
    println!("  -> Total Stages      : 2 (Booster + Sustainer)");
    println!("  -> Airframe Diameter : {:.1} mm", radius * 2000.0);
    println!("  -> Stage 1 Dry Mass  : {:.3} kg", stage1.dry_mass);
    println!("  -> Stage 2 Dry Mass  : {:.3} kg", stage2.dry_mass);

    let mut rocket = MultiStageRocket::new(vec![stage1, stage2], area, 0.42);

    println!("\n[Simulating 2-Stage Trajectory & Staging Dynamics...]");
    let telem = rocket.simulate(0.005); // মোবাইল কার্নেল হাই-রেজুলেশন ৫ মিলিসেকেন্ড স্টেপ

    println!("\n================ Multi-Stage Telemetry ================");
    println!("  ★ Apogee (চূড়ান্ত উচ্চতা)     : {:.2} meters", telem.apogee);
    println!("  ★ Max Velocity (সর্বোচ্চ বেগ) : {:.2} m/s ({:.2} km/h)", 
             telem.max_velocity, telem.max_velocity * 3.6);
    println!("  ★ Stage 1 Separation Alt      : {:.2} meters", telem.stage1_burnout_alt);
    println!("  ★ Separation Timestamp        : {:.2} seconds", telem.stage1_separation_time);
    println!("  ★ Stage 2 Burnout Alt         : {:.2} meters", telem.stage2_burnout_alt);
    println!("  ★ Time to Apogee              : {:.2} seconds", telem.time_to_apogee);
    println!("  ★ Total Mission Duration      : {:.2} seconds", telem.total_flight_time);
    println!("  ★ Touchdown Speed             : {:.2} m/s", telem.landing_velocity);
    println!("=======================================================");
}
