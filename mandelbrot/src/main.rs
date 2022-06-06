extern crate image;
extern crate num;

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use num::Complex;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: Mandelbrot NOMFIC PIXELS SUPGA INFER"
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Exemple: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }

    let bords = analy_paire(&args[2], 'x').expect("Erreur d'analyse de la dimension des bords…");
    let super_ga = analy_complex(&args[3]).expect("Erreur analyse de point super gauche");
    let super_dr = analy_complex(&args[4]).expect("Erreur analyse de point super droit");

    let mut pixels = vec![0; bords.0 * bords.1];
    render(&mut pixels, bords, super_ga, super_dr);
    ecrire_image(&args[1], &pixels, bords).expect("Erreur d'écriture du fichier png");
}

fn ecrire_image(nomfic: &str, pixel: &[u8], bords: (usize, usize)) -> Result<(), std::io::Error> {
    let out = File::create(nomfic)?;
    let encodeur = PngEncoder::new(out);
    encodeur
        .write_image(pixel, bords.0 as u32, bords.1 as u32, ColorType::L8)
        .expect("Error while write file");

    Ok(())
}

fn render(
    pixels: &mut [u8],
    bords: (usize, usize),
    super_ga: Complex<f64>,
    infer_dr: Complex<f64>,
) {
    assert!(pixels.len() == bords.0 * bords.1);

    for lig in 0..bords.1 {
        for col in 0..bords.0 {
            let point = pixel_en_point(bords, (col, lig), super_ga, infer_dr);
            pixels[lig * bords.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn pixel_en_point(
    bords: (usize, usize),
    pixel: (usize, usize),
    super_ga: Complex<f64>,
    infer_dr: Complex<f64>,
) -> Complex<f64> {
    let (largeur, hauteur) = (infer_dr.re - super_ga.re, super_ga.im - infer_dr.im);
    Complex {
        re: super_ga.re + pixel.0 as f64 * largeur / bords.0 as f64,
        im: super_ga.im - pixel.1 as f64 * hauteur / bords.1 as f64,
    }
}

fn analy_complex(s: &str) -> Option<Complex<f64>> {
    match analy_paire(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

fn analy_paire<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[allow(dead_code)]
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c
    }
}

#[test]
fn test_pixel_en_point() {
    assert_eq!(
        pixel_en_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

#[test]
fn test_analy_complex() {
    assert_eq!(
        analy_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(analy_complex(",-0.0625"), None);
}

#[test]
fn test_analy_paire() {
    assert_eq!(analy_paire::<i32>("", ','), None);
    assert_eq!(analy_paire::<i32>("10", ','), None);
    assert_eq!(analy_paire::<i32>(",10", ','), None);
    assert_eq!(analy_paire::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(analy_paire::<i32>("10,20xy", ','), None);
    assert_eq!(analy_paire::<f64>("0.5x", 'x'), None);
    assert_eq!(analy_paire::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}
