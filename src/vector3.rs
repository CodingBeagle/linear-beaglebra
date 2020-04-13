use crate::vector2::Vector2;
use std::ops::{Add, Sub, Mul};

use crate::sqrt_trait::Sqrt;

// By default, variable bindings have "move semantics".
// For our Vectors, this means that if we assign one to a variable "a" and then afterwards assign
// It to a variable "b", it will have "moved out of a and into b", meaning variable a can no longer be used.
// I would like Vectors to have "copy semantics", meaning that the values of the vector can simply be copied at bit level.
// NOTICE: Using the derive strategy for making a type copyable means that a trait bound is put on "T" for Copy and Clone.
#[derive(Copy, Clone)]
pub struct Vector3<T> where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Default> Vector3<T> {
    pub fn from_vector2(vector2: Vector2<T>) -> Vector3<T> {
        Vector3::<T> {
            x: vector2.x,
            y: vector2.y,
            z: T::default()
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 {
            x,
            y,
            z
        }
    }

    pub fn scalar_multiplication(self, scalar: T) -> Self {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }

    pub fn dot_product(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Sqrt> Vector3<T> {
    pub fn length(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, other: Vector3<T>) -> Self {
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
    fn test_construction() {
        // Act
        let vector_a = Vector3::<f64>::new(1.0, 2.0, 3.0);

        // Assert
        assert_eq!(vector_a.x, 1.0);
        assert_eq!(vector_a.y, 2.0);
        assert_eq!(vector_a.z, 3.0);
    }

    #[test]
    fn test_from_vector2() {
        // Arrange
        let vector_a = Vector2::<f64>::new(1.0, 2.0);

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
        let vector_a = Vector3::<f64>::new(1.0, 2.0, 3.0);

        // Act
        let vector_length = vector_a.length();

        // Assert
        assert!( approx_eq!(f64, vector_length, 3.74, epsilon = 0.01) );
    }

    #[test]
    fn test_dot_product() {
        // Arrange
        let vector_a = Vector3::<f64>::new(1.0, 2.0, 1.0);
        let vector_b = Vector3::<f64>::new(2.0, 1.0, 3.0);

        // Act
        let dot_product = vector_a.dot_product(vector_b);

        // Assert
        assert_eq!(dot_product, 7.0);
    }

    #[test]
    fn test_scalar_multiplication() {
        // Arrange
        let vector_a = Vector3::<f64>::new(2.0, 3.0, 4.0);
        let scalar = 2.0;

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
        let vector_a = Vector3::<f64>::new(1.0, 2.0, 3.0);
        let vector_b = Vector3::<f64>::new(2.0, 3.0, 4.0);        

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
        let vector_a = Vector3::<f64>::new(2.0, 3.0, 4.0);
        let vector_b = Vector3::<f64>::new(2.0, 1.0, 3.0);

        // Act
        let subtraction_result = vector_a - vector_b;

        // Assert
        assert_eq!(subtraction_result.x, 0.0);
        assert_eq!(subtraction_result.y, 2.0);
        assert_eq!(subtraction_result.z, 1.0);
    }
}