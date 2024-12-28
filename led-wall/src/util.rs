use embedded_canvas::CanvasAt;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::Drawable;
use rpi_led_matrix::{LedCanvas, LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use std::cell::Cell;

pub struct Dir {
    pub x: i32,
    pub y: i32,
}

pub struct MatrixService {
    matrix: LedMatrix,
    led_canvas: Cell<Option<LedCanvas>>,
}

impl MatrixService {
    pub fn new() -> Self {
        let mut matrix_options = LedMatrixOptions::new();
        matrix_options.set_cols(64);
        matrix_options.set_rows(64);
        matrix_options.set_chain_length(3);
        matrix_options.set_parallel(2);
        //matrix_options.set_pwm_lsb_nanoseconds(130);
        matrix_options.set_pwm_lsb_nanoseconds(50);
        //matrix_options.set_refresh_rate(true);
        matrix_options.set_brightness(75).unwrap();
        matrix_options.set_led_rgb_sequence("BGR");
        matrix_options.set_refresh_rate(false);
        matrix_options.set_hardware_pulsing(true);
        matrix_options.set_limit_refresh(120);

        let mut runtime_options = LedRuntimeOptions::new();
        runtime_options.set_gpio_slowdown(2);

        let matrix = LedMatrix::new(Some(matrix_options), Some(runtime_options)).unwrap();
        let led_canvas = matrix.offscreen_canvas();

        Self {
            matrix,
            led_canvas: Cell::new(Some(led_canvas)),
        }
    }

    pub fn draw_canvas_at(&mut self, canvas_at: &CanvasAt<Rgb888>) -> Result<(), anyhow::Error> {
        let mut led_canvas = self.led_canvas.take().unwrap();
        canvas_at.draw(&mut led_canvas)?;
        led_canvas = self.matrix.swap(led_canvas);
        self.led_canvas.replace(Some(led_canvas));

        Ok(())
    }
}
