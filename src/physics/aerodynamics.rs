use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct BarrowmanComponent {
    pub c_na: f32, // Normal force coefficient derivative
    pub x_cp: f32, // Distance from nose tip to Center of Pressure (m)
}

pub struct BarrowmanSolver;

impl BarrowmanSolver {
    /// নোজ কোনের অ্যারোডাইনামিক কেন্দ্র ও লিফট কোএফিশিয়েন্ট (কনিক্যাল / ওজাইভ)
    pub fn solve_nose_cone(length: f32, base_radius: f32) -> BarrowmanComponent {
        let _ = base_radius;
        BarrowmanComponent {
            c_na: 2.0,
            x_cp: 0.666 * length,
        }
    }

    /// ট্রাপিজয়ডাল ফিন সেটের অ্যারোডাইনামিক বল ও প্রেসার কেন্দ্র
    pub fn solve_trapezoidal_fins(
        fin_count: usize,
        root_chord: f32,
        tip_chord: f32,
        span: f32,
        sweep_length: f32,
        body_radius: f32,
        fin_root_pos: f32,
    ) -> BarrowmanComponent {
        let s = span;
        let c_r = root_chord;
        let c_t = tip_chord;
        let r = body_radius;
        let l_f = (s.powi(2) + (sweep_length + 0.5 * (c_t - c_r)).powi(2)).sqrt();
        let a_ref = PI * r.powi(2);

        // ফিন বডি ইন্টারফেয়ারেন্স ফ্যাক্টর
        let interference = 1.0 + (r / (s + r));
        let num = 4.0 * (fin_count as f32) * (s / (2.0 * r)).powi(2);
        let den = 1.0 + (1.0 + ((2.0 * l_f) / (c_r + c_t)).powi(2)).sqrt();
        let c_na_isolated = (num / den) * (PI * r.powi(2) / a_ref);
        let c_na = c_na_isolated * interference;

        let x_b = sweep_length * (c_r + 2.0 * c_t) / (3.0 * (c_r + c_t))
            + (1.0 / 6.0) * (c_r + c_t - (c_r * c_t) / (c_r + c_t));
        let x_cp = fin_root_pos + x_b;

        BarrowmanComponent { c_na, x_cp }
    }

    /// কমপ্লিট স্ট্যাবিলিটি ক্যালকুলেশন: সিপি, সিজি এবং স্ট্যাটিক ক্যালিবার মার্জিন
    pub fn compute_static_margin(
        components: &[BarrowmanComponent],
        cg: f32,
        body_diameter: f32,
    ) -> (f32, f32) {
        let mut total_cna = 0.0;
        let mut moment_sum = 0.0;

        for comp in components {
            total_cna += comp.c_na;
            moment_sum += comp.c_na * comp.x_cp;
        }

        let cp = if total_cna.abs() > 1e-6 {
            moment_sum / total_cna
        } else {
            0.0
        };

        let static_margin_calibers = (cp - cg) / body_diameter;
        (cp, static_margin_calibers)
    }
}
