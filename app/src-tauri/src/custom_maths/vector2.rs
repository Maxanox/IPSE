use serde::{Serialize, Deserialize};
use std::ops::{Add, Mul, Sub, AddAssign, SubAssign, MulAssign};

#[derive(Copy, Serialize, Deserialize, Clone)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

#[allow(dead_code)]
impl Vector2 {
  // Constructeur de base
  pub fn new(x: f32, y: f32) -> Vector2 {
    Vector2 { x, y }
  }

  // Constructeur de vecteur nul
  pub fn zero() -> Vector2 {
    Vector2 { x: 0.0, y: 0.0 }
  }

  // Constructeur du vecteur droite
  pub fn right() -> Vector2 {
    Vector2 { x: 1.0, y: 0.0 }
  }

  // Constructeur du vecteur gauche
  pub fn left() -> Vector2 {
    Vector2 { x: -1.0, y: 0.0 }
  }

  // vecteur haut et bas, sont inversés pour correspondre
  // à la convention de coordonnées du canvas utilisé par PIXI.js

  // Constructeur du vecteur haut
  pub fn up() -> Vector2 {
    Vector2 { x: 0.0, y: -1.0 }
  }

  // Constructeur du vecteur bas
  pub fn down() -> Vector2 {
    Vector2 { x: 0.0, y: 1.0 }
  }

  // Retourne le vecteur opposé
  fn opposite(&self) -> Vector2 {
    Vector2 {
      x: -self.x,
      y: -self.y,
    }
  }

  // Retourne la norme du vecteur
  fn magnitude(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  // Retourne le vecteur normalisé
  fn normalize(&self) -> Vector2 {
    let magnitude = self.magnitude();
    Vector2 {
      x: self.x / magnitude,
      y: self.y / magnitude,
    }
  }
}

// Implementation de l'addition
impl Add<Vector2> for Vector2 {
  type Output = Vector2;

  fn add(self, rhs: Vector2) -> Self::Output {
    Vector2 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

// Implementation de la soustraction
impl Sub<Vector2> for Vector2 {
  type Output = Vector2;

  fn sub(self, rhs: Vector2) -> Self::Output {
    Vector2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

// Implementation de l'addition avec assignation
impl AddAssign<Vector2> for Vector2 {
  fn add_assign(&mut self, rhs: Vector2) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

// Implementation de la soustraction avec assignation
impl SubAssign<Vector2> for Vector2 {
  fn sub_assign(&mut self, rhs: Vector2) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

// Implementation du produit scalaire
impl Mul<Vector2> for Vector2 {
  type Output = f32;

  fn mul(self, rhs: Vector2) -> Self::Output {
    self.x * rhs.x + self.y * rhs.y
  }
}

// Implementation de la multiplication par un coefficient
impl Mul<f32> for Vector2 {
  type Output = Vector2;

  fn mul(self, rhs: f32) -> Self::Output {
    Vector2 {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

// Implementation de la multiplication par un coefficient avec assignation
impl MulAssign<f32> for Vector2 {
  fn mul_assign(&mut self, rhs: f32) -> () {
    self.x *= rhs;
    self.y *= rhs;
  }
}