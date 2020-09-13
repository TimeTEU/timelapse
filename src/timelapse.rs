pub struct Timelapse<'a> {
    target: i32,
    increment: i32,
    threshold: i32,
    source: &'a [i32],
    prev_delta: i32,
    prev_val: i32,
    index: usize,
}

impl Timelapse<'_> {
    pub fn from_sorted_vector(
        source: &[i32],
        start: i32,
        increment: i32,
        threshold: i32,
    ) -> Timelapse {
        Timelapse {
            target: start,
            increment,
            threshold,
            source,
            prev_delta: std::i32::MAX,
            prev_val: source[0],
            index: 0,
        }
    }
}

impl Iterator for Timelapse<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let input_size = self.source.len();
        'l1: while self.index < input_size {
            let val = self.source[self.index];
            // println!("? {}", val);
            self.index += 1;

            let mut delta = (self.target - val).abs();

            if self.prev_delta <= self.threshold && self.prev_delta <= delta {
                let ret = Some(self.prev_val);
                // println!("👍 {} (target: {}, delta: {})", self.prev_val, self.target, self.prev_delta);
                self.prev_val = val;
                self.target += self.increment;
                self.prev_delta = std::i32::MAX;
                self.index -= 1;
                return ret;
            }

            while delta > self.threshold {
                if self.target > val {
                    continue 'l1;
                } else {
                    self.target += self.increment;
                    delta = (self.target - val).abs();
                }
            }

            self.prev_val = val;
            if delta == 0 {
                // println!("👍 {} (target: {}, delta: {})", val, self.target, delta);
                self.target += self.increment;
                self.prev_delta = std::i32::MAX;
                return Some(val);
            } else {
                self.prev_delta = delta;
            }
        }
        if self.index == input_size && self.prev_delta <= self.threshold {
            // println!("👍 {} (target: {}, delta: {})", self.prev_val, self.target, self.prev_delta);
            self.prev_delta = std::i32::MAX;
            return Some(self.prev_val);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::timelapse::Timelapse;

    #[test]
    fn test_case1() {
        let vector = vec![0, 5, 10, 15, 20];
        let mut tl = Timelapse::from_sorted_vector(&vector, 0, 5, 2);

        assert_eq!(0, tl.next().unwrap());
        assert_eq!(5, tl.next().unwrap());
        assert_eq!(10, tl.next().unwrap());
        assert_eq!(15, tl.next().unwrap());
        assert_eq!(20, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case2() {
        let vector = vec![0, 5, 10, 15, 20];
        let mut tl = Timelapse::from_sorted_vector(&vector, 0, 10, 2);

        assert_eq!(0, tl.next().unwrap());
        assert_eq!(10, tl.next().unwrap());
        assert_eq!(20, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case3() {
        let vector = vec![0, 5, 10, 15, 20];
        let mut tl = Timelapse::from_sorted_vector(&vector, 2, 10, 2);

        assert_eq!(0, tl.next().unwrap());
        assert_eq!(10, tl.next().unwrap());
        assert_eq!(20, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case4() {
        let vector = vec![0, 5, 10, 15, 20];
        let mut tl = Timelapse::from_sorted_vector(&vector, 2, 10, 1);

        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case5() {
        let vector = vec![0, 5, 10, 15, 20];
        let mut tl = Timelapse::from_sorted_vector(&vector, 3, 10, 2);

        assert_eq!(5, tl.next().unwrap());
        assert_eq!(15, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case6() {
        let vector = vec![0, 2, 4, 6, 8, 12, 20, 22, 24, 28];
        // 0 _ 2 _ 4 _ 6 _ 8 _ __ __ 12 __ __ __ __ __ __ __ 20 __ 22 __ 24 __ __ __ 28
        // _ 1 _ _ 4 _ _ 7 _ _ 10 __ __ 13 __ __ 16 __ __ 19 __ __ 22 __ __ 25 __ __ 28
        // 0 _ _ _ 4 _ 6 _ 8 _ __ __ 12 __ __ __ __ __ __ __ 20 __ 22 __ 24 __ __ __ 28

        let mut tl = Timelapse::from_sorted_vector(&vector, 1, 3, 2);
        assert_eq!(0, tl.next().unwrap());
        assert_eq!(4, tl.next().unwrap());
        assert_eq!(6, tl.next().unwrap());
        assert_eq!(8, tl.next().unwrap());
        assert_eq!(12, tl.next().unwrap());
        assert_eq!(20, tl.next().unwrap());
        assert_eq!(22, tl.next().unwrap());
        assert_eq!(24, tl.next().unwrap());
        assert_eq!(28, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }
}
