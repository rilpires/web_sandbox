use std::{cmp::{max, min}, fmt::Display, ops::{Mul, MulAssign}, process::Output};


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2<T> where T: Mul + MulAssign + Copy + Display {
    pub x: T,
    pub y: T,
}


impl<T> Vector2<T> where T: Mul + MulAssign + Copy + Display {

    // Returns self
    pub fn scale<T2>(&mut self, s: T2) -> &mut Self
    where 
        T: Mul<T2, Output = T> + MulAssign<T2> + Copy,
        T2: Copy
    {
        self.x *= s; self.y *= s;
        self
    }

    // Returns as a tuple of any type
    pub fn convert<T2>(&self) -> Vector2<T2>
    where T2: Mul + MulAssign + From<T> + Copy + Display
    {
        return Vector2 {
            x: T2::from(self.x),
            y: T2::from(self.y),
        };
    }
}

impl<T> Display for Vector2<T> where T : Mul + MulAssign + Copy + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


#[derive(Clone)]
pub struct GridMap<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}

impl<T> GridMap<T> 
where T: Clone
{
    pub fn new(w:usize, h:usize, default:T) -> Self {
        GridMap {
            width: w as usize,
            height: h as usize,
            grid: vec![default; w*h]
        }
    }

    pub fn get(&self, x:usize, y:usize) -> &T {
        return &(self.grid[x + y*self.width]);
    }

    pub fn get_mut(&mut self, x:usize, y:usize) -> &mut T {
        unsafe {
            return self.grid.get_unchecked_mut(x + y*self.width);
        }
    }

    pub fn set(&mut self, x:usize, y:usize, new_val:T) {
        self.grid[x + y*self.width] = new_val;
    }
    pub fn swap(&mut self, x1:usize, y1:usize, x2:usize, y2:usize) {
        self.grid.swap(x1 + y1*self.width, x2 + y2*self.width);
    }

    pub fn set_neighbor(&mut self, x:i32, y:i32, new_val:T) {
        for sanitized_x in (max(0,x-1)..=min(x+1,self.width as i32-1)) {
            for sanitized_y in (max(0,y-1)..=min(y+1,self.height as i32-1)) {
                self.set(sanitized_x as usize, sanitized_y as usize, new_val.clone());
            }        
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.grid.iter_mut()
    }

}
