#[warn(dead_code)]
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct Pattern {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Number of holes")]
    number_of_holes: u32,
    #[serde(rename = "Colour palette")]
    colour_palette: Vec<String>,
    #[serde(rename = "Weft colour")]
    weft_colour: u32,
    #[serde(rename = "Number of tablets")]
    number_of_tablets: u32,
    #[serde(rename = "Number of weaving rows")]
    number_of_weaving_rows: u32,
    #[serde(rename = "Threading chart")]
    threading_chart: Vec<Vec<u32>>,
    #[serde(rename = "Tablet orientations")]
    tablet_orientations: Vec<String>,
    #[serde(rename = "Pattern design")]
    pattern_design: PatternDesign,
}
#[derive(Serialize, Deserialize, Debug)]
struct PatternDesign {
    #[serde(rename = "weavingInstructions")]
    weaving_instructions: Vec<Vec<WeavingInstruction>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct WeavingInstruction {
    direction: String,
    #[serde(rename = "numberOfTurns")]
    number_of_turns: u32,
}

#[macroquad::main(window_conf)]
async fn main() {
    let path = std::path::Path::new("Patterns/6 card shoelace x O.twt");
    let file = File::open(path).unwrap();
    let pattern: Pattern = serde_json::from_reader(file).unwrap();
    let colors: Vec<Color> = pattern
        .colour_palette
        .iter()
        .map(|s| {
            let r = u8::from_str_radix(&s[1..3], 16).unwrap();
            let g = u8::from_str_radix(&s[3..5], 16).unwrap();
            let b = u8::from_str_radix(&s[5..7], 16).unwrap();
            Color::from_rgba(r, g, b, 255)
        })
        .collect();

    let font_size: u16 = 30;
    let padding: f32 = 10.0;
    let start_display_setup = 80.0;
    let mut setup: bool = true;
    let mut _current_row: u32 = 0;
    let hole_letters = ["H", "G", "F", "E", "D", "C", "B", "A"];

    loop {
        clear_background(DARKGRAY);
        let dim_pattern_name = measure_text(&pattern.name, None, font_size, 1.0);
        draw_text(
            &pattern.name,
            (screen_width() - dim_pattern_name.width) / 2.0,
            dim_pattern_name.height + 10.0,
            font_size as f32,
            WHITE,
        );

        if setup {
            draw_setup_text(
                font_size as f32,
                &pattern,
                start_display_setup,
                hole_letters,
                padding,
                pattern.number_of_holes,
            );
            draw_setup_thread_colors(
                font_size as f32,
                &pattern,
                start_display_setup,
                padding,
                colors.clone(),
            );
            if is_mouse_button_down(MouseButton::Left) {
                setup = false;
            }
        } else {
        }
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Twisted Threads".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

fn draw_setup_text(
    font_size: f32,
    pattern: &Pattern,
    start_display_setup: f32,
    hole_letters: [&str; 8],
    padding: f32,
    number_of_holes: u32,
) {
    // number_of_holes + 1 to account for the extra space at the top of the display
    // + number_of_holes * padding to account for the padding between the lines
    let vertical_space =
        (number_of_holes as f32 + 1.0) * font_size + number_of_holes as f32 * padding;
    let horizontal_space =
        pattern.number_of_tablets as f32 * font_size + pattern.number_of_tablets as f32 * padding;
    // Draw the tablet numbers at the top of the display
    for i in 0..pattern.number_of_tablets {
        draw_text(
            format!("{}", i + 1),
            (screen_width() - horizontal_space) / 2.0 + i as f32 * font_size + i as f32 * padding,
            start_display_setup,
            font_size,
            WHITE,
        );
    }
    // Draw the hole letters on the left side of the display
    for i in 0..pattern.number_of_holes {
        draw_text(
            hole_letters[(hole_letters.len() as u32 - number_of_holes) as usize + i as usize],
            (screen_width() - horizontal_space) / 2.0 - padding - 15.0,
            start_display_setup + 20.0 + font_size + i as f32 * font_size + i as f32 * padding,
            font_size,
            WHITE,
        );
    }
    // Draw the tablet orientations
    for i in 0..pattern.number_of_tablets {
        draw_text(
            pattern.tablet_orientations[i as usize].as_str(),
            (screen_width() - horizontal_space) / 2.0 + i as f32 * font_size + i as f32 * padding,
            start_display_setup + vertical_space + 20.0,
            font_size,
            WHITE,
        );
    }
}
fn draw_setup_thread_colors(
    font_size: f32,
    pattern: &Pattern,
    start_display_setup: f32,
    padding: f32,
    colors: Vec<Color>,
) {
    for i in 0..pattern.number_of_tablets {
        for j in 0..pattern.number_of_holes {
            draw_rectangle(
                (screen_width()
                    - pattern.number_of_tablets as f32 * font_size as f32
                    - pattern.number_of_tablets as f32 * padding)
                    / 2.0
                    + i as f32 * font_size as f32
                    + i as f32 * padding,
                start_display_setup
                    + font_size as f32
                    + j as f32 * font_size as f32
                    + j as f32 * padding,
                font_size as f32,
                font_size as f32,
                colors[pattern.threading_chart[j as usize][i as usize] as usize],
            );
        }
    }
}
