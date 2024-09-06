type TimeType = i32;
pub enum Time {
    Infinity,
    Definite(TimeType),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Relation {
    ///Picture Example
    /// ```
    /// XXX
    ///       YYY
    /// ```
    Before,
    ///Picture Example
    /// ```
    ///      XXX
    /// YYY
    /// ```
    BeforeInverse,
    /// Picture Example
    /// ```
    ///  XXX
    ///  YYY
    /// ```
    Equal,
    /// Picture Example
    /// ```
    ///  XXX
    ///     YYY
    /// ```
    Meets,
    /// Picture Example
    /// ```
    ///    XXX
    /// YYY
    /// ```
    MeetsInverse,
    ///Picture Example
    /// ```
    /// XXX
    ///   YYYY
    /// ```
    Overlaps,
    ///Picture Example
    /// ```
    ///   XXX
    /// YYY
    /// ```
    OverlapsInverse,
    ///Picture Example
    ///```
    ///   XXX
    /// YYYYYY
    /// ```
    During,
    ///Picture Example
    /// ```
    /// XXXXXX
    ///   YYY
    /// ```
    DuringInverse,
    /// Picture Example
    /// ```
    /// XXX
    /// YYYYYY
    /// ```
    Starts,
    ///Picture Example
    ///```
    /// XXXXXX
    /// YYY
    /// ```
    StartsInverse,
    /// Picture Example
    /// ````
    ///   XXX
    /// YYYYY
    /// ```
    Finishes,
    /// Picture Example
    /// ````
    /// XXXXX
    ///   YYY
    /// ```
    FinishesInverse,
}
pub struct Interval {
    start: Time,
    end: Time,
}
impl Interval {
    pub fn new(start: Time, end: Time) -> Self {
        Self { start, end }
    }
    pub fn compare(&self, rhs: &Self) -> Relation {
        match self.start {
            // Self: [D,?] other: [?,?]
            Time::Definite(self_start) => match rhs.start {
                // self: [D,?] other: [D,?]
                Time::Definite(rhs_start) => match self.end {
                    //self: [D,D] other: [D,?]
                    Time::Definite(self_end) => match rhs.end {
                        //self: [D,D] other: [D,D]
                        Time::Definite(rhs_end) => {
                            if self_start < rhs_start {
                                if self_end < rhs_end {
                                    Relation::Overlaps
                                } else if self_end > rhs_end {
                                    Relation::DuringInverse
                                } else {
                                    todo!("self_end == rhs_end")
                                }
                            } else if self_start > rhs_start {
                                if self_end < rhs_end {
                                    Relation::During
                                } else if self_end > rhs_end {
                                    todo!("self_end > rhs_end")
                                } else {
                                    todo!("self_end == rhs_end")
                                }
                            } else {
                                //self_start==rhs_end

                                if self_end < rhs_end {
                                    Relation::Starts
                                } else if self_end > rhs_end {
                                    Relation::StartsInverse
                                } else {
                                    Relation::Equal
                                }
                            }
                        }
                        //self: [D,D] rhs: [D,oo]
                        Time::Infinity => todo!("rhs end infite"),
                    },
                    // self: [D,oo] other: [D,?]
                    Time::Infinity => match rhs.end {
                        //self: [D,oo] other: [D,D]
                        Time::Definite(_) => todo!("rhs end definte"),
                        //self: [D,oo] other: [D,oo]
                        Time::Infinity => {
                            if self_start < rhs_start {
                                todo!("self_start < rhs_start")
                            } else if self_start > rhs_start {
                                todo!("self_start > rhs_start")
                            } else {
                                Relation::Equal
                            }
                        }
                    },
                },
                // self: [D,?] other: [oo,?]
                Time::Infinity => match self.end {
                    // self: [D,D] other: [oo,?]
                    Time::Definite(_) => match rhs.end {
                        // self: [D,D] other: [oo,D]
                        Time::Definite(_) => todo!("rhs end definite"),
                        // self: [D,D] other: [oo, oo]
                        Time::Infinity => Relation::During,
                    },
                    // self: [D,oo] other: [oo,?]
                    Time::Infinity => match rhs.end {
                        // self: [D,oo] other: [oo,D]
                        Time::Definite(rhs_end) => {
                            if self_start < rhs_end {
                                Relation::OverlapsInverse
                            } else if self_start > rhs_end {
                                Relation::BeforeInverse
                            } else {
                                Relation::MeetsInverse
                            }
                        }
                        //self: [D,oo] other: [oo,oo]
                        Time::Infinity => todo!("rhs end infity"),
                    },
                },
            },
            // Self: [oo,?] other: [?,?]
            Time::Infinity => match rhs.start {
                //self: [oo,?] other: [D,??]
                Time::Definite(rhs_start) => match self.end {
                    //self: [oo,D] other: [D,??]
                    Time::Definite(self_end) => match rhs.end {
                        // self: [oo,D] other: [D,D]
                        Time::Definite(_) => todo!("rhs_end definte"),
                        //self: [oo,D], other: [D,oo]
                        Time::Infinity => {
                            if self_end < rhs_start {
                                Relation::Before
                            } else if self_end > rhs_start {
                                Relation::Overlaps
                            } else {
                                Relation::Meets
                            }
                        }
                    },
                    //self: [oo,oo], other: [D,?]
                    Time::Infinity => match rhs.end {
                        // self: [oo,oo], other: [D,D]
                        Time::Definite(_) => Relation::DuringInverse,
                        // self: [oo,oo] other: [D,oo]
                        Time::Infinity => todo!("rhs end infite"),
                    },
                },
                //self: [oo,?] other: [oo,??]
                Time::Infinity => match self.end {
                    //self: [oo, D] other: [oo,??]
                    Time::Definite(self_end) => match rhs.end {
                        //self: [oo, D] other: [oo,D]
                        Time::Definite(rhs_end) => {
                            if self_end < rhs_end {
                                Relation::Starts
                            } else if self_end > rhs_end {
                                Relation::StartsInverse
                            } else {
                                //both are the same
                                Relation::Equal
                            }
                        }
                        //self: [oo,D] other: [oo,oo]
                        Time::Infinity => Relation::Starts,
                    },
                    //self: [oo, oo] other: [oo, ??]
                    Time::Infinity => match rhs.end {
                        //self: [oo, oo] other: [oo,D]
                        Time::Definite(_) => Relation::StartsInverse,
                        //self: [oo,oo] other: [oo,oo]
                        Time::Infinity => Relation::Equal,
                    },
                },
            },
        }
    }
    pub fn complement(&self) -> Self {
        todo!()
    }
    pub fn compose(&self, rhs: &Self) -> Self {
        todo!()
    }
    pub fn intersection(&self, rhs: &Self) -> Self {
        todo!()
    }
    pub fn union(&self, rhs: &Self) -> Self {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Relation::DuringInverse;

    #[test]
    fn init() {
        let _ = Interval::new(Time::Infinity, Time::Definite(0));
    }
    #[test]
    fn infinite_equals() {
        let lhs = Interval::new(Time::Infinity, Time::Infinity);
        let rhs = Interval::new(Time::Infinity, Time::Infinity);
        assert_eq!(lhs.compare(&rhs), Relation::Equal);
    }
    #[test]
    fn before() {
        let lhs = Interval::new(Time::Infinity, Time::Definite(0));
        let rhs = Interval::new(Time::Definite(1), Time::Infinity);
        assert_eq!(lhs.compare(&rhs), Relation::Before);
        assert_eq!(rhs.compare(&lhs), Relation::BeforeInverse);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
    }
    #[test]
    fn meets() {
        let lhs = Interval::new(Time::Infinity, Time::Definite(0));
        let rhs = Interval::new(Time::Definite(0), Time::Infinity);
        assert_eq!(lhs.compare(&rhs), Relation::Meets);
        assert_eq!(rhs.compare(&lhs), Relation::MeetsInverse);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
    }
    #[test]
    fn overlaps() {
        let lhs = Interval::new(Time::Infinity, Time::Definite(0));
        let rhs = Interval::new(Time::Definite(-1), Time::Infinity);
        assert_eq!(lhs.compare(&rhs), Relation::Overlaps);
        assert_eq!(rhs.compare(&lhs), Relation::OverlapsInverse);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
        let lhs = Interval::new(Time::Definite(0), Time::Definite(2));
        let rhs = Interval::new(Time::Definite(1), Time::Definite(4));
        assert_eq!(lhs.compare(&rhs), Relation::Overlaps);
        assert_eq!(rhs.compare(&lhs), Relation::OverlapsInverse);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
    }
    #[test]
    fn during() {
        let lhs = Interval::new(Time::Infinity, Time::Infinity);
        let rhs = Interval::new(Time::Definite(-1), Time::Definite(1));
        assert_eq!(lhs.compare(&rhs), Relation::DuringInverse);
        assert_eq!(rhs.compare(&lhs), Relation::During);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
        let lhs = Interval::new(Time::Definite(0), Time::Definite(1));
        let rhs = Interval::new(Time::Definite(-1), Time::Definite(2));
        assert_eq!(lhs.compare(&rhs), Relation::During);
        assert_eq!(rhs.compare(&lhs), DuringInverse);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
    }
    #[test]
    fn starts() {
        let lhs = Interval::new(Time::Infinity, Time::Infinity);
        let rhs = Interval::new(Time::Infinity, Time::Definite(1));
        assert_eq!(lhs.compare(&rhs), Relation::StartsInverse);
        assert_eq!(rhs.compare(&lhs), Relation::Starts);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
        let lhs = Interval::new(Time::Definite(0), Time::Definite(10));
        let rhs = Interval::new(Time::Definite(0), Time::Definite(1));
        assert_eq!(lhs.compare(&rhs), Relation::StartsInverse);
        assert_eq!(rhs.compare(&lhs), Relation::Starts);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
        let lhs = Interval::new(Time::Infinity, Time::Infinity);
        let rhs = Interval::new(Time::Infinity, Time::Definite(0));
        assert_eq!(lhs.compare(&rhs), Relation::StartsInverse);
        assert_eq!(rhs.compare(&lhs), Relation::Starts);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);

        let lhs = Interval::new(Time::Infinity, Time::Definite(0));
        let rhs = Interval::new(Time::Infinity, Time::Infinity);
        assert_eq!(lhs.compare(&rhs), Relation::Starts);
        assert_eq!(rhs.compare(&lhs), Relation::StartsInverse);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
        let lhs = Interval::new(Time::Infinity, Time::Definite(0));
        let rhs = Interval::new(Time::Infinity, Time::Definite(1));
        assert_eq!(lhs.compare(&rhs), Relation::Starts);
        assert_eq!(rhs.compare(&lhs), Relation::StartsInverse);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
    }
    fn finishes() {
        let lhs = Interval::new(Time::Definite(0), Time::Definite(10));
        let rhs = Interval::new(Time::Definite(2), Time::Definite(10));
        assert_eq!(lhs.compare(&rhs), Relation::FinishesInverse);
        assert_eq!(rhs.compare(&lhs), Relation::Finishes);
        assert_eq!(lhs.compare(&lhs), Relation::Equal);
        assert_eq!(rhs.compare(&rhs), Relation::Equal);
    }
}
