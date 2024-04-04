use std::fmt::{self, Formatter, Display};
use std::vec::Vec;
use std::collections::HashMap;

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)
    }
}

fn foo(city: &mut City) -> () {
    city.lat += 1.0;
}

fn main() {
    let mut cities = Vec::from([
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ]);
    cities.push(City { name: "asdasdasdas", lat: 1111.0, lon: -6.0 });

    let mut hash = HashMap::new();
    hash.insert("test1", 1);
    hash.insert("test2", 2);
    hash.insert("test3", 3);
    hash.insert("test4", 4);

    for mut city in cities {
        let r: &mut City = &mut city;
        foo(r);
        foo(&mut city);
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", color);
    }
}