use crate::color::Color;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct Canvas {
    pub width: isize,
    pub height: isize,
    pub pixel: Vec<Pixel>,
}

#[derive(Debug, Clone)]
pub struct Pixel {
    pub x: isize,
    pub y: isize,
    pub color: Color,
}

impl Canvas {
    pub fn new(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            pixel: vec![],
        }
    }

    pub fn write_pixel(&mut self, x: isize, y: isize, color: Color) {
        if x > self.width || y > self.height {
            panic!("you can't set each of coordinates bigger than canvas");
        }
        self.pixel.push(Pixel { x, y, color });
    }

    pub fn pixel_at(&self, x: isize, y: isize) -> Option<Color> {
        let pixel_with_coor: Vec<Pixel> = self
            .pixel
            .clone()
            .into_iter()
            .filter(|item| item.x == x && item.y == y)
            .collect();

        if pixel_with_coor.len() >= 1 {
            Some(pixel_with_coor[0].color)
        } else {
            None
        }
    }

    pub fn canvas_to_ppm(&mut self) -> String {
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut pixel_body = "".to_string();

        for i in 0..self.height {
            let mut pixel_line = String::new();

            for j in 0..self.width {
                let is_existed_pixel = self.pixel_at(j, i);
                if let Some(color) = is_existed_pixel {
                    if pixel_line.len() != 60 {
                        if j == (self.width - 1) {
                            pixel_line = format!(
                                "{}{} {} {}",
                                pixel_line,
                                color.red * 255.0,
                                (color.green * 255.0).round(),
                                color.blue * 255.0
                            );
                        } else {
                            pixel_line = format!(
                                "{}{} {} {} ",
                                pixel_line,
                                color.red * 255.0,
                                (color.green * 255.0).round(),
                                color.blue * 255.0
                            );
                        }
                    } else {
                        pixel_line = format!(
                            "{}{} {}\n{} ",
                            pixel_line,
                            color.red * 255.0,
                            (color.green * 255.0).round(),
                            color.blue * 255.0
                        );
                    }
                } else {
                    if pixel_line.len() != 60 {
                        if j == (self.width - 1) {
                            pixel_line = format!("{}0 0 0", pixel_line);
                        } else {
                            pixel_line = format!("{}0 0 0 ", pixel_line);
                        }
                    } else {
                        pixel_line = format!("{}0 0\n0 ", pixel_line);
                    }
                }
            }
            pixel_body = format!("{}{}\n", pixel_body, pixel_line);
        }
        let result = format!("{}{}", header, pixel_body);
        result
    }

    pub fn write_file(&mut self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path).expect("path is wrong");
        let file_data = self.canvas_to_ppm();
        file.write(file_data.as_bytes())
            .expect("data format is wrong");
        Ok(())
    }
}
