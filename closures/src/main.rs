use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Entry;

fn main() {
    let simulated_user_specified_value = 6;
    let simulated_random_number = 3;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity*2)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T,K,U>
    where T: Fn(K) -> U
{
    calculation: T,
    value: HashMap<K,U>,
}

impl<T,K,U> Cacher<T,K,U>
    where T: Fn(K) -> U,
    K: Hash + Eq + Clone,
    U: Clone
{
    fn new(calculation: T) -> Cacher<T,K,U> {
        Cacher {
            // let mut mapper = HashMap::new();
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> &U {
        match self.value.entry(arg.clone()) {
            Entry::Occupied(v) => v.into_mut(),
            Entry::Vacant(v) =>   v.insert((self.calculation)(arg)),
        
        }
    }
}