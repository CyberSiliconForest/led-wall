use crate::conway::Conway;
use crate::image_bouncer::ImageBouncer;
use embedded_canvas::Canvas;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Point, Size};
use embedded_graphics::image::ImageDrawable;
use embedded_graphics::mono_font::ascii::FONT_9X15;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::primitives::Rectangle;
//use embedded_graphics::text::Text;

use crate::image_slide::ImageSlide;
use crate::util::{Dir, MatrixService};
use std::time::Duration;
use tinybmp::Bmp;

mod conway;
mod image_bouncer;
mod image_slide;
mod util;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let mut matrix_service = MatrixService::new();
    let matrix_canvas_size = Size {
        width: 64 * 3,
        height: 64 * 2,
    };

    let mut canvas = Canvas::new(matrix_canvas_size);

    canvas.fill_solid(
        &Rectangle {
            top_left: Point { x: 0, y: 0 },
            size: matrix_canvas_size,
        },
        Rgb888::new(0, 255, 0),
    )?;

    matrix_service.draw_canvas_at(&canvas.place_at(Point { x: 0, y: 0 }))?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    loop {
        canvas.fill_solid(
            &Rectangle {
                top_left: Point { x: 0, y: 0 },
                size: matrix_canvas_size,
            },
            Rgb888::new(0, 0, 0),
        )?;

        let text_style = MonoTextStyle::new(&FONT_9X15, Rgb888::new(0x00, 0x00, 0x00));

        let logo = std::fs::read("./38c3-128.bmp")?;
        let logo_bmp: Bmp<Rgb888> = Bmp::from_slice(&logo).unwrap();
        let mut image_slide =
            ImageSlide::new(matrix_canvas_size, logo_bmp.size(), Dir { x: 1, y: 0 }, 100);
        logo_bmp.draw(&mut image_slide.canvas)?;

        for x in 0..(64 * 3 + logo_bmp.size().width + 100) {
            matrix_service.draw_canvas_at(&image_slide.get_canvas_at())?;
            image_slide.step();
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;

        let bsod = std::fs::read("./bsod.bmp")?;
        let bsod_bmp: Bmp<Rgb888> = Bmp::from_slice(&bsod).unwrap();
        let mut image_bouncer = ImageBouncer::new(matrix_canvas_size, bsod_bmp.size());
        bsod_bmp.draw(&mut image_bouncer.canvas)?;

        for tick in 0..500 {
            matrix_service.draw_canvas_at(&image_bouncer.get_canvas_at())?;
            image_bouncer.bounce();
            tracing::trace!("Image bouncer tick {}", tick);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        let pride_flag = std::fs::read("./trans_pride_flag.bmp")?;
        let pride_flag_bmp: Bmp<Rgb888> = Bmp::from_slice(&pride_flag).unwrap();
        let mut image_slide = ImageSlide::new(
            matrix_canvas_size,
            pride_flag_bmp.size(),
            Dir { x: 0, y: 1 },
            100,
        );
        pride_flag_bmp.draw(&mut image_slide.canvas)?;

        for x in 0..(64 * 3 + pride_flag_bmp.size().width + 100) {
            matrix_service.draw_canvas_at(&image_slide.get_canvas_at())?;
            image_slide.step();
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        let hwinkor31 = &std::fs::read("./hwinkor31.bmp")?;
        let hwinkor31_bmp: Bmp<Rgb888> = Bmp::from_slice(&hwinkor31).unwrap();
        let mut image_bouncer = ImageBouncer::new(matrix_canvas_size, hwinkor31_bmp.size());
        hwinkor31_bmp.draw(&mut image_bouncer.canvas)?;

        for tick in 0..500 {
            matrix_service.draw_canvas_at(&image_bouncer.get_canvas_at())?;
            image_bouncer.bounce();
            tracing::trace!("Image bouncer tick {}", tick);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        let mut conway = Conway::new(64 * 3, 64 * 2);

        conway.randomize_canvas(0.15);

        for tick in 0..500 {
            matrix_service.draw_canvas_at(&conway.canvas.place_at(Point { x: 0, y: 0 }))?;
            conway.calculate_next_frame();

            tracing::trace!("Conway tick {}", tick);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        //for x in 0..64 * 3 {
        //    canvas.fill_solid(
        //        &Rectangle {
        //            top_left: Point { x, y: 0 },
        //            size: Size {
        //                width: 1,
        //                height: 64 * 2,
        //            },
        //        },
        //        Rgb888::new(0xff, 0xff, 0xff),
        //    )?;
        //    canvas
        //        .place_at(Point { x: 0, y: 0 })
        //        .draw(&mut led_canvas)?;
        //    led_canvas = matrix.swap(led_canvas);
        //    thread::sleep(Duration::from_millis(10))
        //}

        //let eg_text = "Hello, LED! ;)";
        //Text::new(eg_text, Point::new(16, 16), text_style)
        //    .draw(&mut canvas)
        //    .unwrap();
        //for red in 0..255 {
        //    for green in 0..255 {
        //        for blue in 0..255 {
        //            let color = LedColor {
        //                red, green, blue
        //            };
        //            canvas.draw_line(0, cnt, 64*3, cnt, &color);
        //            cnt += 1;
        //            if cnt >= 64*2 {
        //                canvas = matrix.swap(canvas);
        //                thread::sleep(std::time::Duration::from_millis(3));
        //                cnt = 0;
        //            }
        //            //canvas.fill(&LedColor { red, green, blue });
        //        }
        //    }
        //}
    }
}
