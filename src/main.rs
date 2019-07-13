use ansi_term::{Colour, Style};

fn main() {
    eprintln!("{}", Colour::Red.paint("エラーが発生しましたよ"));
    println!(
        "一部の色だけを変えてみる: {}",
        Colour::Yellow.paint("色変わってますか？？")
    );
    println!(
        "{}そして，{}",
        Style::new().underline().paint("下線引けていますか？"),
        Style::new().bold().paint("太字になっていますか？")
    );
}
