#[cfg(test)]
mod tests {
    use traveler_problem::utils::generate_svg::GenerateSvg;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_new_generate_svg() {
        let svg = GenerateSvg::new(true, 200, 100, 10);
        assert!(svg.svg_mode);
        assert_eq!(svg.points.len(), 0);
        assert_eq!(svg.width, 200);
        assert_eq!(svg.height, 100);
        assert_eq!(svg.margin, 10);
    }

    #[test]
    fn test_add_point_svg_mode_false() {
        let mut svg = GenerateSvg::new(false, 100, 50, 5);
        let added = svg.add_point((42.0, 1));
        assert!(!added);
    }

    #[test]
    fn test_add_point_svg_mode_true() {
        let mut svg = GenerateSvg::new(true, 100, 50, 5);
        let added = svg.add_point((42.0, 1));
        assert!(added);
        assert_eq!(svg.points.len(), 1);
    }

    #[test]
    fn test_generate_svg_empty() {
        let mut svg = GenerateSvg::new(true, 150, 75, 10);
        let content = svg.generate_svg();
        assert!(content.contains(r#"<svg xmlns="http://www.w3.org/2000/svg" width="150" height="75">"#));
        assert!(!content.contains("<circle"));
        assert!(!content.contains("<path"));
    }

    #[test]
    fn test_generate_svg_with_points() {
        let mut svg = GenerateSvg::new(true, 100, 50, 5);
        svg.add_point((10.0, 0));
        svg.add_point((20.0, 1));
        svg.add_point((30.0, 0));
        let content = svg.generate_svg();
        assert!(content.contains("<path"));
        assert!(content.contains("<circle"));
        assert!(content.contains("fill=\"red\""));
        let red_count = content.matches("fill=\"red\"").count();
        assert_eq!(red_count, 2);
    }

    #[test]
    fn test_save_svg_creates_file() {
        let tmp_dir = tempdir().unwrap();
        let dir_path = tmp_dir.path().to_str().unwrap();

        let mut svg = GenerateSvg::new(true, 100, 50, 5);
        svg.add_point((10.0, 0));
        svg.add_point((20.0, 1));

        let file_name = "test_chart".to_string();
        let file_path = svg.save_svg(dir_path, file_name.clone()).unwrap();
        
        assert!(fs::metadata(&file_path).is_ok());

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("<svg"));
        assert!(content.contains("<path"));
        assert!(content.contains("<circle"));
    }
}
