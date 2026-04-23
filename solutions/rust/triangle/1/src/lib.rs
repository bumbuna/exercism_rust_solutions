pub struct Triangle(u64,u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let a = sides[0];
        let b = sides[1];
        let c = sides[2];
        if a+b >= c && a+c >= b && b+c >= a && a > 0 && b > 0 && c > 0 {
            Some(Triangle(a,b,c))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let Self(a, b, c) = self;
        a==b && a==c
    }

    pub fn is_scalene(&self) -> bool {
        let Self(a, b, c) = self;
        a != b && a != c && b != c
    }

    pub fn is_isosceles(&self) -> bool {
        let Self(a, b, c) = self;
        a == b || a == c || b == c
    }
}
