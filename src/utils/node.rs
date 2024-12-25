use num::Num;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node<T>
where
    T: Num + Copy + Debug,
{
    pub x: T,
    pub y: T,
}

impl<T, TT> From<(TT, TT)> for Node<T>
where
    T: Num + Copy + Debug,
    TT: Into<T>,
{
    fn from((x, y): (TT, TT)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<T> Node<T>
where
    T: Num + Copy + Debug + Neg<Output=T> + Sub<Output=T> + Add<Output=T>,
{
    pub fn new<TT: Into<T>>(x: TT, y: TT) -> Self {
        Self { 
            x: x.into(), 
            y: y.into() 
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - num::one(),
            y: self.y,
        }
    }
    pub fn right(&self) -> Self {
        Self {
            x: self.x + num::one(),
            y: self.y,
        }
    }
    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + num::one(),
        }
    }
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - num::one(),
        }
    }

    pub fn left_up(&self) -> Self {
        Self {
            x: self.x - num::one(),
            y: self.y - num::one(),
        }
    }
    pub fn right_up(&self) -> Self {
        Self {
            x: self.x + num::one(),
            y: self.y - num::one(),
        }
    }
    pub fn left_down(&self) -> Self {
        Self {
            x: self.x - num::one(),
            y: self.y + num::one(),
        }
    }
    pub fn right_down(&self) -> Self {
        Self {
            x: self.x + num::one(),
            y: self.y + num::one(),
        }
    }
    /**
    Rotate clockwise about 90 degree origin
    */
    pub fn rot90_cw(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
    /**
    Rotate counter-clockwise about 90 degree origin
    */
    pub fn rot90_ccw(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn inverse(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

//=======================================================//
// &Node<T> op &Node<T> -> Node<T>
//=======================================================//

impl<'a, T> Add<&'a Node<T>> for &'a Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a, T> AddAssign<&'a Node<T>> for &'a mut Node<T>
where
    T: Num + Copy + Debug + AddAssign<T>,
{
    fn add_assign(&mut self, rhs: &'a Node<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<'a, T> Sub<&'a Node<T>> for &'a Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'a, T> Mul<&'a Node<T>> for &'a Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<'a, T> Div<&'a Node<T>> for &'a Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

//=======================================================//
// &Node<T> op Node<T> -> Node<T>
//=======================================================//

impl<T> Add<Node<T>> for &Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: Node<T>) -> Self::Output {
        self + &rhs
    }
}

impl<T> AddAssign<Node<T>> for &mut Node<T>
where
    T: Num + Copy + Debug + AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Node<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub<Node<T>> for &Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: Node<T>) -> Self::Output {
        self - &rhs
    }
}

impl<T> Mul<Node<T>> for &Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: Node<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T> Div<Node<T>> for &Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: Node<T>) -> Self::Output {
        self / &rhs
    }
}

//=======================================================//
// Node<T> op &Node<T> -> Node<T>
//=======================================================//

impl<'a, T> Add<&'a Node<T>> for Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: &'a Node<T>) -> Self::Output {
        &self + rhs
    }
}

impl<'a, T> AddAssign<&'a Node<T>> for Node<T>
where
    T: Num + Copy + Debug + AddAssign<T>,
{
    fn add_assign(&mut self, rhs: &'a Node<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<'a, T> Sub<&'a Node<T>> for Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: &'a Node<T>) -> Self::Output {
        &self - rhs
    }
}

impl<'a, T> Mul<&'a Node<T>> for Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: &'a Node<T>) -> Self::Output {
        &self * rhs
    }
}

impl<'a, T> Div<&'a Node<T>> for Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: &'a Node<T>) -> Self::Output {
        &self / rhs
    }
}

//=======================================================//
// Node<T> op Node<T> -> Node<T>
//=======================================================//


impl<T> Add<Node<T>> for Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<T> AddAssign<Node<T>> for Node<T>
where
    T: Num + Copy + Debug + AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Node<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub<Node<T>> for Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl<T> Mul<Node<T>> for Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<T> Div<Node<T>> for Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

//=======================================================//
// Node<T> op T -> Node<T>
//=======================================================//

impl<T> Add<T> for Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T> Sub<T> for Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T> Mul<T> for Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

//=======================================================//
// &Node<T> op T -> Node<T>
//=======================================================//

impl<T> Add<T> for &Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T> Sub<T> for &Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T> Mul<T> for &Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for &Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

//=======================================================//
// Node<T> op &T -> Node<T>
//=======================================================//

impl<'a, T> Add<&'a T> for Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: &'a T) -> Self::Output {
        &self + rhs
    }
}

