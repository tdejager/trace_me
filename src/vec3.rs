use rand::Rng;
use std::ops;
use std::ops::Neg;

#[derive(Debug, Copy, Clone)]
/// A simple Vec3 implementation
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns the lenght for a Vec3
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the squared length for the Vec3
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the zero vector
    pub fn zero() -> Self {
        Vec3::new(0., 0., 0.)
    }

    /// Dot product
    pub fn dot(&self, b: &Vec3) -> f64 {
        dot(&self, b)
    }

    /// Cross product
    pub fn cross(&self, b: &Vec3) -> Self {
        cross(self, &b)
    }

    /// Generates a random vector between [0,1]
    pub fn random() -> Self {
        let mut rnd = rand::thread_rng();
        Self::new(rnd.gen(), rnd.gen(), rnd.gen())
    }

    /// Generates a random vector between the range [min, max]
    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rnd = rand::thread_rng();
        Self::new(
            rnd.gen_range(min, max),
            rnd.gen_range(min, max),
            rnd.gen_range(min, max),
        )
    }

    /// Returns the unit vector of this vector
    pub fn unit_vector(&self) -> Self {
        unit_vector(*self)
    }

    /// Return a random vector in the unit sphere
    pub fn random_in_unit_sphere() -> Self {
        // Just loop till you find one
        loop {
            let p = Self::random();
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    /// Random unit vector according to lambertian distribution
    pub fn random_unit_vector() -> Self {
        let mut rnd = rand::thread_rng();
        let a = rnd.gen_range(0., 2. * std::f64::consts::PI);
        let z = rnd.gen_range(-1., 1.);
        let r = (1. as f64 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    /// Reflect the vector with regards to the normal n
    pub fn reflect(&self, n: &Vec3) -> Self {
        *self - 2. * self.dot(n) * (*n)
    }

    /// Refract the vector with regards to the normal and
    /// refraction index
    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = -self.dot(n);
        let r_out_perp = etai_over_etat * (*self + (*n * cos_theta));
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

/// Calculates the dot product for the Vec3
pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// Calculates the cross product for the Vec3
pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    let x = a.y * b.z - a.z * b.y;
    let y = a.z * b.x - a.x * b.z;
    let z = a.x * b.y - a.y * b.x;
    Vec3::new(x, y, z)
}

/// Returns a unit vector for a given Vec3
pub fn unit_vector(a: Vec3) -> Vec3 {
    a / a.length()
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub type Point3 = Vec3;
