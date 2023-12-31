fn get_from_database(key: &str) -> Option<f64> {
    let database = vec![
        ("base", Some(4.0)),
        ("height", Some(-1.0)),
    ];

    for (k, v) in database {
        if k == key {
            return v;
        }
    }
    None
}

fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b <= 0.0 || h <= 0.0 {
                Err("Both base and height must be positive".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        },
        (None, _) => Err("Base not found".to_string()),
        (_, None) => Err("Height not found".to_string()),
    }
}

fn main() {
    let base = get_from_database("base");
    let height = get_from_database("height");

    let area = calculate_triangle_area(base, height);

    match area {
        Ok(area) => println!("Area: {}", area),
        Err(err) => println!("Error: {}", err),
    }
}
