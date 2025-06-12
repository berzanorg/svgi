use oxipng::optimize_from_memory;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Tree},
};
use std::{env::args, fs, path::Path};

fn main() {
    let logo_path = args().nth(1).expect("svg icon path is not given");
    let logo_path = Path::new(&logo_path);

    let svg_data = fs::read(logo_path).expect("svg icon could not be found");

    let opt = Options::default();
    let tree = Tree::from_data(&svg_data, &opt).expect("svg icon could not be parsed");

    let svg_size = tree.size();
    let svg_width = svg_size.width();
    let svg_height = svg_size.height();

    assert_eq!(svg_width, svg_height, "svg icon is not square");

    let variants = [("x", 400), ("dc", 512), ("gh", 1200)];

    for (name, width) in variants {
        let scale = width as f32 / svg_width;

        let transform = resvg::usvg::Transform::from_scale(scale, scale);

        let mut pixmap = Pixmap::new(width, width).expect("could not render svg icon");

        resvg::render(&tree, transform, &mut pixmap.as_mut());

        let png_data = pixmap.encode_png().expect("could not encode svg icon");

        let optimized_png = optimize_from_memory(
            &png_data,
            &oxipng::Options {
                optimize_alpha: true,
                strip: oxipng::StripChunks::Safe,
                ..oxipng::Options::max_compression()
            },
        )
        .expect("could not optimize svg icon");

        fs::write(
            logo_path.with_file_name(name).with_extension("png"),
            optimized_png,
        )
        .expect("could not save svg icon");
    }
}
