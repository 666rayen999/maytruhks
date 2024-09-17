use std::{
    fmt::Debug,
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

#[derive(Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize = R>([[f32; R]; C]);

impl<const R: usize, const C: usize> Matrix<R, C> {
    const unsafe fn empty() -> Self {
        MaybeUninit::uninit().assume_init()
    }
    pub const fn new() -> Self {
        Self([[0.0; R]; C])
    }
    pub fn ident() -> Self {
        let mut m = Self::new();
        for i in 0..C.min(R) {
            m.0[i][i] = 1.0;
        }
        m
    }
    pub fn transpose(&self) -> Matrix<C, R> {
        let mut res = unsafe { Matrix::<C, R>::empty() };
        for i in 0..C {
            for j in 0..R {
                res.0[j][i] = self.0[i][j]
            }
        }
        res
    }
}

#[macro_export]
macro_rules! matrix {
    () => {
        Matrix::new()
    };
    ($s:literal) => {
        Matrix::<$s, $s>::new()
    };
    ($c:literal x $r:literal) => {
        Matrix::<$c, $r>::new()
    };
}

#[macro_export]
macro_rules! identity {
    () => {
        Matrix::ident()
    };
    ($s:literal) => {
        Matrix::<$s, $s>::ident()
    };
    ($c:literal x $r:literal) => {
        Matrix::<$c, $r>::ident()
    };
}

#[macro_export]
macro_rules! point {
    () => {
        point!(0.0, 0.0, 0.0)
    };
    ($x:literal, $y:literal, $z:literal) => {
        Matrix::point(Vec3::new($x, $y, $z))
    };
    ($x:literal $y:literal $z:literal) => {
        point!($x, $y, $z)
    };
}

#[macro_export]
macro_rules! translate {
    () => {
        identity!(4)
    };
    ($x:literal, $y:literal, $z:literal) => {
        Matrix::translation(Vec3::new($x, $y, $z))
    };
    ($x:literal $y:literal $z:literal) => {
        translate!($x, $y, $z)
    };
    (x $p:literal) => {
        translate!($p, 0.0, 0.0)
    };
    (y $p:literal) => {
        translate!(0.0, $p, 0.0)
    };
    (z $p:literal) => {
        translate!(0.0, 0.0, $p)
    };
}

#[macro_export]
macro_rules! rotate {
    () => {
        identity!(4)
    };
    ($x:literal, $y:literal, $z:literal; $a:literal) => {
        Matrix::rotation(Vec3::new($x, $y, $z), $a)
    };
    ($x:literal $y:literal $z:literal, $a:literal) => {
        rotate!($x, $y, $z; $a)
    };
    (x $a:literal) => {
        rotate!(1.0, 0.0, 0.0; $a)
    };
    (y $a:literal) => {
        rotate!(0.0, 1.0, 0.0; $a)
    };
    (z $a:literal) => {
        rotate!(0.0, 0.0, 1.0; $a)
    };
}

#[macro_export]
macro_rules! scale {
    () => {
        identity!(4)
    };
    ($x:literal, $y:literal, $z:literal) => {
        Matrix::scale(Vec3::new($x, $y, $z))
    };
    ($x:literal $y:literal $z:literal) => {
        scale!($x, $y, $z)
    };
    (x $s:literal) => {
        scale!($s, 1.0, 1.0)
    };
    (y $s:literal) => {
        scale!(1.0, $s, 1.0)
    };
    (z $s:literal) => {
        scale!(1.0, 1.0, $s)
    };
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

            let diag = 1.0 / diag;

            for j in 0..S {
                mat.0[i][j] *= diag;
                res.0[i][j] *= diag;
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
        let mut res = unsafe { Self::Output::empty() };
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
        let mut res = unsafe { Self::Output::empty() };
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
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [t.x, t.y, t.z, 1.0],
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
        let (s, cc) = angle.sin_cos();
        let c = 1.0 - cc;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        let xc = x * c;
        let yc = y * c;
        let zc = z * c;

        let xyc = y * xc;
        let yzc = z * yc;
        let zxc = x * zc;

        let xxc = x * xc;
        let yyc = y * yc;
        let zzc = z * zc;

        let ys = y * s;
        let xs = x * s;
        let zs = z * s;

        Self([
            [xxc + cc, xyc - zs, zxc - ys, 0.0],
            [xyc + zs, yyc + cc, yzc - xs, 0.0],
            [zxc - ys, yzc + xs, zzc + cc, 0.0],
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
        let mut res = unsafe { Self::Output::empty() };
        for i in 0..C {
            for j in 0..R {
                res.0[i][j] = 0.0;
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
        f.write_str("\n")?;
        for x in self.0 {
            f.debug_list().entries(x).finish()?;
            f.write_str("\n")?;
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
        Self::rotation(Vec3::new(v.x, v.y, v.z), v.w)
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
