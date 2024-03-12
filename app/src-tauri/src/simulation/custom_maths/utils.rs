// Retourne le signe d'un nombre de type T
pub fn sign_f32(x: f32) -> f32 {
    match x {
        x if x > 0.0 => 1.0,
        x if x < 0.0 => -1.0,
        _ => 0.0,
    }
}