use chrono::Utc;
use opencv::{core::Point, imgcodecs, imgproc, prelude::Mat};
use screenshots::Screen;

use std::{error::Error, fs, time::Instant};
use tauri::regex::Regex;

use crate::models::DiabloItem;

pub fn process_item() -> Result<DiabloItem, Box<dyn Error>> {
    let start = Instant::now();

    //Take screenshot of main display
    let screen = Screen::all()
        .unwrap()
        .into_iter()
        .find(|s| s.display_info.is_primary)
        .unwrap();

    let capture = screen.capture()?;
    let buffer = capture.to_png(None)?;

    //Convert PNG image to opencv MAT
    let img = imgcodecs::imdecode(
        &Mat::from_slice(buffer.as_slice())?,
        imgcodecs::IMREAD_COLOR,
    )?;

    let templates = [
        "ocr_template/bottom_left.png",
        "ocr_template/bottom_right.png",
        "ocr_template/top_left.png",
        "ocr_template/top_right.png",
    ];

    let mut positions: Vec<Point> = Vec::new();

    //Iterate through templates to get bouding box of the hovered item
    for template_file in templates {
        let template = imgcodecs::imread(template_file, imgcodecs::IMREAD_COLOR)?;

        //Template matching
        let mut res = Mat::default();
        imgproc::match_template(
            &img,
            &template,
            &mut res,
            imgproc::TM_CCOEFF_NORMED,
            &Mat::default(),
        )?;

        let min_val: Option<&mut f64> = None;
        let max_val: Option<&mut f64> = None;
        let min_loc: Option<&mut Point> = None;
        let max_loc: &mut Point = &mut Point::default();

        opencv::core::min_max_loc(
            &res,
            min_val,
            max_val,
            min_loc,
            Some(max_loc),
            &Mat::default(),
        )?;

        positions.push(max_loc.to_owned());
    }

    if positions.len() != 4 {
        println!("Not all templates matched, exiting");
        return Err("Not all templates matched, exiting")?;
    }

    //Crop image based on template matching
    let start_row = positions[2].y;
    let end_row = positions[0].y;
    let start_col = positions[0].x;
    let end_col = positions[1].x;

    let cropped_image = Mat::roi(
        &img,
        opencv::core::Rect::new(
            start_col,
            start_row,
            end_col - start_col,
            end_row - start_row,
        ),
    )
    .map_err(|_| "Error generating cropped image")?;

    let now = Utc::now().timestamp_micros();
    let filepath = format!("ocr_results/{}.png", now);

    imgcodecs::imwrite(
        filepath.as_str(),
        &cropped_image,
        &opencv::core::Vector::default(),
    )?;

    let tes_img = rusty_tesseract::Image::from_path(filepath)?;
    let args = rusty_tesseract::Args::default();
    let text_result = rusty_tesseract::image_to_string(&tes_img, &args)?;

    let remove_characters = r"[\\|/*?'-]";
    let regex_remove = Regex::new(remove_characters)?;
    let pretty_result = regex_remove
        .replace_all(&text_result, "")
        .to_string()
        .to_uppercase()
        .replace("@ ", "")
        .replace("® ", "")
        .replace("© ", "")
        .replace("@", "O")
        .replace("®", "O")
        .replace("©", "O")
        .replace("¢", "")
        .replace("{", "[")
        .replace("}", "]")
        .replace("OOO", "OO")
        .replace("&", "E");

    fs::write(format!("ocr_results/{}.txt", now), pretty_result.to_owned())?;

    println!("Processed image in {:?}", start.elapsed());

    return Ok(DiabloItem::new(pretty_result.to_owned(), now.to_string()));
}
