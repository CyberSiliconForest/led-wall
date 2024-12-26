use embedded_canvas::Canvas;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Point, Size};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::primitives::Rectangle;
use rand::Rng;

pub struct Conway {
    width: i32,
    height: i32,
    pub canvas: Canvas<Rgb888>,
}

impl Conway {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            canvas: Canvas::new(Size {
                width: width as u32,
                height: height as u32,
            }),
        }
    }

    pub fn randomize_canvas(&mut self, probability: f64) {
        let mut rng = rand::thread_rng();
        for x in 0..64 * 3 {
            for y in 0..64 * 2 {
                let pixel = if rng.gen::<f64>() < probability {
                    Rgb888::new(0xff, 0xff, 0xff)
                } else {
                    Rgb888::new(0, 0, 0)
                };

                self.canvas
                    .fill_solid(
                        &Rectangle {
                            top_left: Point { x, y },
                            size: Size {
                                width: 1,
                                height: 1,
                            },
                        },
                        pixel,
                    )
                    .unwrap();
            }
        }
    }

    pub fn calculate_next_frame(&mut self) {
        let canvas_at = self.canvas.place_at(Point { x: 0, y: 0 });
        let mut canvas_next = Canvas::new(self.canvas.size());
        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = [
                    canvas_at.get_pixel(Point { x: x - 1, y: y - 1 }),
                    canvas_at.get_pixel(Point { x: x - 1, y: y + 0 }),
                    canvas_at.get_pixel(Point { x: x - 1, y: y + 1 }),
                    canvas_at.get_pixel(Point { x: x + 0, y: y - 1 }),
                    canvas_at.get_pixel(Point { x: x + 0, y: y + 1 }),
                    canvas_at.get_pixel(Point { x: x + 1, y: y - 1 }),
                    canvas_at.get_pixel(Point { x: x + 1, y: y + 0 }),
                    canvas_at.get_pixel(Point { x: x + 1, y: y + 1 }),
                ];

                let mut new_pixel = canvas_at.get_pixel(Point { x, y }).unwrap();

                let mut neighbor_cnt = 0;
                for value in neighbors {
                    if let Some(pixel) = value {
                        if pixel != Rgb888::new(0, 0, 0) {
                            neighbor_cnt += 1;
                        }
                    }
                }

                if neighbor_cnt < 2 {
                    new_pixel = Rgb888::new(0, 0, 0);
                } else if neighbor_cnt == 2 {
                    // Do nothing
                } else if neighbor_cnt == 3 {
                    new_pixel = Rgb888::new(0xff, 0xff, 0xff);
                } else {
                    new_pixel = Rgb888::new(0, 0, 0);
                }

                canvas_next
                    .fill_solid(
                        &Rectangle {
                            top_left: Point { x, y },
                            size: Size {
                                width: 1,
                                height: 1,
                            },
                        },
                        new_pixel,
                    )
                    .unwrap();
            }
        }

        self.canvas = canvas_next;
    }
}
