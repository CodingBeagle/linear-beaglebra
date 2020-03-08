use crate::vector2::Vector2;
use std::ops::{Add, Sub};

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x,
            y,
            z
        }
    }

    pub fn from_vector2(vector2: Vector2) -> Vector3 {
        Vector3 {
            x: vector2.x,
            y: vector2.y,
            z: 0.0
        }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn scalar_multiplication(self, scalar: f32) -> Self {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }

    pub fn dot_product(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Vector3) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_from_vector2() {
        // Arrange
        let vector_a = Vector2::new(1.0, 2.0);

        // Act
        let constructed_vector3 = Vector3::from_vector2(vector_a);

        // Assert
        assert_eq!(constructed_vector3.x, 1.0);
        assert_eq!(constructed_vector3.y, 2.0);
        assert_eq!(constructed_vector3.z, 0.0);
    }

    #[test]
    fn test_length() {
        // Arrange
        let vector_a = Vector3::new(1.0, 2.0, 3.0);

        // Act
        let vector_length = vector_a.length();

        // Assert
        assert!( approx_eq!(f32, vector_length, 3.74, epsilon = 0.01) );
    }

    #[test]
    fn test_dot_product() {
        // Arrange
        let vector_a = Vector3::new(1.0, 2.0, 1.0);
        let vector_b = Vector3::new(2.0, 1.0, 3.0);

        // Act
        let dot_product = vector_a.dot_product(vector_b);

        // Assert
        assert_eq!(dot_product, 7.0);
    }

    #[test]
    fn test_scalar_multiplication() {
        // Arrange
        let vector_a = Vector3::new(2.0, 3.0, 4.0);
        let scalar: f32 = 2.0;

        // Act
        let scalar_multiplication_result = vector_a.scalar_multiplication(scalar);

        // Assert
        assert_eq!(scalar_multiplication_result.x, 4.0);
        assert_eq!(scalar_multiplication_result.y, 6.0);
        assert_eq!(scalar_multiplication_result.z, 8.0);
    }

    #[test]
    fn test_add() {
        // Arrange
        let vector_a = Vector3::new(1.0, 2.0, 3.0);
        let vector_b = Vector3::new(2.0, 3.0, 4.0);        

        // Act
        let addition_result = vector_a + vector_b;

        // Assert
        assert_eq!(addition_result.x, 3.0);
        assert_eq!(addition_result.y, 5.0);
        assert_eq!(addition_result.z, 7.0);
    }

    #[test]
    fn test_subtraction() {
        // Arrange
        let vector_a = Vector3::new(2.0, 3.0, 4.0);
        let vector_b = Vector3::new(2.0, 1.0, 3.0);

        // Act
        let subtraction_result = vector_a - vector_b;

        // Assert
        assert_eq!(subtraction_result.x, 0.0);
        assert_eq!(subtraction_result.y, 2.0);
        assert_eq!(subtraction_result.z, 1.0);
    }
}