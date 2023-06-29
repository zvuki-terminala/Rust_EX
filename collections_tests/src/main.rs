use std::collections::HashMap;

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}
/* 
    Вместо вектора используем массив и суммируем стандартными методами
*/

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
        /* 
            Мода списка - создаем хэш-мапу считываем в нее массив с назначением 
            порядкового номера 
        */
}

fn main() {
    let mut numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    println!("AVERAGE: {}", average(&numbers));
    println!("MEDIAN: {}", median(&mut numbers));
    println!("MODE: {}", mode(&numbers));
}
