use std::fs::File;
use std::io::Write;

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Mesh3D {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<[u32; 3]>,
}

impl Mesh3D {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn export_obj_with_mtl(&self, obj_name: &str, mtl_name: &str) -> std::io::Result<()> {
        // ১. মেটেরিয়াল ফাইল তৈরি
        let mut mtl_file = File::create(mtl_name)?;
        writeln!(mtl_file, "# AquaRocket Carbon Composite Material")?;
        writeln!(mtl_file, "newmtl RocketSkin")?;
        writeln!(mtl_file, "Kd 0.15 0.18 0.22")?; // অ্যারোস্পেস ডার্ক গ্রে ম্যাট
        writeln!(mtl_file, "Ks 0.5 0.5 0.5")?;   // স্পেকুলার হাইলাইট
        writeln!(mtl_file, "Ns 60.0")?;

        // ২. অবজেক্ট ফাইল তৈরি
        let mut obj_file = File::create(obj_name)?;
        writeln!(obj_file, "# AquaRocket Watertight Solid Aerospace Mesh")?;
        writeln!(obj_file, "mtllib {}", mtl_name)?;
        writeln!(obj_file, "usemtl RocketSkin")?;

        for v in &self.vertices {
            writeln!(obj_file, "v {:.4} {:.4} {:.4}", v.x, v.y, v.z)?;
        }

        for f in &self.indices {
            writeln!(obj_file, "f {} {} {}", f[0] + 1, f[1] + 1, f[2] + 1)?;
        }

        Ok(())
    }
}
