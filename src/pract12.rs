use rand::Rng;

// Генерація випадкового вектора з значеннями в діапазоні [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Функція для знаходження мінімальної суми сусідніх елементів
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_indexes = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indexes = (i, i + 1);
        }
    }

    (min_sum, min_indexes.0, min_indexes.1)
}

// Функція для виводу даних у зрозумілому вигляді
fn print_data(data: &[i32]) {
    // Виводимо позиції та значення мінімальної сусідньої пари
    let (min_sum, idx1, idx2) = min_adjacent_sum(data);
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[idx1], data[idx2], min_sum, idx1, idx2
    );

    // Виводимо індекси
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>2}. ", i);
    }
    println!();

    // Виводимо дані
    print!("data:    [");
    for (i, &val) in data.iter().enumerate() {
        print!("{}", val);
        if i < data.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    // Виводимо стрілки
    print!("indexes:  ");
    for i in 0..data.len() {
        if i == idx1 {
            print!("↑  "); // Відмічаємо перший індекс стрілкою
        } else if i == idx2 {
            print!(" ↑ "); // Відмічаємо другий індекс стрілкою
        } else {
            print!("    ");
        }
    }
    println!();
}

pub fn main() {
    const SIZE: usize = 20;

    // Генеруємо випадковий вектор
    let data = gen_random_vector(SIZE);

    // Виводимо дані
    print_data(&data);
}
