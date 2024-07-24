use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

#[derive(Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize = R>([[f32; R]; C]);

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub const fn new() -> Self {
        Self([[0.0; R]; C])
    }
    pub fn ident() -> Self {
        if C == R {
            let mut m = Self::new();
            for i in 0..C {
                m.0[i][i] = 1.0;
            }
            m
        } else {
            Self::new()
        }
    }
    pub fn transpose(&self) -> Matrix<C, R> {
        let mut res = Matrix::<C, R>::new();
        for i in 0..C {
            for j in 0..R {
                res.0[j][i] = self.0[i][j]
            }
        }
        res
    }
}

impl<const S: usize> Matrix<S, S> {
    pub fn inverse(&self) -> Option<Self> {
        let mut mat = *self;
        let mut res = Self::ident();

        for i in 0..S {
            let diag = mat.0[i][i];
            if diag == 0.0 {
                return None;
            }

            for j in 0..S {
                mat.0[i][j] /= diag;
                res.0[i][j] /= diag;
            }

            for k in 0..S {
                if i != k {
                    let fac = mat.0[k][i];
                    for j in 0..S {
                        mat.0[k][j] -= fac * mat.0[i][j];
                        res.0[k][j] -= fac * res.0[i][j];
                    }
                }
            }
        }

        Some(res)
    }
}

impl<const R: usize, const C: usize> Add for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::Output::new();
        for i in 0..C {
            for j in 0..R {
                res.0[i][j] = self.0[i][j] + rhs.0[i][j];
            }
        }
        res
    }
}

impl<const R: usize, const C: usize> AddAssign for Matrix<R, C> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..C {
            for j in 0..R {
                self.0[i][j] += rhs.0[i][j];
            }
        }
    }
}

impl<const R: usize, const C: usize> Sub for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Self::Output::new();
        for i in 0..C {
            for j in 0..R {
                res.0[i][j] = self.0[i][j] - rhs.0[i][j];
            }
        }
        res
    }
}

impl<const R: usize, const C: usize> SubAssign for Matrix<R, C> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..C {
            for j in 0..R {
                self.0[i][j] -= rhs.0[i][j];
            }
        }
    }
}

impl Matrix<4, 4> {
    pub const fn translation(t: Vec3) -> Self {
        Self([
            [1.0, 0.0, 0.0, t.x],
            [0.0, 1.0, 0.0, t.y],
            [0.0, 0.0, 1.0, t.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub const fn scale(s: Vec3) -> Self {
        Self([
            [s.x, 0.0, 0.0, 0.0],
            [0.0, s.y, 0.0, 0.0],
            [0.0, 0.0, s.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn rotation(axis: Vec3, angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        let cc = 1.0 - c;
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        Self([
            [x * x * cc + c, x * y * cc - z * s, x * z * cc - y * s, 0.0],
            [x * y * cc + z * s, y * y * cc + c, z * y * cc - x * s, 0.0],
            [x * z * cc - y * s, y * z * cc + x * s, z * z * cc + c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Matrix<4, 1> {
    pub const fn point(p: Vec3) -> Self {
        Self([[p.x, p.y, p.z, 1.0]])
    }
}

impl<const R: usize, const C: usize> From<[[f32; R]; C]> for Matrix<R, C> {
    fn from(value: [[f32; R]; C]) -> Self {
        Self(value)
    }
}

impl<const S: usize> From<[f32; S]> for Matrix<S, 1> {
    fn from(value: [f32; S]) -> Self {
        Self([value])
    }
}

impl<const R: usize, const B: usize, const C: usize> Mul<Matrix<R, B>> for Matrix<B, C> {
    type Output = Matrix<R, C>;
    fn mul(self, rhs: Matrix<R, B>) -> Self::Output {
        let mut res = Self::Output::new();
        for i in 0..C {
            for j in 0..R {
                for k in 0..B {
                    res.0[i][j] += self.0[i][k] * rhs.0[k][j];
                }
            }
        }
        res
    }
}

impl<const R: usize, const C: usize> Debug for Matrix<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..C {
            f.write_fmt(format_args!("{:?}\n", self.0[i]))?
        }
        Ok(())
    }
}

pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({:.2}, {:.2}, {:.2})",
            self.x, self.y, self.z
        ))
    }
}

pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl From<Vec3> for Matrix<4, 4> {
    fn from(value: Vec3) -> Self {
        let mut m = Self::ident();
        m.0[3][0] = value.x;
        m.0[3][1] = value.y;
        m.0[3][2] = value.z;
        m
    }
}

impl From<Vec4> for Matrix<4, 4> {
    fn from(v: Vec4) -> Self {
        let (s, c) = v.w.sin_cos();
        let cc = 1.0 - c;
        let x = v.x;
        let y = v.y;
        let z = v.z;
        Self::from([
            [x * x * cc + c, x * y * cc - z * s, x * z * cc - y * s, 0.0],
            [x * y * cc + z * s, y * y * cc + c, z * y * cc - x * s, 0.0],
            [x * z * cc - y * s, y * z * cc + x * s, z * z * cc + c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Matrix<4, 1>> for Vec3 {
    fn from(v: Matrix<4, 1>) -> Self {
        Vec3::new(v.0[0][0], v.0[0][1], v.0[0][2])
    }
}
