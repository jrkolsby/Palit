use xmltree::Element;
use ndarray::arr2;
use std::fs::{self, File};
use std::env;
use std::io::Write;

const SIZE: f32 = 512.0;
const BEZIER_LEN: u8 = 10;

// TODO: Does our font database need to contain floats?

#[derive(Clone, Debug)]
struct Line {
    character: String,
    x: u16,
    y: u16,
    z: u16,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    length: u16,
}

enum Transform {
    Rotate(f32, f32, f32),
    Translate(f32, f32),
}

impl Line {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        let dx = x2 - x1;
        let dy = y2 - y1;

        let mid_x = x1 + (dx / 2.0);
        let mid_y = y1 + (dy / 2.0);
        let length = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
        let theta = if dx == 0.0 { 90. } else { 180. * (dy / dx).atan() / std::f32::consts::PI };
        let theta_norm = SIZE * (theta + 90.0) / 180.0;

        Line {
            x1, y1, x2, y2,
            x: mid_x as u16,
            y: mid_y as u16,
            z: theta_norm as u16,
            length: length as u16,
            character: "".to_string(),
        }
    }
}

struct Window {
    min_x: u16,
    max_x: u16,
    min_y: u16,
    max_y: u16,
    min_z: u16,
    max_z: u16,
}

impl Window {
    fn new(x: u16, y: u16, z: u16, dim: u16) -> Self {
        Window {
            min_x: x - dim,
            max_x: x + dim,
            min_y: y - dim,
            max_y: y + dim,
            min_z: z - dim,
            max_z: z + dim
        }
    }

    fn contains(&self, line: &Line) -> bool {
        return line.x >= self.min_x &&
            line.x <= self.max_x &&
            line.y >= self.min_y &&
            line.y <= self.max_y &&
            line.z >= self.min_z &&
            line.z <= self.max_z;
    }

    fn intersects(&self, window: &Window) -> bool {
        return self.max_x > window.min_x &&
            self.min_x < window.max_x &&
            self.max_y > window.min_y &&
            self.min_y < window.max_y &&
            self.max_z > window.min_z &&
            self.min_z < window.max_z
    }
}

// Node of an octree describing (x,y) and theta normalized to the same range as x,y
struct OctNode {
    children: Vec<OctNode>,
    data: Vec<Line>,
    window: Window
}

impl OctNode {
    fn new(mut el: Element) -> Self {

        let x = el.attributes.get("x").unwrap().parse::<u16>().unwrap();
        let y = el.attributes.get("y").unwrap().parse::<u16>().unwrap();
        let z = el.attributes.get("z").unwrap().parse::<u16>().unwrap();
        let dim = el.attributes.get("dim").unwrap().parse::<u16>().unwrap();

        let mut lines = vec![];

        while let Some(line) = el.take_child("line") {
            let line_x = line.attributes.get("x").unwrap().parse::<u16>().unwrap();
            let line_y = line.attributes.get("y").unwrap().parse::<u16>().unwrap();
            let line_z = line.attributes.get("z").unwrap().parse::<u16>().unwrap();
            let line_len = line.attributes.get("len").unwrap().parse::<u16>().unwrap();
            let line_char = line.attributes.get("char").unwrap();

            let unescaped_char: String = match line_char.as_str() {
                "&quot" => "\"".to_string(),
                "&apos;" => "'".to_string(),
                "&lt;" => "<".to_string(),
                "&gt;" => ">".to_string(),
                "&amp;" => "&".to_string(),
                x => x.to_string()
            };

            lines.push(Line {
                x1: 0.0,
                y1: 0.0,
                x2: 0.0,
                y2: 0.0,
                x: line_x,
                y: line_y,
                z: line_z,
                length: line_len,
                character: unescaped_char,
            });
        }

        let children: Vec<OctNode> = el.children.into_iter().map(|child_el| 
            OctNode::new(child_el)
        ).collect();

        OctNode {
            children,
            data: lines,
            window: Window::new(x, y, z, dim)
        }
    }

    fn search(&self, window: &Window) -> Vec<Line> {
        let mut lines: Vec<Line> = vec![];

        if !window.intersects(&self.window) {
            return lines;
        }

        for line in self.data.iter() {
            if window.contains(line) {
                lines.push(line.clone())
            }
        }

        if self.children.len() == 0 {
            return lines;
        }

        for child in self.children.iter() {
            lines.append(&mut child.search(window.clone()))
        }

        return lines;
    }
}

// Convert bezier curves to n lines 
fn interpolate_curve(x1: f32, y1: f32, cx1: f32, cy1: f32, 
    cx2: f32, cy2: f32, x2: f32, y2: f32, n: u8) -> Vec<Line> {

    let mut lines: Vec<Line> = vec![];

    let a = arr2(&[[-1.0, 3.0,-3.0, 1.0],
                   [ 3.0,-6.0, 3.0, 0.0],
                   [-3.0, 3.0, 0.0, 0.0],
                   [ 1.0, 0.0, 0.0, 0.0]]);
    
    let b = arr2(&[[x1,   y1],
                   [cx1, cy1],
                   [cx2, cy2],
                   [x2,   y2]]);
    
    let c = a.dot(&b);

    let mut cursor: Option<(f32, f32)> = None;
    let step: f32 = 1.0 / n as f32;
    let mut t: f32 = 0.0;
    while t <= 1.0 {
        let d = arr2(&[[t.powf(3.0), t.powf(2.0), t, 1.0]]);

        let point = d.dot(&c);
        let (_x1, _y1) = (point[[0,0]], point[[0,1]]);

        if let Some((_x2, _y2)) = cursor {
            lines.push(Line::new(_x1, _y1, _x2, _y2));
        }

        cursor = Some((_x1, _y1));
        t += step;
    }

    // Final point to end
    let (_xf, _yf) = cursor.unwrap();
    lines.push(Line::new(_xf, _yf, x2, y2));

    return lines;
}

