use figlet_rs::FIGfont;
use pancurses::{curs_set, endwin, initscr, Window};
use std::{thread::sleep, time::{Duration, Instant}};

#[cfg(target_os = "windows")]
const NEWLINE: &str = "\n\r";
#[cfg(not(target_os = "windows"))]
const NEWLINE: &str = "\n";

/// adds whitespace to text to move it to (x, y)
fn pad_to_position(text: &str, x: i32, y: i32) -> String {
    let text_width = text.lines().map(|line| line.len()).max().unwrap_or(0);
    let text_height = text.lines().count();

    // add left padding to text
    let text: String = text
        .lines()
        .map(|line| {
            format!(
                "{}{}",
                " ".repeat((x / 2) as usize - (text_width / 2)),
                line
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    // add top padding to text
    let mut text = text.to_owned();
    text.insert_str(0, &NEWLINE.repeat((y / 2) as usize - (text_height / 2)));

    text
}

/// draws stylized text to the center of the window
fn draw_text(window: &Window, text: &str) {
    let (y, x) = window.get_max_yx();

    let standard_font = FIGfont::standard().expect("standard font not found");
    let text = standard_font
        .convert(&text)
        .expect(
            "no text was provided daw_text\
                 (if developing, use `window.clear()`to clear instead)",
        )
        .to_string();

    window.clear();
    // window.printw(pad_to_position(&format!("{:?}", (x, y)), x, y));
    window.printw(pad_to_position(&text, x, y));
    window.refresh();
}

fn stopwatch(window: Window) {
    let start = Instant::now();
    loop {
        draw_text(&window, &start.elapsed().as_secs().to_string());
        sleep(Duration::new(0, (10 as u32).pow(7)));
    }
}

fn main() {
    let window = initscr();
    curs_set(0);
    stopwatch(window);
    endwin();
}

#[cfg(test)]
mod tests {
    use super::pad_to_position;

    #[test]
    fn test_pad_to_position() {
        assert_eq!(pad_to_position("@", 0, 0), "@");
        assert_eq!(pad_to_position("@", 2, 0), " @");
        assert_eq!("\n@", pad_to_position("@", 0, 2));
        assert_eq!("\n @", pad_to_position("@", 2, 2));
        assert_eq!("\n\n @@\n @@", pad_to_position("@@\n@@", 3, 5));
        assert_eq!(
            "\n\n\n  01\n  012\n  0",
            pad_to_position("01\n012\n0", 4, 6)
        );
        assert_eq!("  01\n  012\n  0", pad_to_position("01\n012\n0", 4, 1));
        assert_eq!("\n\n\n01\n012\n0", pad_to_position("01\n012\n0", 0, 6));
    }
}
