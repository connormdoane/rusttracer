#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3f = Vec3<f64>;
pub type Vec3i = Vec3<i64>;

impl<T> PartialEq for Vec3<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
	self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl<T> Eq for Vec3<T> where T: Eq {}

impl<T> std::ops::Add<Vec3<T>> for Vec3<T> where T: std::ops::Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
	let op: Self::Output = Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z};
	op
    }
}

impl<T> std::ops::Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, xyz: usize) -> &Self::Output {
	match xyz {
	    0 => &self.x,
	    1 => &self.y,
	    _ => &self.z,
	}
    }
}

impl<T> std::ops::Sub<Vec3<T>> for Vec3<T> where T: std::ops::Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
	let op: Self::Output = Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z};
	op
    }
}

impl<T> std::ops::Mul<Vec3<T>> for Vec3<T> where T: std::ops::Mul<Output = T> + std::ops::MulAssign {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
	let op: Self::Output = Vec3{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z};
	op
    }
}

//impl<T> std::ops::Mul<T> for Vec3<T> where T: std::ops::Mul<Output = T> + std::ops::MulAssign {
//    type Output = Self;
//
//    fn mul(self, rhs: T) -> Self::Output {
//	let op: Self::Output = Vec3{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs};
//	op
//    }
//}

impl<T> Vec3<T> where T: std::ops::Mul<Output = T> + std::ops::MulAssign + std::ops::Add<Output = T> + std::fmt::Debug {
    pub fn dot_product(self, other: Vec3<T>) -> T {
	let f: T = (self.x * other.x) + (self.y * other.y) + (self.z * other.z); 
	f
    }
}

impl Vec3f {
    pub fn normalize(self) -> Vec3f {
	let norm: f64 = f64::sqrt(self.x*self.x+self.y*self.y+self.z*self.z);
	let op: Vec3f = Vec3f{x: self.x*(1./norm), y: self.y*(1./norm), z: self.z*(1./norm)};
	op
    }
}
