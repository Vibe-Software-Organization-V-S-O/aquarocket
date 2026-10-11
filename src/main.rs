mod physics;
mod geometry;

use std::f32::consts::PI;
use geometry::builder::Rocket3DBuilder;
use physics::aerodynamics::{NoseCone, TrapezoidalFinSet};
use physics::motor::SolidMotor;
use physics::recovery::Parachute;
use physics::trajectory::RocketVehicle;

fn main() {
    println!("==================================================");
    println!("   AquaRocket: 3D CAD & RK4 Integrated Pipeline   ");
    println!("==================================================");

    // 1. Dhimmoota Ji'oomeetirii fi Diizaayinii (Parametric Dimensions)
    let radius: f32 = 0.025;         // Raadiyeesii qaamaa (25 mm)
    let nose_len: f32 = 0.30;        // Dheerina qoffee (30 cm)
    let body_len: f32 = 0.65;        // Dheerina qaamaa (65 cm)
    let fin_thickness: f32 = 0.003;  // Furdiina baallee (3 mm)
    let fin_count: u32 = 4;          // Baay'ina baallee
    let fin_span: f32 = 0.08;         // Dheerina baallee (span)
    let fin_root: f32 = 0.12;         // Dheerina hundee baallee (root chord)
    let fin_tip: f32 = 0.06;          // Dheerina fiixee baallee (tip chord)
    let fin_sweep: f32 = 0.04;        // Sweep

    // Bal'ina qaxxaamuraa ji'oomeetirii irraa shallagame (Cross-sectional Area)
    let cross_section_area = PI * radius * radius;

    // 2. Ji'oomeetirii 3D Uumuu fi Faayiliitti Baasuu (Export 3D Solid Model)
    println!("\n[1. Ji'oomeetirii 3D Uumuu fi Baasuu...]");
    let mesh = Rocket3DBuilder::build_solid_vehicle(
        nose_len,
        body_len,
        radius,
        fin_count,
        fin_span,
        fin_root,
        fin_thickness,
        32,
    );

    let obj_filename = "rocket_integrated.obj";
    let mtl_filename = "rocket_integrated.mtl";
    mesh.export_obj_with_mtl(obj_filename, mtl_filename)
        .expect("Faayilii 3D baasuu irratti dogoggorri uumame!");

    println!("  ★ Faayilii 3D milkaa'inaan baafame: {} fi {}", obj_filename, mtl_filename);
    println!("  ★ Baay'ina Vertices: {} | Triangles: {}", mesh.vertices.len(), mesh.indices.len());

    // 3. Eerodaayinaamiksii fi Tasgabbii (Aerodynamics & Barrowman Stability)
    let nose = NoseCone { length: nose_len, base_radius: radius };
    let fins = TrapezoidalFinSet {
        fin_count,
        root_chord: fin_root,
        tip_chord: fin_tip,
        span: fin_span,
        mid_chord_sweep: fin_sweep,
        body_radius: radius,
        fin_root_position: body_len,
    };

    let total_cna = nose.cna() + fins.cna();
    let total_cp = (nose.center_of_pressure() * nose.cna() + fins.center_of_pressure() * fins.cna()) / total_cna;

    println!("\n[2. Xiinxala Eerodaayinaamiksii fi Tasgabbii]");
    println!("  -> Bal'ina Qaxxaamuraa (A) : {:.6} m^2", cross_section_area);
    println!("  -> Bakka Dhiibbaa (CP)       : {:.4} m", total_cp);
    println!("  -> Ko'efisiyantii Humnichaa (CNa): {:.2}", total_cna);

    // 4. Mooteera fi Sirna Deebisanii Fudhachuu (Motor & Recovery)
    let motor = SolidMotor {
        burn_time: 1.6,
        total_impulse: 16.8,
        propellant_mass: 0.025,
    };

    let parachute = Parachute::new(0.35);

    let mut rocket = RocketVehicle {
        dry_mass: 0.120,
        cross_section_area,
        drag_coefficient: 0.45,
        motor,
        parachute,
    };

    // 5. Simuleeshinii Balallii RK4 Adeemsisuu
    println!("\n[3. Simuleeshinii Balallii (RK4 Integration) Hojjechiisuu...]");
    let telem = rocket.simulate(0.01);

    println!("\n================ Odeeffannoo Balallii (Telemetry) ================");
    println!("  ★ Olka'iinsa Olaanaa (Apogee)  : {:.2} meters", telem.apogee);
    println!("  ★ Saffisa Olaanaa Ol-ka'uu     : {:.2} m/s ({:.2} km/h)", 
             telem.max_velocity, telem.max_velocity * 3.6);
    println!("  ★ Olka'iinsa Mooteerri Dhaame  : {:.2} meters", telem.burnout_altitude);
    println!("  ★ Yeroo Apogee Geessu          : {:.2} seconds", telem.time_to_apogee);
    println!("  ★ Yeroo Balallii Guutuu        : {:.2} seconds", telem.total_flight_time);
    println!("  ★ Saffisa Lafarra Bu'iinsaa    : {:.2} m/s ({:.2} km/h)", 
             telem.landing_velocity, telem.landing_velocity * 3.6);
    println!("==================================================================");

    if telem.landing_velocity < 6.0 {
        println!("🚀 Ergamni Milkaa'eera! Lafarra bu'iinsi nageenya qaba (< 6.0 m/s).");
    } else {
        println!("⚠️ Akeekkachiisa: Parashuutii bal'isuun barbaachisaa dha!");
    }
}
