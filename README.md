# Maytruhks

A high-performance matrix library for Rust, tailored for game development. This library provides essential matrix operations, including addition, subtraction, multiplication, inversion, and transposition. It also includes support for creating rotation, translation, and scale matrices.

### Features
- Matrix Addition
- Matrix Subtraction
- Matrix Multiplication
- Matrix Inversion
- Matrix Transposition
- Rotation Matrices
- Translation Matrices
- Scale Matrices

### Usage
###### Creation
```rust
  // empty matrix
  let matrix = Matrix::<3, 3>::new();

  // from 2d array
  let matrix = Matrix::from([
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
    [7.0, 8.0, 9.0],
  ]);

  // identity matrix
  let matrix = Matrix::<3, 3>::ident();
```
###### 3D matrices
```rust
  // translation matrix that translate 1 in x axis
  let t = Matrix::tranlation(Vec3::new(1.0, 0.0, 0.0));

  // rotation matrix that rotate on Y axis 180°
  let r = Matrix::rotation(Vec3::new(0.0, 1.0, 0.0), PI);

  // scale matrix that scale x2
  let s = Matrix::scale(Vec3::new(2.0, 2.0, 2.0));

  // create point in the space x = 1.0, y = 0.0, z = 0.0
  let p = Matrix::point(Vec3::new(1.0, 0.0, 0.0));
  
  // rotate -> scale -> translate
  let p = p * r * s * t;
```
###### Math
```rust
  let mut mat_1 = ...;
  let mut mat_2 = ...;

  let add = mat_1 + mat_2;
  mat_1 += mat_2;

  let sub = mat_1 - mat_2;
  mat_2 -= mat_1;

  let mul = mat_1 * mat_2;

  if let Some(inv) = mat_1.inverse() {
    // it has and inverse
  }

  let trans = mat_1.transpose();
```

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### License

This project is licensed under the MIT License.
