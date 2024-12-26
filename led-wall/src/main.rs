use crate::conway::Conway;
use crate::image_bouncer::ImageBouncer;
use embedded_canvas::Canvas;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Point, Size};
use embedded_graphics::image::{Image, ImageDrawable};
use embedded_graphics::mono_font::ascii::FONT_9X15;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::Text;

use embedded_graphics::Drawable;
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use std::time::Duration;
use tinybmp::Bmp;

mod conway;
mod image_bouncer;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let mut matrix_options = LedMatrixOptions::new();
    matrix_options.set_cols(64);
    matrix_options.set_rows(64);
    matrix_options.set_chain_length(3);
    matrix_options.set_parallel(2);
    //matrix_options.set_pwm_lsb_nanoseconds(130);
    matrix_options.set_pwm_lsb_nanoseconds(50);
    //matrix_options.set_refresh_rate(true);
    matrix_options.set_led_rgb_sequence("BGR");
    matrix_options.set_refresh_rate(false);
    matrix_options.set_hardware_pulsing(true);
    matrix_options.set_limit_refresh(120);

    let mut runtime_options = LedRuntimeOptions::new();
    runtime_options.set_gpio_slowdown(2);

    let matrix = LedMatrix::new(Some(matrix_options), Some(runtime_options)).unwrap();

    let mut canvas = Canvas::new(Size {
        width: 64 * 3,
        height: 64 * 2,
    });

    let mut led_canvas = matrix.offscreen_canvas();

    canvas.fill_solid(
        &Rectangle {
            top_left: Point { x: 0, y: 0 },
            size: Size {
                width: 64 * 3,
                height: 64 * 2,
            },
        },
        Rgb888::new(0, 255, 0),
    )?;

    canvas
        .place_at(Point { x: 0, y: 0 })
        .draw(&mut led_canvas)?;
    led_canvas = matrix.swap(led_canvas);

    tokio::time::sleep(Duration::from_secs(1)).await;

    loop {
        canvas.fill_solid(
            &Rectangle {
                top_left: Point { x: 0, y: 0 },
                size: Size {
                    width: 64 * 3,
                    height: 64 * 2,
                },
            },
            Rgb888::new(0, 0, 0),
        )?;

        let logo = std::fs::read("./38c3-128.bmp")?;
        let bmp = Bmp::from_slice(&logo).unwrap();

        let text_style = MonoTextStyle::new(&FONT_9X15, Rgb888::new(0x00, 0x00, 0x00));

        tracing::debug!("Start scrolling the logo");
        for x in (-128 + 32)..32 {
            tracing::trace!("off = {}", x);
            Image::new(&bmp, Point::new(x, 0)).draw(&mut canvas)?;
            canvas
                .place_at(Point { x: 0, y: 0 })
                .draw(&mut led_canvas)?;
            led_canvas = matrix.swap(led_canvas);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;

        let bsod = std::fs::read("./bsod.bmp")?;
        let bsod_bmp: Bmp<Rgb888> = Bmp::from_slice(&bsod).unwrap();

        let bsod_size = bsod_bmp.size();

        let mut image_bouncer = ImageBouncer::new(
            Size {
                width: 64 * 3,
                height: 64 * 2,
            },
            bsod_size,
        );

        bsod_bmp.draw(&mut image_bouncer.canvas)?;

        for tick in 0..1000 {
            image_bouncer.get_canvas_at().draw(&mut led_canvas)?;
            led_canvas = matrix.swap(led_canvas);
            image_bouncer.bounce();
            tracing::trace!("Image bouncer tick {}", tick);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        let mut conway = Conway::new(64 * 3, 64 * 2);

        conway.randomize_canvas(0.15);

        for tick in 0..10000 {
            conway
                .canvas
                .place_at(Point { x: 0, y: 0 })
                .draw(&mut led_canvas)?;
            led_canvas = matrix.swap(led_canvas);
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
