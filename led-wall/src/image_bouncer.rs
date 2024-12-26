use embedded_canvas::{Canvas, CanvasAt};
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::Rgb888;
use rand::Rng;

struct Dir {
    pub x: i32,
    pub y: i32,
}

pub struct ImageBouncer {
    viewport_size: Size,
    image_size: Size,
    direction: Dir,
    cursor: Point,
    pub canvas: Canvas<Rgb888>,
}

impl ImageBouncer {
    pub fn new(viewport_size: Size, image_size: Size) -> Self {
        Self {
            viewport_size,
            image_size,
            direction: Dir { x: 1, y: 1 },
            cursor: Point::new(0, 0),
            canvas: Canvas::new(image_size),
        }
    }

    pub fn randomize_start(&mut self) {
        let max_x = (self.image_size.width - self.viewport_size.width) as i32 - 1;
        let max_y = (self.image_size.height - self.viewport_size.height) as i32 - 1;

        let mut rng = rand::thread_rng();

        let rand_x = rng.gen::<i32>() % max_x;
        let rand_y = rng.gen::<i32>() % max_y;

        self.cursor = Point::new(rand_x, rand_y);
    }

    pub fn bounce(&mut self) {
        let max_x = (self.image_size.width - self.viewport_size.width) as i32 - 1;
        let max_y = (self.image_size.height - self.viewport_size.height) as i32 - 1;

        if self.cursor.x + self.direction.x > max_x {
            self.direction.x = -1;
        }

        if self.cursor.x + self.direction.x < 0 {
            self.direction.x = 1;
        }

        if self.cursor.y + self.direction.y > max_y {
            self.direction.y = -1;
        }

        if self.cursor.y + self.direction.y < 0 {
            self.direction.y = 1;
        }

        tracing::trace!("Cur pos: ({}, {})", self.cursor.x, self.cursor.y);

        self.cursor.x += self.direction.x;
        self.cursor.y += self.direction.y;

        tracing::trace!("New pos: ({}, {})", self.cursor.x, self.cursor.y);
    }

    pub fn get_canvas_at(&mut self) -> CanvasAt<Rgb888> {
        self.canvas
            .place_at(Point::new(-self.cursor.x, -self.cursor.y))
    }
}
