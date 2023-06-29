pub mod output;
pub mod macros;

use core::fmt;
use std::ops::AddAssign;
// use std::fmt; 


#[derive(Debug)]
// #[derive(Debug, PartialEq, Eq)] // def auto de == et != field by field
pub struct City{
    pub name: String,
    pub population: u32,
    pub country: String
}
// other ideas: lat, lon, list celebrities

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (pop={}, country={})", self.name, self.population, self.country)
    }
}

// impl +=
impl AddAssign<u32> for City {
    fn add_assign(&mut self, rhs: u32) {
        self.population += rhs;
    }
}

// impl ==, !=
impl PartialEq for City {
    fn eq(&self, other: &Self) -> bool {
        return (self.name.to_lowercase() == other.name.to_lowercase())
            && (self.country.to_lowercase() == other.country.to_lowercase())
    }
}

impl Eq for City {}

// impl From<&str> for City {
//     /**
//      * format "csv": name,population,country
//      */
//     fn from(value: &str) -> Self {
//         let fields: Vec<&str> = value.split(',').collect();
        
//         City{
//             name: String::from(fields[0]), 
//             population: u32::from_str_radix(fields[1], 10)
//                     .ok().unwrap(), 
//             country: String::from(fields[2])
//         }
//     }
// }

impl TryFrom<&str> for City {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fields: Vec<&str> = value.split(',').collect();
        if fields.len() != 3 {
            Err("Wrong number of fields")
        } else {
            match u32::from_str_radix(fields[1], 10) {
                Ok(population) => Ok(
                    City{
                    name: fields[0].to_string(), 
                    population: population,
                    country: fields[2].to_string()
                    }
                ),
                Err(_) => Err("Population is not an integer")
            }
        }
    }
}

impl City {
    pub fn new_with_str(name: &str, population: u32, country: &str) -> City {
        City {
            name: name.to_string(),
            population: population,
            country: country.to_string()
        }
    }

    pub fn new(name: String, population: u32, country: String) -> City {
        City {
            name: name,
            population: population,
            country: country
        }
    }

    pub fn is_small(&self) -> bool {
        self.population < 100_000u32
    }
}

impl City {
    pub fn is_medium(&self) -> bool {
        (self.population >= 100_000u32) && (self.population < 1_000_000u32)
    }

    pub fn incr_population(&mut self, delta: u32) {
        self.population += delta;
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_from_ref_str() {
//         let text = "Toulouse,470000,France";
//         let city = City::from(text);
//         assert_eq!(city.name, "Toulouse");
//         assert_eq!(city.population, 470_000);
//         assert_eq!(city.country, "France");
//     }

//     #[test]
//     #[should_panic]
//     fn test_from_ref_str_ko_pop_not_integer() {
//         let text = "Toulouse,big,France";
//         let _ = City::from(text);
//     }

//     #[test]
//     #[should_panic]
//     fn test_from_ref_str_ko_wrong_number_fields() {
//         let text = "Toulouse,470000";
//         let _ = City::from(text);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_str(){
        let city = City::new_with_str("Toulouse", 470000, "France");
        assert_eq!(city.name, "Toulouse".to_string());
        assert_eq!(city.population, 470000);
        assert_eq!(city.country, "France".to_string())
    }

    #[test]
    fn test_try_from_ref_str() {
        let text = "Toulouse,470000,France";
        let result =  City::try_from(text);
        assert!(result.is_ok());
        let city = result.ok().unwrap();
        assert_eq!(city.name, "Toulouse");
        assert_eq!(city.population, 470_000);
        assert_eq!(city.country, "France");
    }

    #[test]
    fn test_try_from_ref_str_ko_pop_not_integer() {
        let text = "Toulouse,big,France";
        let result =  City::try_from(text);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_ref_str_ko_wrong_number_fields() {
        let text = "Toulouse,470000";
        let result =  City::try_from(text);
        assert!(result.is_err());
    }
}