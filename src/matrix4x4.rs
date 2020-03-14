use crate::vector2::{Vector2};

use std::fmt;
use std::ops::{Index};

pub struct Matrix4x4 {
    // The matrix is represented as a one-dimensional arrray in column-major order
    // So far I haven't found a good reason to use a multi-dimensional array in code
    array: [f32; 16]
}

// LEARN: Default Trait in Rust
// The "Default" trait can be implemented as a way to easily fall back to a default value for a struct and its members.
// It can be used in two ways:
// - By deriving the default trait on the struct itself. This works if all members of the struct implements Default themselves.
// - By manually implementing the method default()
// I manually implement the method default here
// TODO: From what I understand, I don't actually have to implement Default here. Arrays should be able to be Default so long as they are under 32? in size and use Default types. 
impl Default for Matrix4x4 {
    fn default() -> Self {
        Matrix4x4 {
            array: [
                0.0, 0.0, 0.0, 0.0, 
                0.0, 0.0, 0.0, 0.0, 
                0.0, 0.0, 0.0, 0.0, 
                0.0, 0.0, 0.0, 0.0 ]
        }
    }
}

impl Index<[usize; 2]> for Matrix4x4 {
    // TODO: Gotta learn exactly what this Output pattern is about when implementing some traits..
    type Output = f32;

    fn index(&self, index: [usize; 2]) -> &f32 {
        let requested_column = index[1];
        let requested_row = index[0];

        // LEARN: I actually panic in library code here.
        // Reason: Providing an index outside the range of the 4x4 matrix represents a flat out incorrect call to this indexing method.
        // The client is violating the preconditions of the function, and so I will refuse to even accept the input as it represents a bug in the calling code.
        // This is NOT something that should occur run-time on the client-side.
        // LEARN: Think about pros / cons about this in terms of users using the library... and what good alternatives could be.
        // Definite pro: Client code gets to write quick, short and to-the-point indexing code that doesn't have to deal with a Result and unpacking it, etc, which can very quickly write long hard-to-read code
        // If you're dealing with matrix math.
        if requested_column > 3 {
            panic!("You requested column {}, but the max allowed index is 3!", requested_column);
        }

        if requested_row > 3 {
            panic!("You requested row {}, but the max allowed index is 3!", requested_column);
        }

        &self.array[ requested_column * 4 + requested_row  ]
    }
}

