use std::thread;
use std::time::Duration;

fn main() {
    println!("Practice 13.1");
    practice_13_1();
    println!("---------------------------");

    println!("Practice 13.2");
    practice_13_2();
    println!("---------------------------");
}

fn practice_13_1() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    let x1 = 4;
    let x2 = vec![1, 2, 3];

    let equal_to_x1 = |z| z == x1;
    let equal_to_x2 = move |z| z == x2;

    let y1 = 4;
    let y2 = vec![1, 2, 3];

    assert!(equal_to_x1(y1));
    assert!(equal_to_x2(y2));

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });
    
        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_result.value(intensity));
            }
        }
    }
    
    struct Cacher<T>
        where T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>,
    }
    
    impl<T> Cacher<T>
        where T: Fn(u32) -> u32
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
    
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }
}

fn practice_13_2() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();

    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    let total: i32 = v2.iter().sum();

    assert_eq!(total, 6);

    let v3: Vec<i32> = vec![1, 2, 3];
    let v4: Vec<_> = v3.iter().map(|x| x + 1).collect();

    assert_eq!(v4, vec![2, 3, 4]);
}