use embedded_canvas::{Canvas, CanvasAt};
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::Rgb888;

use crate::util::Dir;

pub struct ImageSlide {
    viewport_size: Size,
    image_size: Size,
    cursor: Point,
    direction: Dir,
    tick_skip_at_center: i32,
    tick_cnt: i32,
    pub canvas: Canvas<Rgb888>,
}

impl ImageSlide {
    pub fn new(
        viewport_size: Size,
        image_size: Size,
        direction: Dir,
        tick_skip_at_center: i32,
    ) -> Self {
        let mut cursor = Point::new(0, 0);

        if direction.x > 0 {
            cursor.x -= image_size.width as i32;
        } else if direction.x < 0 {
            cursor.x = image_size.width as i32;
        }

        if direction.y > 0 {
            cursor.y -= image_size.height as i32;
        } else if direction.y < 0 {
            cursor.y = image_size.height as i32;
        }

        Self {
            viewport_size,
            image_size,
            cursor,
            direction,
            tick_skip_at_center,
            tick_cnt: 0,
            canvas: Canvas::new(image_size),
        }
    }

    pub fn step(&mut self) {
        let x_center_off = (self.viewport_size.width as i32 - self.image_size.width as i32) / 2;
        let y_center_off = (self.viewport_size.height as i32 - self.image_size.height as i32) / 2;
        tracing::debug!("xc: {}, yc: {}", x_center_off, y_center_off);
        // TODO: This logic only works on specific case.
        if self.cursor.x != x_center_off || self.cursor.y != y_center_off {
            tracing::debug!("x: {}, y: {}", self.cursor.x, self.cursor.y);
            self.tick_cnt = self.tick_skip_at_center;
            self.cursor.x += self.direction.x;
            self.cursor.y += self.direction.y;
        } else if self.tick_cnt > 0 {
            self.tick_cnt -= 1;
        } else {
            self.cursor.x += self.direction.x;
            self.cursor.y += self.direction.y;
        }
    }

    pub fn get_canvas_at(&self) -> CanvasAt<Rgb888> {
        self.canvas
            .place_at(Point::new(self.cursor.x, self.cursor.y))
    }
}
