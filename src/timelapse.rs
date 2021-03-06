pub struct Timelapse<'a> {
    source: &'a mut dyn Iterator<Item = i32>,
    target: i32,
    increment: i32,
    threshold: i32,
    _prev: Option<i32>,
}

impl Timelapse<'_> {
    pub fn from_sorted_iterator(
        source: &mut dyn Iterator<Item = i32>,
        start: i32,
        increment: i32,
        threshold: i32,
    ) -> Timelapse {
        if threshold >= increment {
            panic!("Threshold must be lower than increment")
        }

        Timelapse {
            source,
            target: start,
            increment,
            threshold,
            _prev: None,
        }
    }

    fn delta(&mut self, candidate: i32) -> i32 {
        (candidate - self.target).abs()
    }
}

impl Iterator for Timelapse<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        loop {
            let current = match self.source.next() {
                None => break,
                Some(i) => i,
            };

            if let Some(prev) = self._prev {
                if prev > current {
                    panic!("sequence is not sorted");
                }

                while prev > self.target && self.delta(prev) > self.threshold {
                    self.target += self.increment
                }

                let delta = self.delta(prev);
                if delta <= self.threshold && delta <= self.delta(current) {
                    self.target += self.increment;
                    self._prev = Some(current);
                    return Some(prev);
                }
            }
            self._prev = Some(current);
        }

        let prev = match self._prev {
            None => return None,
            Some(i) => i,
        };
        while prev > self.target && self.delta(prev) > self.threshold {
            self.target += self.increment
        }
        if self.delta(prev) <= self.threshold {
            self._prev = None;
            Some(prev)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::timelapse::Timelapse;

    #[test]
    fn test_case1() {
        let vector = vec![0, 5, 10, 15, 20];
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 0, 5, 2);

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
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 0, 10, 2);

        assert_eq!(0, tl.next().unwrap());
        assert_eq!(10, tl.next().unwrap());
        assert_eq!(20, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case3() {
        let vector = vec![0, 5, 10, 15, 20];
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 2, 10, 2);

        assert_eq!(0, tl.next().unwrap());
        assert_eq!(10, tl.next().unwrap());
        assert_eq!(20, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case4() {
        let vector = vec![0, 5, 10, 15, 20];
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 2, 10, 1);

        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case5() {
        let vector = vec![0, 5, 10, 15, 20];
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 3, 10, 2);

        assert_eq!(5, tl.next().unwrap());
        assert_eq!(15, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case6() {
        let vector = vec![0, 2, 4, 6, 8, 12, 20, 22, 24, 28];
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 1, 3, 2);
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

    #[test]
    fn test_case7() {
        let vector = vec![11, 28, 29, 31];
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 0, 5, 2);
        assert_eq!(11, tl.next().unwrap());
        assert_eq!(29, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case8() {
        let vector = vec![11, 28, 31, 32];
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 0, 5, 2);
        assert_eq!(11, tl.next().unwrap());
        assert_eq!(31, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case9() {
        let vector = vec![11, 15, 72];
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 0, 5, 2);
        assert_eq!(11, tl.next().unwrap());
        assert_eq!(15, tl.next().unwrap());
        assert_eq!(72, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }

    #[test]
    fn test_case10() {
        let vector = vec![11, 15, 69];
        let mut it = vector.into_iter();
        let mut tl = Timelapse::from_sorted_iterator(&mut it, 0, 5, 2);
        assert_eq!(11, tl.next().unwrap());
        assert_eq!(15, tl.next().unwrap());
        assert_eq!(69, tl.next().unwrap());
        assert_eq!(None, tl.next());
    }
}
