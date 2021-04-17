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

/// Useful macro to create a Num instance simple.
/// 
/// # Examples:
/// ```rust
/// use rcs_parser::{Num,num};
/// assert_eq!( Num{numbers: vec![]}, num![]);
/// assert_eq!( Num{numbers: vec![1,2,3,4]}, num![1,2,3,4]);
/// ```
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
    /// Shows that the Num is a valid revison number. 
    /// 
    /// # Examples:
    /// ```
    /// use rcs_parser::{Num,num};
    /// assert_eq!( true, num![1,2].is_valid_revision());
    /// assert_eq!( true, num![1,2,3,4].is_valid_revision());
    /// 
    /// assert_eq!( false, num![1].is_valid_revision());
    /// assert_eq!( false, num![1,2,3].is_valid_revision());
    /// assert_eq!( false, num![].is_valid_revision());
    /// assert_eq!( false, num![0].is_valid_revision());
    /// assert_eq!( false, num![1,0,2].is_valid_revision());
    /// assert_eq!( false, num![1,1,0].is_valid_revision());
    /// ```
    pub fn is_valid_revision(&self) -> bool {
        !self.numbers.is_empty()
            && self.is_revision()
            && self.numbers.iter().all( |n| *n > 0)
    }

    /// Shows if a num is a branch's number.
    /// 
    /// # Examples:
    /// ```
    /// use rcs_parser::{Num,num};
    /// assert_eq!(true, num![1].is_branch());
    /// assert_eq!(true, num![1,1,1].is_branch());
    /// 
    /// assert_eq!(false, num![1,1].is_branch());
    /// ```
    pub fn is_branch(&self) -> bool {
        self.numbers.len() % 2 == 1
    }

    /// Shows if a num is a revision's number.
    /// 
    /// # Examples:
    /// ```
    /// use rcs_parser::{Num,num};
    /// assert_eq!(true, num![1,1].is_revision());
    /// assert_eq!(true, num![1,1,1,1].is_revision());
    /// 
    /// assert_eq!(false, num![1].is_revision());
    /// assert_eq!(false, num![1,1,1].is_revision());
    /// ```
    pub fn is_revision(&self) -> bool {
        !self.is_branch()
    }

    /// Retreives the branching point of a num.
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

    /// Retreives all branching points of a num.
    /// 
    /// # Examples:
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
            i += 2;
        }
        points
    }
}
