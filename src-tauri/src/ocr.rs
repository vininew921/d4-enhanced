use chrono::Utc;
use opencv::{core::Point, imgcodecs, imgproc, prelude::Mat};
use screenshots::Screen;
use std::time::Instant;

pub fn process_item() {
    let start = Instant::now();

    //Take screenshot of main display
    let screen = Screen::all()
        .unwrap()
        .into_iter()
        .find(|s| s.display_info.is_primary)
        .unwrap();

    let capture = screen.capture().unwrap();
    let buffer = capture.to_png(None).unwrap();

    //Convert PNG image to opencv MAT
    let img = imgcodecs::imdecode(
        &Mat::from_slice(buffer.as_slice()).unwrap(),
        imgcodecs::IMREAD_COLOR,
    )
    .unwrap();

    let templates = [
        "ocr_template/bottom_left.png",
        "ocr_template/bottom_right.png",
        "ocr_template/top_left.png",
        "ocr_template/top_right.png",
    ];

    let mut positions: Vec<Point> = Vec::new();

    //Iterate through templates to get bouding box of the hovered item
    for template_file in templates {
        let template = imgcodecs::imread(template_file, imgcodecs::IMREAD_COLOR).unwrap();

        //Template matching
        let mut res = Mat::default();
        imgproc::match_template(
            &img,
            &template,
            &mut res,
            imgproc::TM_CCOEFF_NORMED,
            &Mat::default(),
        )
        .unwrap();

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
        )
        .unwrap();

        positions.push(max_loc.to_owned());
    }

    if positions.len() != 4 {
        println!("Not all templates matched, exiting");
        return;
    }

    //Crop image based on template matching
    let start_row = positions[2].y;
    let end_row = positions[0].y;
    let start_col = positions[0].x;
    let end_col = positions[1].x;

    let cropped_image_result = Mat::roi(
        &img,
        opencv::core::Rect::new(
            start_col,
            start_row,
            end_col - start_col,
            end_row - start_row,
        ),
    );

    if let Ok(cropped_image) = cropped_image_result {
        println!("Processed image in {:?}", start.elapsed());

        imgcodecs::imwrite(
            format!("ocr_results/{}.png", Utc::now().timestamp_micros()).as_str(),
            &cropped_image,
            &opencv::core::Vector::default(),
        )
        .unwrap();
    }

    println!("Error generating cropped image");
}