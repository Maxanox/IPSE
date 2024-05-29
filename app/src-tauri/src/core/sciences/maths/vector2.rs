use serde::{Serialize, Deserialize};
use std::ops::{Add, Mul, Div, Sub, AddAssign, SubAssign, MulAssign, DivAssign};

use rand::Rng;

/// Represente un vecteur 2D
#[derive(Copy, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

#[allow(dead_code)]
impl Vector2 {
  /// Constructeur de base
  pub fn new(x: f32, y: f32) -> Vector2 {
    Vector2 { x, y }
  }

  /// Constructeur de vecteur nul
  pub fn zero() -> Vector2 {
    Vector2 { x: 0.0, y: 0.0 }
  }

  /// Constructeur du vecteur unitaire
  pub fn one() -> Vector2 {
    Vector2 { x: 1.0, y: 1.0 }
  }

  /// Constructeur du vecteur droite
  pub fn right() -> Vector2 {
    Vector2 { x: 1.0, y: 0.0 }
  }

  /// Constructeur du vecteur gauche
  pub fn left() -> Vector2 {
    Vector2 { x: -1.0, y: 0.0 }
  }

  /// Constructeur du vecteur haut
  /// 
  /// # IMPORTANT
  /// le vecteur haut et bas, sont inversés pour correspondre
  /// à la convention de coordonnées du canvas utilisé par PIXI.js
  pub fn up() -> Vector2 {
    Vector2 { x: 0.0, y: -1.0 }
  }

  /// Constructeur du vecteur bas
  /// 
  /// # IMPORTANT
  /// le vecteur haut et bas, sont inversés pour correspondre
  /// à la convention de coordonnées du canvas utilisé par PIXI.js
  pub fn down() -> Vector2 {
    Vector2 { x: 0.0, y: 1.0 }
  }

  /// Constructeur du vecteur normalisé aléatoire
  /// * Généralement utilisé pour générer des vecteurs de direction aléatoire
  pub fn random() -> Vector2 {
    let mut rng = rand::thread_rng();

    let mut x = 0.0;
    let mut y = 0.0;

    // On s'assure que le vecteur n'est pas nul
    // Pour éviter une erreurs de normalisation
    // (division par zéro)
    while (x, y) == (0.0, 0.0) {
      x = rng.gen_range(-1.0..=1.0);
      y = rng.gen_range(-1.0..=1.0);
    }
    
    let vec = Vector2 { x, y };

    vec.normalize().unwrap()
  }

  /// Constructeur du vecteur normalisé aléatoire avec une rng donnée
  /// * Généralement utilisé pour générer des vecteurs de direction aléatoire
  /// 
  /// # Arguments
  /// * `rng` - générateur de nombre aléatoire
  pub fn random_with_rng(rng: &mut rand::rngs::ThreadRng) -> Vector2 {
    let mut x = 0.0;
    let mut y = 0.0;

    // On s'assure que le vecteur n'est pas nul
    // Pour éviter une erreurs de normalisation
    // (division par zéro)
    while (x, y) == (0.0, 0.0) {
      x = rng.gen_range(-1.0..=1.0);
      y = rng.gen_range(-1.0..=1.0);
    }
    
    let vec = Vector2 { x, y };

    vec.normalize().unwrap()
  }

  /// Retourne la distance entre deux vecteurs
  pub fn distance(v1: Vector2, v2: Vector2) -> f32 {
    (v1 - v2).magnitude()
  }

  /// Retourne la distance entre le vecteur et un autre vecteur
  pub fn distance_to(&self, other: Vector2) -> f32 {
    (*self - other).magnitude()
  }

  /// Retourne le vecteur opposé
  pub fn opposite(&self) -> Vector2 {
    Vector2 {
      x: -self.x,
      y: -self.y,
    }
  }

  /// Retourne la norme du vecteur
  pub fn magnitude(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  /// Retourne le vecteur normalisé.
  /// * Généralement utilisé pour récupéré le vecteur directeur
  pub fn normalize(&self) -> Result<Vector2, String> {
    let magnitude = self.magnitude();
    if magnitude == 0.0 {
      Err("La distance entre les deux points est nulle".to_string())
    } else {
      Ok(
        Vector2 {
          x: self.x / magnitude,
          y: self.y / magnitude,
        }
      )
    }
  }

  /// Retourne le vecteur directeur entre deux coordonnées
  pub fn direction_to(&self, other: Vector2) -> Result<Vector2, String> {
    (*self - other).normalize()
  }
}

/// Implementation de l'addition
impl Add<Vector2> for Vector2 {
  type Output = Vector2;

  fn add(self, rhs: Vector2) -> Self::Output {
    Vector2 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

/// Implementation de la soustraction
impl Sub<Vector2> for Vector2 {
  type Output = Vector2;

  fn sub(self, rhs: Vector2) -> Self::Output {
    Vector2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

/// Implementation de l'addition avec assignation
impl AddAssign<Vector2> for Vector2 {
  fn add_assign(&mut self, rhs: Vector2) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

/// Implementation de la soustraction avec assignation
impl SubAssign<Vector2> for Vector2 {
  fn sub_assign(&mut self, rhs: Vector2) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

/// Implementation du produit scalaire
impl Mul<Vector2> for Vector2 {
  type Output = f32;

  fn mul(self, rhs: Vector2) -> Self::Output {
    self.x * rhs.x + self.y * rhs.y
  }
}

/// Implementation de la multiplication par un coefficient
impl Mul<f32> for Vector2 {
  type Output = Vector2;

  fn mul(self, rhs: f32) -> Self::Output {
    Vector2 {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

/// Implementation de la multiplication par un coefficient avec assignation
impl MulAssign<f32> for Vector2 {
  fn mul_assign(&mut self, rhs: f32) -> () {
    self.x *= rhs;
    self.y *= rhs;
  }
}

/// Implementation de la division par un coefficient
impl Div<f32> for Vector2 {
  type Output = Vector2;

  fn div(self, rhs: f32) -> Self::Output {
    Vector2 {
      x: self.x / rhs,
      y: self.y / rhs,
    }
  }
}

/// Implementation de la division par un coefficient avec assignation
impl DivAssign<f32> for Vector2 {
  fn div_assign(&mut self, rhs: f32) -> () {
    self.x /= rhs;
    self.y /= rhs;
  }
}