impl<'a, T> Sub<&'a T> for Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: &'a T) -> Self::Output {
        &self - rhs
    }
}

impl<'a, T> Mul<&'a T> for Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: &'a T) -> Self::Output {
        &self * rhs
    }
}

impl<'a, T> Div<&'a T> for Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: &'a T) -> Self::Output {
        &self / rhs
    }
}

//=======================================================//
// &Node<T> op &T -> Node<T>
//=======================================================//

impl<'a, T> Add<&'a T> for &'a Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: &'a T) -> Self::Output {
        Self::Output {
            x: self.x + *rhs,
            y: self.y + *rhs,
        }
    }
}

impl<'a, T> Sub<&'a T> for &'a Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: &'a T) -> Self::Output {
        Self::Output {
            x: self.x - *rhs,
            y: self.y - *rhs,
        }
    }
}

impl<'a, T> Mul<&'a T> for &'a Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: &'a T) -> Self::Output {
        Self::Output {
            x: self.x * *rhs,
            y: self.y * *rhs,
        }
    }
}

impl<'a, T> Div<&'a T> for &'a Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: &'a T) -> Self::Output {
        Self::Output {
            x: self.x / *rhs,
            y: self.y / *rhs,
        }
    }
}

//=======================================================//
// Node<T> op (T, T) -> Node<T>
//=======================================================//

impl<T> Add<(T, T)> for Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: (T, T)) -> Self::Output {
        &self + rhs
    }
}

impl<T> Sub<(T, T)> for Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: (T, T)) -> Self::Output {
        &self - rhs
    }
}

impl<T> Mul<(T, T)> for Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: (T, T)) -> Self::Output {
        &self * rhs
    }
}

impl<T> Div<(T, T)> for Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: (T, T)) -> Self::Output {
        &self / rhs
    }
}

//=======================================================//
// &Node<T> op (T, T) -> Node<T>
//=======================================================//

impl<T> Add<(T, T)> for &Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: (T, T)) -> Self::Output {
        Self::Output {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl<T> Sub<(T, T)> for &Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: (T, T)) -> Self::Output {
        Self::Output {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl<T> Mul<(T, T)> for &Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: (T, T)) -> Self::Output {
        Self::Output {
            x: self.x * rhs.0,
            y: self.y * rhs.1,
        }
    }
}

impl<T> Div<(T, T)> for &Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: (T, T)) -> Self::Output {
        Self::Output {
            x: self.x / rhs.0,
            y: self.y / rhs.1,
        }
    }
}

//=======================================================//
// Node<T> op &(T, T) -> Node<T>
//=======================================================//

impl<'a, T> Add<&'a (T, T)> for Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: &'a (T, T)) -> Self::Output {
        &self + rhs
    }
}

impl<'a, T> Sub<&'a (T, T)> for Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: &'a (T, T)) -> Self::Output {
        &self - rhs
    }
}

impl<'a, T> Mul<&'a (T, T)> for Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: &'a (T, T)) -> Self::Output {
        &self * rhs
    }
}

impl<'a, T> Div<&'a (T, T)> for Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: &'a (T, T)) -> Self::Output {
        &self / rhs
    }
}

//=======================================================//
// &Node<T> op &(T, T) -> Node<T>
//=======================================================//

impl<'a, T> Add<&'a (T, T)> for &'a Node<T>
where
    T: Num + Copy + Debug + Add<Output=T>,
{
    type Output = Node<T>;

    fn add(self, rhs: &'a (T, T)) -> Self::Output {
        Self::Output {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl<'a, T> Sub<&'a (T, T)> for &'a Node<T>
where
    T: Num + Copy + Debug + Sub<Output=T>,
{
    type Output = Node<T>;

    fn sub(self, rhs: &'a (T, T)) -> Self::Output {
        Self::Output {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl<'a, T> Mul<&'a (T, T)> for &'a Node<T>
where
    T: Num + Copy + Debug + Mul<Output=T>,
{
    type Output = Node<T>;

    fn mul(self, rhs: &'a (T, T)) -> Self::Output {
        Self::Output {
            x: self.x * rhs.0,
            y: self.y * rhs.1,
        }
    }
}

impl<'a, T> Div<&'a (T, T)> for &'a Node<T>
where
    T: Num + Copy + Debug + Div<Output=T>,
{
    type Output = Node<T>;

    fn div(self, rhs: &'a (T, T)) -> Self::Output {
        Self::Output {
            x: self.x / rhs.0,
            y: self.y / rhs.1,
        }
    }
}