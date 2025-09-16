use std::fs::{File, create_dir_all};
use std::io::Write;
pub struct GenerateSvg{
    pub svg_mode : bool,
    pub points : Vec<(f64,u8)>,
    pub width: u32, 
    pub height: u32, 
    pub margin: u32, 
    pub sample_rate: usize
}

impl GenerateSvg{
    pub fn new(svg_mode : bool,width: u32,height: u32,margin: u32) -> Self{
        GenerateSvg {
            svg_mode,
            points : Vec::new(),
            width,
            height,
            margin,
            sample_rate : 0
        }
    }

    pub fn add_point(&mut self, point : (f64,u8)) -> bool {
        if !self.svg_mode {
            return false
        }

        self.points.push(point);
        true
    } 

    pub fn save_svg(&mut self, dir_path : &str, file_name : String) -> Result<String, std::io::Error> {        
        create_dir_all(dir_path)?;
        let svg_content = self.generate_svg();
        let file_path = format!("{}/{}.svg", dir_path, file_name);
        let mut file = File::create(&file_path)?;
        file.write_all(svg_content.as_bytes())?;

        Ok(file_path)
    }

    pub fn generate_svg(&mut self) -> String{
        if self.points.is_empty() {
            return format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}"></svg>"#, w=self.width, h=self.height);
        }

        self.sample_rate = 1;
        if self.points.len() > 10_000 {
            self.sample_rate = ((self.points.len() as f64) * 0.001) as usize;
        }

        let values: Vec<f64> = self.points.iter().map(|(v, _)| *v).collect();
        let min_y = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_y = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let inner_w = (self.width - 2* self.margin) as f64;
        let inner_h = (self.height - 2* self.margin) as f64;
        let n = self.points.len() as f64;
        let dy = if (max_y - min_y).abs() < f64::EPSILON { 1.0 } else { max_y - min_y };

        let mut svg = String::new();
        svg.push_str(&format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">"#, w=self.width, h=self.height));
        svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);

        svg.push_str(&format!(r#"<line x1="{m}" y1="{h_m}" x2="{w_m}" y2="{h_m}" stroke="black"/>"#, m=self.margin, h_m=self.height-self.margin, w_m=self.width-self.margin));
        svg.push_str(&format!(r#"<line x1="{m}" y1="{m}" x2="{m}" y2="{h_m}" stroke="black"/>"#, m=self.margin, h_m=self.height-self.margin));

        let mut path_data = String::new();
        for (i, &(y, _)) in self.points.iter().enumerate().step_by(self.sample_rate) {
            let nx = i as f64 / (n - 1.0).max(1.0);
            let ny = 1.0 - (y - min_y)/dy;
            let cx = self.margin as f64 + nx*inner_w;
            let cy = self.margin as f64 + ny*inner_h;

            if path_data.is_empty() {
                path_data.push_str(&format!("M{:.2},{:.2}", cx, cy));
            } else {
                path_data.push_str(&format!(" L{:.2},{:.2}", cx, cy));
            }
        }
        svg.push_str(&format!(r#"<path d="{path}" fill="none" stroke="blue" stroke-width="1"/>"#, path=path_data));

        for (i, &(y, flag)) in self.points.iter().enumerate().step_by(self.sample_rate) {
            if flag != 0 { continue; }

            let nx = i as f64 / (n - 1.0).max(1.0);
            let ny = 1.0 - (y - min_y)/dy;
            let cx = self.margin as f64 + nx*inner_w;
            let cy = self.margin as f64 + ny*inner_h;

            svg.push_str(&format!(
                r#"<circle cx="{:.2}" cy="{:.2}" r="2" fill="red"><title>iter {}: {:.4}</title></circle>"#,
                cx, cy, i, y
            ));
        }

        svg.push_str("</svg>");
        svg
    }
}