#[#[derive(Copy, Clone, Default)]]

// Ray tracers can use either f32 of f64 - its all preference.
// Derive attribute asks the compiler to implement the Copy trait for Vec3.
// It changes the variable binding from move semantics to copy semantics.
//
// Helpful for reusing and useassing Vec3 variables without borrowing,
// making operations easier.
//
//
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }
}

// Type alias
pub type Point3 = Vec3;
