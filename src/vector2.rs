// ** Things to look up **
// TODO: Unit Test Naming Conventions
// TODO: Parameterized tests and when to use them
// TODO: Can you do parameterized tests in Rust???
// TODO: Write proper documentation for the library with Rust documentation Markdown

// std::ops contains overloadable operators.
// In Rust, operator overloading happens by implementing Traits that back up the methods for different operators.
// For example, the "+" operator is backed by the .Add method of the Add trait.
// Notice that this also means that operator overloading can only happen on operators backed by traits.
// It's also not possible to create new operators.
use std::ops::{Add, Sub, Mul};

use crate::sqrt_trait::Sqrt;

// By default, variable bindings have "move semantics".
// For our Vectors, this means that if we assign one to a variable "a" and then afterwards assign
// It to a variable "b", it will have "moved out of a and into b", meaning variable a can no longer be used.
// I would like Vectors to have "copy semantics", meaning that the values of the vector can simply be copied at bit level.
// NOTICE: Using the derive strategy for making a type copyable means that a trait bound is put on "T" for Copy and Clone.
#[derive(Copy, Clone)]
pub struct Vector2<T> where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy {
    pub x: T,
    pub y: T
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2::<T> {
            x,
            y
        }
    }

    pub fn scalar_multiplication(self, scalar: T) -> Vector2<T> {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

    pub fn dot_product(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Sqrt + Copy> Vector2<T> {
    pub fn length(self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

// The convention in Rust for Unit Tests is that they reside in a module named "tests" within the same file for the code that they test
#[cfg(test)]
mod tests {
    // The super::* idiom imports all names from the outer scope.
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_construction() {
        // Act
        let vector_a = Vector2::new(2.0, 5.0);

        // Assert
        assert_eq!(vector_a.x, 2.0);
        assert_eq!(vector_a.y, 5.0);
    }

    #[test]
    fn test_length() {
        // Arrange
        let vector_a = Vector2::new(3.0, 3.0);

        // Act
        let length_result = vector_a.length();

        // Assert
        // TODO: Learn more about float comparison: https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
        // Attempting to compare floats is apparently (mostly, it seems anyways) always taught to be a bad idea.
        // The idea being that float math is difficult, and the behaviour of floating point math makes it difficult to write consistent tests
        // For SOME ranges, an okay idea is to use what is called an "epsilon". Instead of comparing the equality of two floats, you inspect
        // the difference between them, and accept them as being equal if that different is within some range or percentage.
        // I use the epsilon approach for this test
        assert!( approx_eq!(f32, length_result, 4.24, epsilon = 0.01) );
    }

    #[test]
    fn test_dot_product() {
        // Arrange
        let vector_a = Vector2::new(3.0, 7.0);
        let vector_b = Vector2::new(4.0, 9.0);

        // Act
        let dot_product_result = vector_a.dot_product(vector_b);

        // Assert
        assert_eq!(dot_product_result, 75.0);
    }

    #[test]
    fn test_scalar_multiplication() {
        // Arrange
        let vector_a = Vector2::new(1.0, 2.0);

        // Act
        let scalar_multiplication_result = vector_a.scalar_multiplication(5.0);

        // Assert
        assert_eq!(scalar_multiplication_result.x, 5.0);
        assert_eq!(scalar_multiplication_result.y, 10.0);
    }

    #[test]
    fn test_add() {
        // Arrange
        let vector_a = Vector2::new(1.0, 2.0);
        let vector_b = Vector2::new(1.0, 1.0);

        // Act
        let addition_result = vector_a + vector_b;

        // Assert
        assert_eq!(addition_result.x, 2.0);
        assert_eq!(addition_result.y, 3.0);
    }

    #[test]
    fn test_sub() {
        // Arrange
        let vector_a = Vector2::new(2.0, 3.0);
        let vector_b = Vector2::new(1.0, 3.0);

        // Act
        let subtraction_result = vector_a - vector_b;

        // Assert
        assert_eq!(subtraction_result.x, 1.0);
        assert_eq!(subtraction_result.y, 0.0);
    }
}