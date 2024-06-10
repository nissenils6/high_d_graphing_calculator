#[derive(Clone, Copy)]
struct Complex {
    // z = a + bi
    a: f64,
    b: f64,
}

impl Complex {
    fn new(a: f64, b: f64) -> Complex {
        Complex { a, b }
    }

    fn from_polar(r: f64, theta: f64) -> Complex {
        Complex { a: r * theta.cos(), b: r * theta.sin() }
    }

    fn abs(self) -> f64 {
        ((self.a * self.a) + (self.b * self.b)).sqrt()
    }

    fn arg(self) -> f64 {
        (self.b.atan2(self.a) + std::f64::consts::TAU) % (std::f64::consts::TAU)
    }

    fn con(self) -> Complex {
        Complex { a: self.a, b: -self.b }
    }

    fn pow(self, v: f64) -> Complex {
        let ang: f64 = self.arg() * v;
        let rad: f64 = self.abs().powf(v);
        Complex::from_polar(rad, ang)
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex { a: self.a + rhs.a, b: self.b + rhs.b }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Complex { a: self.a - rhs.a, b: self.b - rhs.b }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex {
            a: (self.a * rhs.a) - (self.b * rhs.b),
            b: (self.a * rhs.b) + (self.b * rhs.a),
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let den: f64 = (rhs.a * rhs.a) + (rhs.b * rhs.b);
        Complex {
            a: ((self.a * rhs.a) + (self.b * rhs.b)) / (den),
            b: ((self.b * rhs.a) + (self.a * rhs.b)) / (den),
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.a, self.b)
    }
}
