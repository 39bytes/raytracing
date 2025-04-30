use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub const ZERO: Self = Self(0.0, 0.0, 0.0);

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn with_x(x: f64) -> Self {
        Vec3(x, 0.0, 0.0)
    }

    pub fn with_y(y: f64) -> Self {
        Vec3(0.0, y, 0.0)
    }

    pub fn with_z(z: f64) -> Self {
        Vec3(0.0, 0.0, z)
    }

    pub fn random() -> Self {
        Vec3::new(rand::random(), rand::random(), rand::random())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            rand::random_range(min..max),
            rand::random_range(min..max),
            rand::random_range(min..max),
        )
    }

    pub fn random_unit() -> Self {
        loop {
            let p = Vec3::random();
            let mag_sq = p.magnitude_squared();
            if 1e-160 < mag_sq && mag_sq <= 1.0 {
                return p / mag_sq.sqrt();
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                rand::random_range(-1.0..1.0),
                rand::random_range(-1.0..1.0),
                0.0,
            );
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(norm: Vec3) -> Self {
        let unit = Vec3::random_unit();
        if unit.dot(norm) > 0.0 { unit } else { -unit }
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.magnitude_squared())
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Self) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn near_zero(&self) -> bool {
        let threshold = 1e-8;
        self.0.abs() < threshold && self.1.abs() < threshold && self.2.abs() < threshold
    }

    pub fn reflect(v: Vec3, norm: Vec3) -> Vec3 {
        v - 2.0 * v.dot(norm) * norm
    }

    pub fn refract(v: Vec3, norm: Vec3, refraction_ratio: f64) -> Vec3 {
        let cos = f64::min((-v).dot(norm), 1.0);

        let perp = refraction_ratio * (v + cos * norm);
        let parallel = -(1.0 - perp.magnitude_squared()).sqrt() * norm;

        perp + parallel
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}
