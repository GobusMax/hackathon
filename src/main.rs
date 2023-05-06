use image::open;
#[path = "bin/detect.rs"]
mod detect;
mod image_manipulation;
fn main() {
    let a = open("data/plane_2_1.jpg").unwrap();
    let b = open("data/no_plane_2.jpg").unwrap();
    let mut diff = detect::difference2(a.into_rgb8(), b.into_rgb8());
    diff.save("Testdiff.jpg");
    //let a = image_manipulation::to_grayscale(a);
    //let b = image_manipulation::to_grayscale(b);
    //dbg!("Grayscaled");
    //let res = detect::average(detect::difference(a, b));
    //dbg!(res);
    //let a = open("data/plane.jpg").unwrap();
}
