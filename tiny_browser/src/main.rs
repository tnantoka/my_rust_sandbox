mod css;
mod dom;
mod html;
mod layout;
mod painting;
mod style;

use std::fs::File;
use std::io::Read;

fn main() {
    // char_indices
    // let input = String::from("test");
    // println!("{}", input);
    // println!(
    //     "{}",
    //     input[1..]
    //         .char_indices()
    //         .map(|(i, c)| format!("{}: {}", i, c))
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );

    let html = read_source("examples/index.html");
    let css = read_source("examples/style.css");

    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let filename = "output.png";

    let ok = {
        let canvas = painting::paint(&layout_root, viewport.content);
        let (w, h) = (canvas.width as u32, canvas.height as u32);
        let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
            let color = canvas.pixels[(y * w + x) as usize];
            *image::Pixel::from_slice(&[color.r, color.g, color.b, color.a])
        });
        image::DynamicImage::ImageRgba8(img).save(&filename).is_ok()
    };
    if ok {
        println!("Saved output as {}", filename)
    } else {
        println!("Error saving output as {}", filename)
    }
}

fn read_source(filename: &str) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}
