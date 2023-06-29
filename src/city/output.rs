use super::City;

pub fn print_cities_lines(cities: &Vec<City>) {
    cities.iter().for_each(|c| println!(" - {c}"));
}

pub fn println_vector_cities(cities: &Vec<City>) {
    print!("[");
    let mut it = cities.iter();

    if let Some(city) = it.next() {
        print!("{city}");
        while let Some(city) = it.next() {
            print!(", {city}");
        }
    }
    println!("]");
}