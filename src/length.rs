#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Length {
    mm: f64,
}
impl Length {
    pub fn from_mm(value: f64) -> Self {
        Length { mm: value }
    }
    pub fn mm(self) -> f64 {
        self.mm
    }
    pub fn from_m(value: f64) -> Self {
        Length { mm: value * 1000. }
    }
    pub fn m(self) -> f64 {
        self.mm / 1000.
    }
}
