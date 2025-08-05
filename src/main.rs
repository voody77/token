use rand::prelude::IndexedRandom;
use rand::rng;

// Создаем константу с символами один раз
const SYMBOLS: [char; 72] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

fn generate_pass() -> String {
    let mut rng = rng();
    let mut part = String::with_capacity(6); // Задаем емкость заранее

    for _ in 0..6 {
        if let Some(&c) = SYMBOLS.choose(&mut rng) {
            part.push(c);
        }
    }

    part
}

fn main() {
    // Оптимизированный вывод с использованием интерполяции строк
    println!(
        "{}-{}-{}",
        generate_pass(),
        generate_pass(),
        generate_pass()
    );
}
