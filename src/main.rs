fn get_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    }
    else {
        None
    }
}

fn main() {
    let result = get_square_root(13 as f64);
    match result {
        Some(value) => println!(" = {}", value),
        None => println!("음수는 입력할 수 없음"),
    }
}