impl Matrix4x4 {
    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32) -> Matrix4x4 {
            Matrix4x4 {
                array: [ 
                    m00, m10, m20, m30, m01, m11, m21, m31, m02, m12, m22, m32, m03, m13, m23, m33 ]
            }
    }

    pub fn first(&self) -> &f32 {
        &self.array[0]
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    // TODO: Should take a reference to a vector2 instead of moving it
    // TODO: Need to write test for this method
    pub fn translate(&self, vector2: Vector2) -> Matrix4x4 {
        let translate_matrix = Matrix4x4::new(
            1.0, 0.0, 0.0, vector2.x, 
            0.0, 1.0, 0.0, vector2.y,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0);

        self.mul(translate_matrix)
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4x4 {
        let orthographic_projection = Matrix4x4::new(
            2.0 / (right - left), 0.0,                0.0,              -((right + left)/(right - left)), 
            0.0,                  2.0/(top - bottom), 0.0,              -((top + bottom)/(top - bottom)), 
            0.0,                  0.0,                2.0/(far - near), -((far + near)/(far - near)), 
            0.0,                  0.0,                0.0,              1.0);

        orthographic_projection
    }

    // TODO: Should take a reference to another matrix instead of moving it
    pub fn mul(&self, matrix4x4: Matrix4x4) -> Matrix4x4 {
        // TODO: Now I don't know much about SIMD instructions yet, but it might have something to do with doing calculations in single instructions...
        // That is definitely not what I'm doing here (I think???? Maybe???). Have to read up on all that at some point.
        // For now I have bigger fish to fry.
        let m00 = self[[0, 0]] * matrix4x4[[0, 0]] + self[[0, 1]] * matrix4x4[[1, 0]] + self[[0, 2]] * matrix4x4[[2, 0]] + self[[0, 3]] * matrix4x4[[3, 0]];
        let m01 = self[[0, 0]] * matrix4x4[[0, 1]] + self[[0, 1]] * matrix4x4[[1, 1]] + self[[0, 2]] * matrix4x4[[2, 1]] + self[[0, 3]] * matrix4x4[[3, 1]];
        let m02 = self[[0, 0]] * matrix4x4[[0, 2]] + self[[0, 1]] * matrix4x4[[1, 2]] + self[[0, 2]] * matrix4x4[[2, 2]] + self[[0, 3]] * matrix4x4[[3, 2]];
        let m03 = self[[0, 0]] * matrix4x4[[0, 3]] + self[[0, 1]] * matrix4x4[[1, 3]] + self[[0, 2]] * matrix4x4[[2, 3]] + self[[0, 3]] * matrix4x4[[3, 3]];

        let m10 = self[[1, 0]] * matrix4x4[[0, 0]] + self[[1, 1]] * matrix4x4[[1, 0]] + self[[1, 2]] * matrix4x4[[2, 0]] + self[[1, 3]] * matrix4x4[[3, 0]];
        let m11 = self[[1, 0]] * matrix4x4[[0, 1]] + self[[1, 1]] * matrix4x4[[1, 1]] + self[[1, 2]] * matrix4x4[[2, 1]] + self[[1, 3]] * matrix4x4[[3, 1]];
        let m12 = self[[1, 0]] * matrix4x4[[0, 2]] + self[[1, 1]] * matrix4x4[[1, 2]] + self[[1, 2]] * matrix4x4[[2, 2]] + self[[1, 3]] * matrix4x4[[3, 2]];
        let m13 = self[[1, 0]] * matrix4x4[[0, 3]] + self[[1, 1]] * matrix4x4[[1, 3]] + self[[1, 2]] * matrix4x4[[2, 3]] + self[[1, 3]] * matrix4x4[[3, 3]];

        let m20 = self[[2, 0]] * matrix4x4[[0, 0]] + self[[2, 1]] * matrix4x4[[1, 0]] + self[[2, 2]] * matrix4x4[[2, 0]] + self[[2, 3]] * matrix4x4[[3, 0]];
        let m21 = self[[2, 0]] * matrix4x4[[0, 1]] + self[[2, 1]] * matrix4x4[[1, 1]] + self[[2, 2]] * matrix4x4[[2, 1]] + self[[2, 3]] * matrix4x4[[3, 1]];
        let m22 = self[[2, 0]] * matrix4x4[[0, 2]] + self[[2, 1]] * matrix4x4[[1, 2]] + self[[2, 2]] * matrix4x4[[2, 2]] + self[[2, 3]] * matrix4x4[[3, 2]];
        let m23 = self[[2, 0]] * matrix4x4[[0, 3]] + self[[2, 1]] * matrix4x4[[1, 3]] + self[[2, 2]] * matrix4x4[[2, 3]] + self[[2, 3]] * matrix4x4[[3, 3]];

        let m30 = self[[3, 0]] * matrix4x4[[0, 0]] + self[[3, 1]] * matrix4x4[[1, 0]] + self[[3, 2]] * matrix4x4[[2, 0]] + self[[3, 3]] * matrix4x4[[3, 0]];
        let m31 = self[[3, 0]] * matrix4x4[[0, 1]] + self[[3, 1]] * matrix4x4[[1, 1]] + self[[3, 2]] * matrix4x4[[2, 1]] + self[[3, 3]] * matrix4x4[[3, 1]];
        let m32 = self[[3, 0]] * matrix4x4[[0, 2]] + self[[3, 1]] * matrix4x4[[1, 2]] + self[[3, 2]] * matrix4x4[[2, 2]] + self[[3, 3]] * matrix4x4[[3, 2]];
        let m33 = self[[3, 0]] * matrix4x4[[0, 3]] + self[[3, 1]] * matrix4x4[[1, 3]] + self[[3, 2]] * matrix4x4[[2, 3]] + self[[3, 3]] * matrix4x4[[3, 3]];

        let concatenated_matrix = Matrix4x4::new(
            m00, m01, m02, m03, 
            m10, m11, m12, m13, 
            m20, m21, m22, m23, 
            m30, m31, m32, m33);

        concatenated_matrix
    }
}

impl fmt::Debug for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Can you call multiple write! macros per line instead of having all in one call??
        write!(f, "{},{},{},{}\n{},{},{},{}\n{},{},{},{}\n{},{},{},{}",
                self[[0, 0]], self[[0, 1]], self[[0, 2]], self[[0, 3]],
                self[[1, 0]], self[[1, 1]], self[[1, 2]], self[[1, 3]],
                self[[2, 0]], self[[2, 1]], self[[2, 2]], self[[2, 3]],
                self[[3, 0]], self[[3, 1]], self[[3, 2]], self[[3, 3]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_orthographic_projection_construction() {
        // Act
        let orthographic_projection = Matrix4x4::orthographic(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0);

        println!("{:?}", orthographic_projection);

        // Assert
        assert!( approx_eq!(f32, orthographic_projection[[0, 0]], 0.002, epsilon = 0.0001) );
        assert_eq!( orthographic_projection[[0, 1]], 0.0 );
        assert_eq!( orthographic_projection[[0, 2]], 0.0 );
        assert_eq!( orthographic_projection[[0, 3]], -1.0 );

        assert_eq!( orthographic_projection[[1, 0]], 0.0 );
        assert!( approx_eq!(f32, orthographic_projection[[1, 1]], -0.003, epsilon = 0.0004) );
        assert_eq!( orthographic_projection[[1, 2]], 0.0 );
        assert_eq!( orthographic_projection[[1, 3]], 1.0 );

        assert_eq!( orthographic_projection[[2, 0]], 0.0 );
        assert_eq!( orthographic_projection[[2, 1]], 0.0 );
        assert_eq!( orthographic_projection[[2, 2]], 1.0 );
        assert_eq!( orthographic_projection[[2, 3]], 0.0 );

        assert_eq!( orthographic_projection[[3, 0]], 0.0 );
        assert_eq!( orthographic_projection[[3, 1]], 0.0 );
        assert_eq!( orthographic_projection[[3, 2]], 0.0 );
        assert_eq!( orthographic_projection[[3, 3]], 1.0 );
    }

    #[test]
    fn test_matrix_mul() {
        // Arrange
        let matrix_a = Matrix4x4::new(
            1.0, 2.0, 3.0, 4.0, 
            5.0, 6.0, 7.0, 8.0, 
            9.0, 10.0, 11.0, 12.0, 
            13.0, 14.0, 15.0, 16.0);

        let matrix_b = Matrix4x4::new(
            16.0, 15.0, 14.0, 13.0, 
            12.0, 11.0, 10.0, 9.0, 
            8.0, 7.0, 6.0, 5.0, 
            4.0, 3.0, 2.0, 1.0);

        // Act
        let multiplication_result = matrix_a.mul(matrix_b);

        // Assert
        // TODO: Do actual proper assertion here
        println!("{:?}", multiplication_result);
    }

    #[test]
    fn test_pretty_print() {
        // Arrange
        let matrix_a = Matrix4x4::identity();

        // Act
        println!("{:?}", matrix_a);
    }

    #[test]
    fn test_indexing() {
        // Arrange
        let matrix_a = Matrix4x4::new(
            1.0,  2.0,   3.0,    4.0, 
            5.0,  6.0,   7.0,    8.0, 
            9.0,  10.0,  11.0,   12.0, 
            13.0, 14.0,  15.0,   16.0);

        // Act
        let m00 = matrix_a[[0, 0]];
        let m10 = matrix_a[[1, 0]];
        let m20 = matrix_a[[2, 0]];
        let m30 = matrix_a[[3, 0]];

        let m01 = matrix_a[[0, 1]];
        let m11 = matrix_a[[1, 1]];
        let m21 = matrix_a[[2, 1]];
        let m31 = matrix_a[[3, 1]];

        let m02 = matrix_a[[0, 2]];
        let m12 = matrix_a[[1, 2]];
        let m22 = matrix_a[[2, 2]];
        let m32 = matrix_a[[3, 2]];

        let m03 = matrix_a[[0, 3]];
        let m13 = matrix_a[[1, 3]];
        let m23 = matrix_a[[2, 3]];
        let m33 = matrix_a[[3, 3]];

        // Assert
        assert_eq!(m00, 1.0);
        assert_eq!(m10, 5.0);
        assert_eq!(m20, 9.0);
        assert_eq!(m30, 13.0);

        assert_eq!(m01, 2.0);
        assert_eq!(m11, 6.0);
        assert_eq!(m21, 10.0);
        assert_eq!(m31, 14.0);

        assert_eq!(m02, 3.0);
        assert_eq!(m12, 7.0);
        assert_eq!(m22, 11.0);
        assert_eq!(m32, 15.0);

        assert_eq!(m03, 4.0);
        assert_eq!(m13, 8.0);
        assert_eq!(m23, 12.0);
        assert_eq!(m33, 16.0);
    }
}