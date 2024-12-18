use std::thread;
use std::time::Duration;
use rpi_led_matrix::{LedMatrix, LedColor, LedMatrixOptions, LedRuntimeOptions};

fn main() {
    tracing_subscriber::fmt::init();

    let mut matrix_options = LedMatrixOptions::new();
    matrix_options.set_cols(64);
    matrix_options.set_rows(64);
    matrix_options.set_chain_length(3);
    matrix_options.set_parallel(2);
    matrix_options.set_pwm_lsb_nanoseconds(130);
    matrix_options.set_refresh_rate(true);
    matrix_options.set_hardware_pulsing(true);
    matrix_options.set_limit_refresh(120);

    let mut runtime_options = LedRuntimeOptions::new();
    runtime_options.set_gpio_slowdown(2);

    let matrix = LedMatrix::new(Some(matrix_options), Some(runtime_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();
    let mut cnt = 0;
    canvas.fill(&LedColor{
        red: 255,
        green: 255,
        blue: 255,
    });
    canvas = matrix.swap(canvas);
    thread::sleep(Duration::from_secs(300));
    for red in 0..255 {
        for green in 0..255 {
            for blue in 0..255 {
                let color = LedColor {
                    red, green, blue
                };
                canvas.draw_line(0, cnt, 64*3, cnt, &color);
                cnt += 1;
                if cnt >= 64*2 {
                    canvas = matrix.swap(canvas);
                    thread::sleep(std::time::Duration::from_millis(3));
                    cnt = 0;
                }
                //canvas.fill(&LedColor { red, green, blue });
            }
        }
    }
}
