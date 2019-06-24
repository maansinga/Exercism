pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let [a, b, c] = sides;
        if a+b>c && b+c>a && a+c>b {
            Some(Triangle(a,b,c))
        }else{
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let Triangle(a, b, c) = self;
        a==b && b==c
    }

    pub fn is_scalene(&self) -> bool {
        let Triangle(a, b, c) = self;
        a != b && a !=c && b != c
    }

    pub fn is_isosceles(&self) -> bool {
        
        !self.is_equilateral() && !self.is_scalene()
    }
}
