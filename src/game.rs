use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::borrow::BorrowMut;

pub trait Particle {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
    fn set_x(&mut self, x: i32);
    fn set_y(&mut self, y: i32);
    fn get_color(&self) -> Color;
    fn does_exists(&self) -> bool;
    fn destroy(&mut self);
    fn update(&mut self, width: i32, height: i32, positions: &mut HashMap<(i32, i32), bool>);
}

pub struct World {
    pub particles: Vec<Box<dyn Particle>>,
    pub width: i32,
    pub height: i32,
    pub positions: HashMap<(i32, i32), bool>
}

impl World {
    pub fn new(width: i32, height: i32) -> Self {

        let mut positions = HashMap::new();

        for x in 0..=width {
            for y in 0..=height {
                positions.insert((x, y), false);
            }
        }

        Self {
            particles: Vec::new(),
            width,
            height,
            positions
        }
    }

    pub fn get_particle_at(&self, x: i32, y: i32) -> Option<&dyn Particle> {
        for p in self.particles.iter() {
            if p.get_x() == x && p.get_y() == y {
                return Some(&***Box::new(p));
            }
        }
        None
    }


    pub fn update(&mut self) {

        let mut positions_copy = self.positions.clone();

        self.particles.retain(|p| {
           if !p.does_exists() {
               positions_copy.insert((p.get_x(), p.get_y()), false);
               return false;
           }
            true
        });
        self.positions = positions_copy;

        let mut positions_copy = self.positions.clone();
        for p in self.particles.iter_mut().rev() {
            p.update(self.width.clone(), self.height.clone(), &mut positions_copy);
        }
        self.positions = positions_copy;

    }

    pub fn clean(&mut self) {
        let mut positions = HashMap::new();

        for x in 0..=self.width {
            for y in 0..=self.height {
                positions.insert((x, y), false);
            }
        }

        self.positions = positions;

        self.particles = Vec::new();

    }

    pub fn spawn_particle(&mut self, particle: Box<dyn Particle>) {
        self.particles.push(particle);
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        for p in self.particles.iter() {
            canvas.set_draw_color(p.get_color());
            canvas.fill_rect(Rect::new(p.get_x(), p.get_y(), 2, 2));
        }
    }

    pub fn destroy_particle(&mut self, x: i32, y: i32) {
        for p in self.particles.iter_mut() {
            if p.get_x() == x && p.get_y() == y {
                p.destroy();
            }
        }
    }

}

pub struct Sand {
    pub x: i32,
    pub y: i32,
    pub does_exists: bool
}

impl Particle for Sand {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    fn get_color(&self) -> Color {
        Color::RGB(235, 189, 52)
    }

    fn does_exists(&self) -> bool {
        self.does_exists
    }

    fn destroy(&mut self) {
        self.does_exists = false;
    }

    fn update(&mut self, width: i32, height: i32, positions: &mut HashMap<(i32, i32), bool>) {
        if self.get_y() + 2 < height && *positions.get(&(self.get_x(), self.get_y() + 2)).unwrap() == false {
            positions.insert((self.get_x(), self.get_y()), false);
            self.set_y(self.get_y() + 2);
            positions.insert((self.get_x(), self.get_y()), true);
        } else if self.get_y() + 2 < height && self.get_x() + 2 < width && *positions.get(&(self.get_x() + 2, self.get_y() + 2)).unwrap() == false {
            positions.insert((self.get_x(), self.get_y()), false);
            self.set_y(self.get_y() + 2);
            self.set_x(self.get_x() + 2);
            positions.insert((self.get_x(), self.get_y()), true);
        }  else if self.get_y() + 2 < height && self.get_x() - 2 >= 0 && *positions.get(&(self.get_x() - 2, self.get_y() + 2)).unwrap() == false {
            positions.insert((self.get_x(), self.get_y()), false);
            self.set_y(self.get_y() + 2);
            self.set_x(self.get_x() - 2);
            positions.insert((self.get_x(), self.get_y()), true);
        }
    }
}

pub struct Wood {
    pub x: i32,
    pub y: i32,
    pub does_exists: bool
}

impl Particle for Wood {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    fn get_color(&self) -> Color {
        Color::RGB(38, 3, 3)
    }

    fn does_exists(&self) -> bool {
        self.does_exists
    }

    fn destroy(&mut self) {
        self.does_exists = false;
    }

    fn update(&mut self, width: i32, height: i32, positions: &mut HashMap<(i32, i32), bool>) {
        positions.insert((self.get_x(), self.get_y()), true);
    }
}

pub enum ParticleType {
    WOOD,
    SAND,
    WATER
}

pub struct Water {
    pub x: i32,
    pub y: i32,
    pub does_exists: bool
}

impl Particle for Water {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    fn get_color(&self) -> Color {
        Color::RGB(66, 135, 245)
    }

    fn does_exists(&self) -> bool {
        self.does_exists
    }

    fn destroy(&mut self) {
        self.does_exists = false;
    }

    fn update(&mut self, width: i32, height: i32, positions: &mut HashMap<(i32, i32), bool>) {
        if self.get_y() + 2 < height && *positions.get(&(self.get_x(), self.get_y() + 2)).unwrap() == false {
            positions.insert((self.get_x(), self.get_y()), false);
            self.set_y(self.get_y() + 2);
            positions.insert((self.get_x(), self.get_y()), true);
        } else if self.get_y() + 2 < height && self.get_x() + 2 < width && *positions.get(&(self.get_x() + 2, self.get_y() + 2)).unwrap() == false {
            positions.insert((self.get_x(), self.get_y()), false);
            self.set_y(self.get_y() + 2);
            self.set_x(self.get_x() + 2);
            positions.insert((self.get_x(), self.get_y()), true);
        }  else if self.get_y() + 2 < height && self.get_x() - 2 >= 0 && *positions.get(&(self.get_x() - 2, self.get_y() + 2)).unwrap() == false {
            positions.insert((self.get_x(), self.get_y()), false);
            self.set_y(self.get_y() + 2);
            self.set_x(self.get_x() - 2);
            positions.insert((self.get_x(), self.get_y()), true);
        } else if self.get_x() + 2 < width &&  *positions.get(&(self.get_x() + 2, self.get_y())).unwrap() == false {
            positions.insert((self.get_x(), self.get_y()), false);
            self.set_x(self.get_x() + 2);
            positions.insert((self.get_x(), self.get_y()), true);
        } else if self.get_x() - 2 >= 0 &&  *positions.get(&(self.get_x() - 2, self.get_y())).unwrap() == false {
            positions.insert((self.get_x(), self.get_y()), false);
            self.set_x(self.get_x() - 2);
            positions.insert((self.get_x(), self.get_y()), true);
        }
    }
}