use crate::geometry::mesh::{Mesh3D, Vertex};
use std::f32::consts::PI;

pub struct Rocket3DBuilder;

impl Rocket3DBuilder {
    pub fn build_solid_vehicle(
        nose_len: f32,
        body_len: f32,
        radius: f32,
        fin_count: u32,
        fin_span: f32,
        fin_root: f32,
        fin_thickness: f32,
        segments: u32,
    ) -> Mesh3D {
        let mut mesh = Mesh3D::new();

        // ১. সলিড নোজ কোন (Watertight Ogive/Cone)
        let tip_idx = 0;
        mesh.vertices.push(Vertex { x: 0.0, y: nose_len + body_len, z: 0.0 });

        for i in 0..segments {
            let theta = (i as f32 / segments as f32) * 2.0 * PI;
            let x = radius * theta.cos();
            let z = radius * theta.sin();
            mesh.vertices.push(Vertex { x, y: body_len, z });
        }

        for i in 0..segments {
            let current = 1 + i;
            let next = 1 + (i + 1) % segments;
            mesh.indices.push([tip_idx, current, next]);
        }

        // ২. সলিড বডি টিউব
        let body_bot_offset = mesh.vertices.len() as u32;
        for i in 0..segments {
            let theta = (i as f32 / segments as f32) * 2.0 * PI;
            let x = radius * theta.cos();
            let z = radius * theta.sin();
            mesh.vertices.push(Vertex { x, y: 0.0, z });
        }

        for i in 0..segments {
            let top_c = 1 + i;
            let top_n = 1 + (i + 1) % segments;
            let bot_c = body_bot_offset + i;
            let bot_n = body_bot_offset + (i + 1) % segments;

            mesh.indices.push([top_c, bot_c, top_n]);
            mesh.indices.push([top_n, bot_c, bot_n]);
        }

        // ৩. বোট-টেল / বেস ক্যাপ (সিলিন্ডারের তলা বন্ধ করা - Watertight Bottom)
        let base_center_idx = mesh.vertices.len() as u32;
        mesh.vertices.push(Vertex { x: 0.0, y: 0.0, z: 0.0 }); // বেস সেন্টার
        for i in 0..segments {
            let current = body_bot_offset + i;
            let next = body_bot_offset + (i + 1) % segments;
            mesh.indices.push([base_center_idx, next, current]);
        }

        // ৪. ৩D সলিড ফিন (Thickness / ডাবল সাইডেড প্রিমিটিভ)
        let half_th = fin_thickness / 2.0;
        for f in 0..fin_count {
            let angle = (f as f32 / fin_count as f32) * 2.0 * PI;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            let perp_x = -sin_a * half_th;
            let perp_z = cos_a * half_th;

            let start_idx = mesh.vertices.len() as u32;

            // বাম পাশ (Side A)
            mesh.vertices.push(Vertex { x: radius * cos_a + perp_x, y: 0.0, z: radius * sin_a + perp_z });
            mesh.vertices.push(Vertex { x: (radius + fin_span) * cos_a + perp_x, y: 0.0, z: (radius + fin_span) * sin_a + perp_z });
            mesh.vertices.push(Vertex { x: radius * cos_a + perp_x, y: fin_root, z: radius * sin_a + perp_z });

            // ডান পাশ (Side B)
            mesh.vertices.push(Vertex { x: radius * cos_a - perp_x, y: 0.0, z: radius * sin_a - perp_z });
            mesh.vertices.push(Vertex { x: (radius + fin_span) * cos_a - perp_x, y: 0.0, z: (radius + fin_span) * sin_a - perp_z });
            mesh.vertices.push(Vertex { x: radius * cos_a - perp_x, y: fin_root, z: radius * sin_a - perp_z });

            // দুই পাশের ফেস ও এজ সিল করা (Solid Volume Fin)
            mesh.indices.push([start_idx, start_idx + 1, start_idx + 2]);
            mesh.indices.push([start_idx + 3, start_idx + 5, start_idx + 4]);
            mesh.indices.push([start_idx + 1, start_idx + 4, start_idx + 2]);
            mesh.indices.push([start_idx + 2, start_idx + 4, start_idx + 5]);
        }

        mesh
    }
}