fn convert_polyline(points: &str) -> Vec<Line> {
    return vec![];
}

// Parse an SVG path description to a Vec of Lines
fn convert_path(path: &str) -> Vec<Line> {
    let mut tokens: Vec<String> = vec![];
    let mut operand: Option<String> = None;

    // Tokenization
    for c in path.chars().rev() {
        match c {
            // Numbers (incl scientific notation)
            '0'..='9' | '.' | '-' | 'e' => { 
                operand = Some(format!("{}{}", c, 
                    operand.unwrap_or("".to_string()))
                ) 
            },
            // Delimiters
            ' ' | ',' => { 
                if let Some(token) = operand.take() { 
                    tokens.push(token) 
                } 
            },
            // Command or comma
            operator => {
                if let Some(token) = operand.take() { 
                    tokens.push(token) 
                }
                tokens.push(operator.to_string());
            }
        }
    }

    let mut lines: Vec<Line> = vec![];

    // Parsing
    let mut cursor: (f32, f32) = (0.0,0.0);
    let mut start: Option<(f32, f32)> = None;

    while let Some(token) = tokens.pop() {
        match &token[..] {
            // Sketch tends only to output the following absolute path commands 
            "M" => {
                let x = tokens.pop().unwrap().parse::<f32>().unwrap();
                let y = tokens.pop().unwrap().parse::<f32>().unwrap();

                cursor = (x, y);
            },
            "L" => {
                let (x1, y1) = cursor;
                let x2 = tokens.pop().unwrap().parse::<f32>().unwrap();
                let y2 = tokens.pop().unwrap().parse::<f32>().unwrap();

                lines.push(Line::new(x1, y1, x2, y2));

                cursor = (x2, y2);
            },
            "C" => {
                let (x1, y1) = cursor;
                let cx1 = tokens.pop().unwrap().parse::<f32>().unwrap();
                let cy1 = tokens.pop().unwrap().parse::<f32>().unwrap();
                let cx2 = tokens.pop().unwrap().parse::<f32>().unwrap();
                let cy2 = tokens.pop().unwrap().parse::<f32>().unwrap();
                let x2 = tokens.pop().unwrap().parse::<f32>().unwrap();
                let y2 = tokens.pop().unwrap().parse::<f32>().unwrap();

                lines = [lines, interpolate_curve(x1, y1, cx1, cy1, cx2, cy2, x2, y2, BEZIER_LEN)].concat();

                cursor = (x2, y2);
            },
            "Z" => {
                let (x1, y1) = cursor;
                let (x2, y2) = start.unwrap();

                lines.push(Line::new(x1, y1, x2, y2));

                cursor = (x2, y2);
            },
            _ => {},
        }
        if start.is_none() {
            start = Some(cursor.clone());
        }
    }

    return lines;
}

fn collect_lines(mut el: Element) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![];

    while let Some(path) = el.take_child("path") {
        let path = path.attributes.get("d").unwrap();
        lines = [lines, convert_path(path)].concat();
    }

    while let Some(group) = el.take_child("g") {
        lines = [lines, collect_lines(group)].concat();
    }

    return lines;
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 3 {
        println!("Usage: rendersvg <font.xml> <frame.svg>");
        return Ok(())
    }

    let db_str: String = fs::read_to_string(&argv[1]).unwrap();
    let db: Element = Element::parse(db_str.as_bytes()).unwrap();
    
    // Build octree from xml
    let root = OctNode::new(db);

    // BEGIN RENDER
    println!("START");

    let svg_str: String = fs::read_to_string(&argv[2]).unwrap();
    let svg: Element = Element::parse(svg_str.as_bytes()).unwrap();

    // Convert SVG to all straight lines and normalize angles
        // [X] Bezier curves 
        // [] Shapes (ellipse, circle, rectangle)
        // [] Strip text

    let lines = collect_lines(svg);

    let mut line_test = File::create("frame_lines.svg").unwrap();
    write!(line_test, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400px\" height=\"400px\" viewBox=\"0 0 400 400\">").unwrap();
    write!(line_test, "<g stroke=\"#000000\" stroke-width=\"1\">").unwrap();

    for line in lines.iter() {
        write!(line_test, "<path d=\"M {} {} L {} {} \" />", line.x1, line.y1, line.x2, line.y2).unwrap();
    }

    write!(line_test, "</g></svg>").unwrap();

    // Initialize character grid of size h * w * [(score: u16, char: String)]

    // For each line in svg, compute x, y, theta_norm and search root for nearest neighbors

    let search_area = Window::new(256, 256, 512, 50);
    let nearest = root.search(&search_area);

    let chars: Vec<String> = nearest.iter().map(|l| l.character.clone()).collect();

    println!("{:?}", chars);

    //  Score the similarity of the lines by angle (z), length, then distance

    //  Accumulate the score for the overlapping character in the grid 

    // For each char in grid, print first element

    Ok(())
}