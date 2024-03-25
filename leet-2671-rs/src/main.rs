use core::hash::Hash;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Counter<T> {
    inner: HashMap<T, u32>,
}

#[derive(Debug)]
struct FrequencyTracker {
    inner: Counter<i32>,
    track: Counter<u32>,
}

impl<T> Counter<T>
where
    T: Copy + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add_one(&mut self, e: T) -> u32 {
        self.inner.entry(e).and_modify(|v| *v += 1).or_insert(1);
        self.inner[&e]
    }

    pub fn sub_one(&mut self, e: T) -> Option<u32> {
        if let Some(k) = self.inner.get_mut(&e) {
            if *k > 1 {
                *k -= 1;
                Some(*k)
            } else {
                self.inner.remove(&e);
                Some(0)
            }
        } else {
            None
        }
    }

    pub fn get_count(&self, e: T) -> u32 {
        *self.inner.get(&e).unwrap_or(&0)
    }
}

impl FrequencyTracker {
    fn new() -> Self {
        Self {
            inner: Counter::new(),
            track: Counter::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let freq = self.inner.add_one(number);
        self.track.sub_one(freq - 1);
        self.track.add_one(freq);
    }

    fn delete_one(&mut self, number: i32) {
        match self.inner.sub_one(number) {
            None => {},
            Some(0) => {
                self.track.sub_one(1);
            },
            Some(freq) => {
                self.track.sub_one(freq + 1);
                self.track.add_one(freq);
            },
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.track.get_count(frequency as u32) > 0
    }
}

// impl FrequencyTracker {
//     fn new() -> Self {
//         Self {
//             inner: HashMap::new(),
//             track: HashMap::new(),
//         }
//     }
//
//     fn add(&mut self, number: i32) {
//         let freq = *self
//             .inner
//             .entry(number)
//             .and_modify(|e| *e += 1)
//             .or_insert(1);
//         self.track.entry(freq).and_modify(|e| *e += 1).or_insert(1);
//     }
//
//     fn delete_one(&mut self, number: i32) {
//         if let Some(&freq) = self.inner.get(&number) {
//             self.inner.remove(&number);
//         } else {
//         }
//     }
//
//     fn has_frequency(&self, frequency: i32) -> bool {
//         self.track.contains_key(&(frequency as u32))
//     }
// }

fn main() {
    let mut ft = FrequencyTracker::new();

    ft.add(3);
    ft.add(4);
    ft.delete_one(3);
    ft.delete_one(3);


    dbg!(&ft);
    ft.has_frequency(1);

    println!("Hello, world!");
}
