use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;

pub struct DynamicRocketCAD;

impl DynamicRocketCAD {
    pub fn generate_obj_file(
        path: &str,
        caliber: f32,
        nose_len: f32,
        body_len: f32,
        fin_root: f32,
        fin_tip: f32,
        fin_span: f32,
        fin_sweep: f32,
        fin_count: u32,
    ) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "# AquaRocket Dynamic Parametric CAD Mesh")?;

        let r = caliber * 0.5;
        let segments = 24;
        let mut v_idx = 1;

        // ১. নোজ কোন (Conical/Tangent Ogive Approximation)
        // Tip vertex
        writeln!(file, "v 0.0 {:.4} 0.0", body_len * 0.5 + nose_len)?;
        let tip_idx = v_idx;
        v_idx += 1;

        let base_y = body_len * 0.5;
        let nose_base_start = v_idx;
        for i in 0..segments {
            let theta = (i as f32 / segments as f32) * 2.0 * PI;
            let x = r * theta.cos();
            let z = r * theta.sin();
            writeln!(file, "v {:.4} {:.4} {:.4}", x, base_y, z)?;
            v_idx += 1;
        }

        // নোজ কোন ফেস
        for i in 0..segments {
            let next = if i + 1 == segments { 0 } else { i + 1 };
            writeln!(file, "f {} {} {}", tip_idx, nose_base_start + i, nose_base_start + next)?;
        }

        // ২. বডি টিউব
        let bottom_y = -body_len * 0.5;
        let tube_bottom_start = v_idx;
        for i in 0..segments {
            let theta = (i as f32 / segments as f32) * 2.0 * PI;
            let x = r * theta.cos();
            let z = r * theta.sin();
            writeln!(file, "v {:.4} {:.4} {:.4}", x, bottom_y, z)?;
            v_idx += 1;
        }

        // বডি টিউব কোয়াড/ট্রাই ফেস
        for i in 0..segments {
            let next = if i + 1 == segments { 0 } else { i + 1 };
            let top_cur = nose_base_start + i;
            let top_next = nose_base_start + next;
            let bot_cur = tube_bottom_start + i;
            let bot_next = tube_bottom_start + next;
            writeln!(file, "f {} {} {}", top_cur, bot_cur, bot_next)?;
            writeln!(file, "f {} {} {}", top_cur, bot_next, top_next)?;
        }

        // ৩. ট্রাপিজয়ডাল ফিন সেট
        for f in 0..fin_count {
            let phi = (f as f32 / fin_count as f32) * 2.0 * PI;
            let normal_x = phi.cos();
            let normal_z = phi.sin();

            let root_top_y = bottom_y + fin_root;
            let root_bot_y = bottom_y;
            let tip_top_y = root_top_y - fin_sweep;
            let tip_bot_y = tip_top_y - fin_tip;

            // ৪টি কর্নার ভার্টেক্স (Root Top, Tip Top, Tip Bottom, Root Bottom)
            let v1 = v_idx;
            writeln!(file, "v {:.4} {:.4} {:.4}", r * normal_x, root_top_y, r * normal_z)?;
            let v2 = v_idx + 1;
            writeln!(file, "v {:.4} {:.4} {:.4}", (r + fin_span) * normal_x, tip_top_y, (r + fin_span) * normal_z)?;
            let v3 = v_idx + 2;
            writeln!(file, "v {:.4} {:.4} {:.4}", (r + fin_span) * normal_x, tip_bot_y, (r + fin_span) * normal_z)?;
            let v4 = v_idx + 3;
            writeln!(file, "v {:.4} {:.4} {:.4}", r * normal_x, root_bot_y, r * normal_z)?;
            v_idx += 4;

            // ফিনের সম্মুখ ও পশ্চাৎ ফেস
            writeln!(file, "f {} {} {}", v1, v2, v3)?;
            writeln!(file, "f {} {} {}", v1, v3, v4)?;
            writeln!(file, "f {} {} {}", v3, v2, v1)?;
            writeln!(file, "f {} {} {}", v4, v3, v1)?;
        }

        Ok(())
    }
}
