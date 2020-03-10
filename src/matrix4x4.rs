use std::fmt;
use std::default;

#[derive(Debug)]
pub enum MatrixError {
    InvalidInput(String)
}

pub struct Matrix4x4 {
    // The matrix is represented as a one-dimensional arrray in column-major order
    // So far I haven't found a good reason to use a multi-dimensional array in code
    array: [f32; 16]
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Matrix4x4 {
            array: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        }
    }
}

impl Matrix4x4 {
    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32) -> Matrix4x4 {
            Matrix4x4 {
                array: [ m00, m10, m20, m30, m01, m11, m21, m31, m02, m12, m22, m32, m03, m13, m23, m33 ]
            }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn getEntry(&self, row: usize, column: usize) -> Result<f32, MatrixError> {
        // REASONING: Error-Handling of expected possible bad input.
        // I return an error message that the client needs to handle.
        // The entire application using this library certainly shouldn't shut down simply
        // Because of bad input to this function. The client should have a chance to do its own thing.
        if row > 3 {
            return Err(MatrixError::InvalidInput(String::from("Max possible row is 3.")));
        }

        if column > 3 {
            return Err(MatrixError::InvalidInput(String::from("Max possible row is 3.")));
        }

        // LEARN: In order to index arrays in Rust, you need usize. Find out why.
        let array_index: usize = (column * 4) + row;
        Ok(self.array[array_index])
    }
}

impl fmt::Debug for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = "Failed to read matrix entry on pretty print.";

        // TODO: Really wanna make this shorter and neater...
        let m00 = self.getEntry(0, 0).expect(error_message);
        let m01 = self.getEntry(0, 1).expect(error_message);
        let m02 = self.getEntry(0, 2).expect(error_message);
        let m03 = self.getEntry(0, 3).expect(error_message);

        let m10 = self.getEntry(1, 0).expect(error_message);
        let m11 = self.getEntry(1, 1).expect(error_message);
        let m12 = self.getEntry(1, 2).expect(error_message);
        let m13 = self.getEntry(1, 3).expect(error_message);

        let m20 = self.getEntry(2, 0).expect(error_message);
        let m21 = self.getEntry(2, 1).expect(error_message);
        let m22 = self.getEntry(2, 2).expect(error_message);
        let m23 = self.getEntry(2, 3).expect(error_message);

        let m30 = self.getEntry(3, 0).expect(error_message);
        let m31 = self.getEntry(3, 1).expect(error_message);
        let m32 = self.getEntry(3, 2).expect(error_message);
        let m33 = self.getEntry(3, 3).expect(error_message);

        // TODO: Can you call multiple write! macros per line instead of having all in one call??
        write!(f, "{},{},{},{}\n{},{},{},{}\n{},{},{},{}\n{},{},{},{}",
                m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23,
                m30, m31, m32, m33,)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identy_matrix() {
        // Arrange & Act
        let identity_matrix = Matrix4x4::identity();

        // Assert
        assert_eq!(identity_matrix.getEntry(0,0).expect("Failed to get entry in matrix."), 1.0);
        assert_eq!(identity_matrix.getEntry(1,1).expect("Failed to get entry in matrix."), 1.0);
        assert_eq!(identity_matrix.getEntry(2,2).expect("Failed to get entry in matrix."), 1.0);
        assert_eq!(identity_matrix.getEntry(3,3).expect("Failed to get entry in matrix."), 1.0);

        assert_eq!(identity_matrix.getEntry(0,1).expect("Failed to get entry in matrix."), 0.0);
        assert_eq!(identity_matrix.getEntry(0,2).expect("Failed to get entry in matrix."), 0.0);
        assert_eq!(identity_matrix.getEntry(0,3).expect("Failed to get entry in matrix."), 0.0);

        assert_eq!(identity_matrix.getEntry(1,0).expect("Failed to get entry in matrix."), 0.0);
        assert_eq!(identity_matrix.getEntry(1,2).expect("Failed to get entry in matrix."), 0.0);
        assert_eq!(identity_matrix.getEntry(1,3).expect("Failed to get entry in matrix."), 0.0);

        assert_eq!(identity_matrix.getEntry(2,0).expect("Failed to get entry in matrix."), 0.0);
        assert_eq!(identity_matrix.getEntry(2,1).expect("Failed to get entry in matrix."), 0.0);
        assert_eq!(identity_matrix.getEntry(2,3).expect("Failed to get entry in matrix."), 0.0);

        assert_eq!(identity_matrix.getEntry(3,0).expect("Failed to get entry in matrix."), 0.0);
        assert_eq!(identity_matrix.getEntry(3,1).expect("Failed to get entry in matrix."), 0.0);
        assert_eq!(identity_matrix.getEntry(3,2).expect("Failed to get entry in matrix."), 0.0);
    }
}