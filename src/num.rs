/// Num stores an RCS revision number as vector of unsigned integers.
///
/// E.g.: 1.2.3.4 will be represented as :
/// ```rust
/// # use rcs_parser::Num;
/// # fn not_needed()-> Num{
/// Num{numbers:vec![1,2,3,4]}
/// # }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Num {
    ///The numbers of a revision number
    pub numbers: Vec<u32>,
}

#[macro_export]
macro_rules! num {
    ( ) => { Num{numbers: Vec::new()} };
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Num{numbers: temp_vec}
        }
    };
}

impl Num {
    /// ```
    /// use rcs_parser::{Num,num};
    /// assert_eq!( false, num![1].is_valid_revision());
    /// assert_eq!( true, num![1,2].is_valid_revision());
    /// assert_eq!( false, num![1,2,3].is_valid_revision());
    /// assert_eq!( true, num![1,2,3,4].is_valid_revision());
    /// assert_eq!( false, num![].is_valid_revision());
    /// assert_eq!( false, num![0].is_valid_revision());
    /// assert_eq!( false, num![1,0,2].is_valid_revision());
    /// assert_eq!( false, num![1,1,0].is_valid_revision());
    /// ```
    pub fn is_valid_revision(&self) -> bool {
        self.numbers.len() > 0
            && self.is_revision()
            && self.numbers.iter().fold(true, |s, n| s && *n > 0)
    }

    /// ```
    /// use rcs_parser::{Num,num};
    /// assert_eq!(true, num![1].is_branch());
    /// assert_eq!(true, num![1,1,1].is_branch());
    /// assert_eq!(false, num![1,1].is_branch());
    /// ```
    pub fn is_branch(&self) -> bool {
        self.numbers.len() % 2 == 1
    }

    /// ```
    /// use rcs_parser::{Num,num};
    /// assert_eq!(true, num![1,1].is_revision());
    /// assert_eq!(true, num![1,1,1,1].is_revision());
    /// assert_eq!(false, num![1].is_revision());
    /// assert_eq!(false, num![1,1,1].is_revision());
    /// ```
    pub fn is_revision(&self) -> bool {
        !self.is_branch()
    }

    /// ```
    /// use rcs_parser::{Num,num};
    /// assert_eq!(num![1,2], num![1,2,3].get_branching_point());
    /// assert_eq!(num![1,2], num![1,2,3,4].get_branching_point());
    /// ```
    pub fn get_branching_point(&self) -> Num {
        if self.is_branch() {
            let numbers = Vec::from(&(self.numbers[0..self.numbers.len() - 1]));
            Num { numbers }
        } else {
            let numbers = Vec::from(&(self.numbers[0..self.numbers.len() - 2]));
            Num { numbers }
        }
    }

    /// ```
    /// use rcs_parser::{Num,num};
    /// assert_eq!(vec![num![1,2], num![1,2,3,4], num![1,2,3,4,5,6]] , num![1,2,3,4,5,6,7,8].get_branching_points());
    /// ```
    pub fn get_branching_points(&self) -> Vec<Num> {
        let mut points = Vec::<Num>::new();
        let mut i = 2;
        while i < self.numbers.len() {
            let numbers = Vec::from(&(self.numbers[0..i]));
            points.push(Num { numbers });
            i = i + 2;
        }
        points
    }
}
