use serde::Serialize;
use opencv::{core, imgcodecs, imgproc, objdetect, prelude::*};
use anyhow::Result;

#[derive(Serialize)]
struct FaceDetectionResult {
    count: usize,
    faces: Vec<(i32, i32, i32, i32)>,
}

#[api]
fn detect_faces(path: String) -> Result<FaceDetectionResult> {
    // Bild laden
    let img = imgcodecs::imread(&path, imgcodecs::IMREAD_GRAYSCALE)?;

    // Klassifikator laden
    let mut face_cascade =
        objdetect::CascadeClassifier::new("haarcascade_frontalface_default.xml")?;

    // Erkennung
    let mut faces = opencv::types::VectorOfRect::new();
    face_cascade.detect_multi_scale(
        &img,
        &mut faces,
        1.1,
        3,
        0,
        core::Size::new(30, 30),
        core::Size::new(0, 0),
    )?;

    let rects = faces.iter().map(|r| (r.x, r.y, r.width, r.height)).collect();

    Ok(FaceDetectionResult {
        count: rects.len(),
        faces: rects,
    })
}
