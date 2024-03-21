use std::collections::HashMap;

struct FrequencyTracker {
    val_counter: HashMap<i32, usize>,
    fre_counter: HashMap<usize, usize>,
}

#[allow(unused)]
impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            val_counter: Default::default(),
            fre_counter: Default::default(),
        }
    }

    fn add(&mut self, number: i32) {
        let f = self.val_counter.entry(number).or_insert(0);
        if *f > 0 {
            let g = self.fre_counter.entry(*f).or_default();
            *g -= 1;
            if *g == 0 {
                self.fre_counter.remove(f);
            }
        }
        *f += 1;
        *self.fre_counter.entry(*f).or_default() += 1;
    }

    fn delete_one(&mut self, number: i32) {
        if self.val_counter.contains_key(&number) {
            let f = self.val_counter.entry(number).or_default();

            let g = self.fre_counter.entry(*f).or_default();
            *g -= 1;
            if *g == 0 {
                self.fre_counter.remove(f);
            }

            *f -= 1;
            if *f == 0 {
                self.val_counter.remove(&number);
            } else {
                *self.fre_counter.entry(*f).or_default() += 1;
            }
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.fre_counter.contains_key(&(frequency as usize))
    }
}

#[test]
fn test1() {
    let mut tracker = FrequencyTracker::new();
    tracker.add(3);
    tracker.add(3);
    assert!(tracker.has_frequency(2));
}
