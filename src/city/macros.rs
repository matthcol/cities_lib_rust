use super::City;
use std::any::Any;

#[macro_export] 
macro_rules! city {
    ($name:expr, $pop:expr) => {
        // {
        //     let mut name = "?".to_string();
        //     if  let Some(string) = (&$name as &dyn Any).downcast_ref::<String>() {
        //         name = string.to_string()
        //     } else if let Some(string_str) = (&$name as &dyn Any).downcast_ref::<&str>() {
        //         name = string_str.to_string()
        //     }
        //     City::new(name, $pop, "France".to_string())
        // }
        City::new($name.to_string(), $pop, "France".to_string())
    };
    ($name:expr, $pop:expr, $country:expr) => {
        City::new($name.to_string(), $pop, country.to_string())
    };
}

#[macro_export] 
macro_rules! city_s {
    (s $name:expr; $pop:expr) => {
        City::new($name.to_string(), $pop, "France".to_string())
    };
    (S $name:expr; $pop:expr) => {
        City::new($name, $pop, "France".to_string())
    };
    (s $name:expr; $pop:expr; s $country:expr) => {
        City::new($name.to_string(), $pop, $country.to_string())
    };
    (S $name:expr; $pop:expr; S $country:expr) => {
        City::new($name, $pop, $country)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_city(){
        let city = city!("Toulouse", 470000);
        assert_eq!(city.name, "Toulouse".to_string());
        assert_eq!(city.population, 470000);
        assert_eq!(city.country, "France".to_string())
    }

    #[test]
    fn test_macro_city_s(){
        let city = city_s!(s "Toulouse"; 470000);
        assert_eq!(city.name, "Toulouse".to_string());
        assert_eq!(city.population, 470000);
        assert_eq!(city.country, "France".to_string())
    }
}