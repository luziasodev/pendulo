struct Pendulo {
    origem: vector::Vector,
    posicao: vector::Vector,
    angulo: f32,
    velocidade_angular: f32,
    aceleracao_angular: f32,
    l: f32, 
    m: f32, 
    g: f32, 
}

impl Pendulo {
    fn new(x: f32, y: f32, r: f32) -> Pendulo {
        Pendulo {
            origem: vector::Vector::new(x, y),
            posicao: vector::Vector::new(0.0, 0.0),
            angulo: 1.0,             
            velocidade_angular: 0.0,     
            aceleracao_angular: 0.0, 
            l: l,
            m: 1.0, 
            g: 10.0, 
        }
    }

    fn update(&mut self) {
        self.aceleracao_angular = -1.0 * self.g * self.angle.sin() / self.l;

        self.velocidade_angular += self.aceleracao_angular;

        self.angulo += self.velocidade_angular;

        self.posicao
            .set(self.l * self.angulo.sin(), self.l * self.angulo.cos());

        self.position.add(&self.origem);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origem.x, self.origem.y),
            (self.posicao.x, self.posicao.y),
            3.0,
            Color::PURPLE,
        );

        graphics.draw_circle((self.posicao.x, self.posicao.y), 30.0, Color::PURPLE);
    }
}

pub mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y } 
        }

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }
    }
}
