pub struct NoseCone {
    pub length: f32,
    pub _base_radius: f32,
}

impl NoseCone {
    // কনিক্যাল নোজ কোনের CP
    pub fn center_of_pressure(&self) -> f32 {
        0.666 * self.length
    }

    pub fn cna(&self) -> f32 {
        2.0
    }
}

pub struct TrapezoidalFinSet {
    pub fin_count: u32,
    pub root_chord: f32, // বডির সাথে জোড়া অংশের দৈর্ঘ্য
    pub tip_chord: f32,  // শীর্ষ অংশের দৈর্ঘ্য
    pub span: f32,       // উচ্চতা
    pub mid_chord_sweep: f32,
    pub body_radius: f32,
    pub fin_root_position: f32, // নোজের ডগা থেকে দূরত্ব
}

impl TrapezoidalFinSet {
    pub fn cna(&self) -> f32 {
        let n = self.fin_count as f32;
        let s = self.span;
        let cr = self.root_chord;
        let ct = self.tip_chord;
        let r = self.body_radius;

        let k = 1.0 + (r / (s + r));
        let numerator = 4.0 * n * (s / (2.0 * r)).powi(2);
        let denominator = 1.0 + (1.0 + (2.0 * self.mid_chord_sweep / (cr + ct)).powi(2)).sqrt();

        k * (numerator / denominator)
    }

    pub fn center_of_pressure(&self) -> f32 {
        let cr = self.root_chord;
        let ct = self.tip_chord;
        let xf = (self.mid_chord_sweep / 3.0) * ((cr + 2.0 * ct) / (cr + ct))
            + (1.0 / 6.0) * ((cr.powi(2) + ct.powi(2) + cr * ct) / (cr + ct));
        self.fin_root_position + xf
    }
}
