use std::ops::Add;

#[allow(unused)]
fn do_something<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

#[allow(unused)]
#[derive(Debug)]
struct City {
    name: String,
    years: Vec<u32>,
    populations: Vec<u32>,
}

impl City {
    fn new(name: &str, years: Vec<u32>, populations: Vec<u32>) -> Self {
        City {
            name: name.to_owned(),
            years,
            populations,
        }
    }

    fn city_data<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
    {
        f(&mut self.years, &mut self.populations);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Iterator for Rectangle {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Rectangle {
            width: self.width + 1,
            height: self.height + 1,
        })
    }
}

impl Add for Rectangle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Rectangle {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnonce() {
        let some_vec = vec![6, 8, 5];
        do_something(|| {
            some_vec
                .into_iter()
                .for_each(|x| println!("The number is: {}", x))
        })
    }

    #[test]
    fn test_fnmut() {
        let years = vec![
            1372, 1834, 1851, 1881, 1897, 1925, 1959, 1989, 2000, 2005, 2010, 2020,
        ];
        let populations = vec![
            3_250, 15_300, 24_000, 45_900, 58_800, 119_800, 283_071, 478_974, 400_378, 401_694,
            406_703, 437_619,
        ];

        let mut cq = City::new("Cq", years, populations);
        cq.city_data(|city_years, city_populations| {
            let new_vec = city_years
                .into_iter()
                .zip(city_populations.into_iter())
                .take(5)
                .collect::<Vec<(_, _)>>();
            println!("{:?}", new_vec);
        });

        cq.city_data(|x, y| {
            x.push(2030);
            y.push(5_000_000);
        });

        cq.city_data(|x, y| {
            let position_option = x.iter().position(|x| *x == 1834);
            if let Some(position) = position_option {
                println!(
                    "going to delete {} at position {} now.",
                    x[position], position
                );
                x.remove(position);
                y.remove(position);
            }
        });

        println!(
            "Years left are {:?}\nPopulations left are {:?}",
            cq.years, cq.populations
        );
    }

    #[test]
    fn test_sort_by_key() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        list.sort_by_key(|r| r.width);
        println!("{:#?}", list);
    }
}
