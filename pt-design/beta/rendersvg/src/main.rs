use std::fs::{self, File};
use std::env;
use std::io::{Write};
use std::os::unix::io::FromRawFd;
use std::collections::HashMap;

use bresenham::Bresenham;
use termion::{clear, cursor, terminal_size};
use termion::raw::IntoRawMode;
use xmltree::Element;
use ndarray::arr2;

const SIZE: f32 = 512.0;
const SEARCH_DIM: u16 = 40;
const BEZIER_LEN: u8 = 4;

/* TODO: 
- [ ] Convert all line coords to float
- [X] Bezier curves 
- [ ] Shapes (ellipse, circle, rectangle)
- [ ] Strip text
*/

/*
    HEURISTICS:
    x_pos: 0-50
    y_pos: 0-50
    theta: 0-50

    length: 0-50
    portion: 0-50

    SCORE: 0-250
*/

#[derive(Clone, Debug)]
struct Line {
    x: u16,
    y: u16,
    z: u16,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    slope: f32,
    length: u16,
    portion: u16,
    character: String,
}

#[derive(Clone, Debug)]
enum Transform {
    Rotate(f32, f32, f32),
    Translate(f32, f32),
    Scale(f32, f32),
}

fn transform_point(p: (f32, f32), t: Vec<Transform>) -> (f32, f32) {
    let (mut x, mut y) = p;
    for transform in t.iter() {
        let (_x, _y) = match transform {
            Transform::Translate(dx, dy) => (x + dx, y + dy),
            Transform::Scale(sx, sy) => (x * sx, y * sy),
            Transform::Rotate(theta, ox, oy) => {
                // https://stackoverflow.com/questions/2259476/rotating-a-point-about-another-point-2d
                let s = theta.to_radians().sin();
                let c = theta.to_radians().cos();
              
                // translate point back to origin:
                let x_t = x - ox;
                let y_t = y - oy;
              
                // rotate point
                let x_r = x_t * c - y_t * s;
                let y_r = x_t * s + y_t * c;
              
                // translate point back:
                (x_r + ox, y_r + oy)
            },
        };
        x = _x;
        y = _y;
    }
    return (x, y);
}

impl Line {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32, transforms: Vec<Transform>) -> Self {
        let (_x1, _y1) = transform_point((x1, y1), transforms.clone());
        let (_x2, _y2) = transform_point((x2, y2), transforms.clone());

        let dx = _x2 - _x1;
        let dy = _y2 - _y1;

        let slope = dy / dx;

        let mid_x = _x1 + (dx / 2.0);
        let mid_y = _y1 + (dy / 2.0);
        let length = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
        let theta = if dx == 0.0 { 90. } else { slope.atan().to_degrees() };
        let theta_norm = SIZE * (theta + 90.0) / 180.0;

        Line {
            x1: _x1, 
            y1: _y1, 
            x2: _x2, 
            y2: _y2,
            x: mid_x as u16,
            y: mid_y as u16,
            z: theta_norm as u16,
            slope,
            length: length as u16,
            portion: 0,
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
            min_x: if x > dim { x - dim } else { 0 },
            max_x: x + dim,
            min_y: if y > dim { y - dim } else { 0 },
            max_y: y + dim,
            min_z: if z > dim { z - dim } else { 0 },
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
            let line_portion = line.attributes.get("portion").unwrap().parse::<u16>().unwrap();
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
                // UNUSED
                x1: 0.0,
                y1: 0.0,
                x2: 0.0,
                y2: 0.0,
                slope: 0.0,
                x: line_x,
                y: line_y,
                z: line_z,
                length: line_len,
                portion: line_portion,
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
    cx2: f32, cy2: f32, x2: f32, y2: f32, n: u8, transforms: Vec<Transform>) -> Vec<Line> {

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
            lines.push(Line::new(_x1, _y1, _x2, _y2, transforms.clone()));
        }

        cursor = Some((_x1, _y1));
        t += step;
    }

    // Final point to end
    let (_xf, _yf) = cursor.unwrap();
    lines.push(Line::new(_xf, _yf, x2, y2, transforms.clone()));

    return lines;
}

fn convert_polyline(points: &str) -> Vec<Line> {
    return vec![];
}

// Parse an SVG path description to a Vec of Lines
fn parse_path(path: &str, transforms: Vec<Transform>) -> Vec<Line> {
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

                lines.push(Line::new(x1, y1, x2, y2, transforms.clone()));

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

                lines = [
                    lines, 
                    interpolate_curve(x1, y1, cx1, cy1, cx2, cy2, x2, y2, BEZIER_LEN, transforms.clone())
                ].concat();

                cursor = (x2, y2);
            },
            "Z" => {
                let (x1, y1) = cursor;
                let (x2, y2) = start.unwrap();

                lines.push(Line::new(x1, y1, x2, y2, transforms.clone()));

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

fn take_transform(transforms: &str) -> Option<(&str, Transform)> {

    if let Some(args_begin) = transforms.find('(') {
        let args_end = args_begin + transforms[args_begin..].find(')').unwrap();
        let args: Vec<&str> = transforms[args_begin+1..args_end].split(',').map(|a| a.trim()).collect();

        match &transforms[..args_begin] {
            "translate" => {
                let t_x: f32 = if let Some(arg) = args.get(0) { arg.parse::<f32>().unwrap() } else { 0.0 };
                let t_y: f32 = if let Some(arg) = args.get(1) { arg.parse::<f32>().unwrap() } else { 0.0 };

                return Some((&transforms[args_end+1..].trim(), Transform::Translate(t_x, t_y)))
            },
            "rotate" => {
                let theta: f32 = if let Some(arg) = args.get(0) { arg.parse::<f32>().unwrap() } else { 0.0 };
                let o_x: f32 = if let Some(arg) = args.get(1) { arg.parse::<f32>().unwrap() } else { 0.0 };
                let o_y: f32 = if let Some(arg) = args.get(2) { arg.parse::<f32>().unwrap() } else { 0.0 };

                return Some((&transforms[args_end+1..].trim(), Transform::Rotate(theta, o_x, o_y)))
            }
            "scale" => {
                let s_x: f32 = if let Some(arg) = args.get(0) { arg.parse::<f32>().unwrap() } else { 0.0 };
                let s_y: f32 = if let Some(arg) = args.get(1) { arg.parse::<f32>().unwrap() } else { 0.0 };

                return Some((&transforms[args_end+1..].trim(), Transform::Scale(s_x, s_y)))
            }
            a @ _ => { eprintln!("Unknown Transform {}", a); }
        }
    }

    return None;
}

fn parse_transforms(mut transforms: &str) -> Vec<Transform> {
    let mut result: Vec<Transform> = vec![];

    while let Some((new_transforms, transform)) = take_transform(transforms) {
        transforms = new_transforms;
        result.insert(0, transform);
    }

    return result;
}

fn collect_lines(mut el: Element, transforms: Vec<Transform>) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![];

    while let Some(path) = el.take_child("path") {
        let t = if let Some(transform_str) = path.attributes.get("transform") {
            [transforms.clone(), parse_transforms(transform_str)].concat()
        } else {
            transforms.clone()
        };
        let path_str = path.attributes.get("d").unwrap();
        lines = [lines, parse_path(path_str, t)].concat();
    }

    while let Some(group) = el.take_child("g") {
        let t = if let Some(transform_str) = group.attributes.get("transform") {
            [transforms.clone(), parse_transforms(transform_str)].concat()
        } else {
            transforms.clone()
        };
        lines = [lines, collect_lines(group, t)].concat();
    }

    return lines;
}

fn write_svg(filename: &str, lines: Vec<Line>) {
    let mut line_test = File::create(filename).unwrap();
    write!(line_test, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400px\" height=\"400px\" viewBox=\"0 0 400 400\">").unwrap();
    write!(line_test, "<g stroke=\"#000000\" stroke-width=\"1\">").unwrap();

    for line in lines.iter() {
        write!(line_test, "<path d=\"M {} {} L {} {} \" />", line.x1, line.y1, line.x2, line.y2).unwrap();
    }

    write!(line_test, "</g></svg>").unwrap();
}

fn heuristic(line: &Line) -> u16 {
    return 3*line.length + line.portion;
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

    let mut out = unsafe {
        File::from_raw_fd(1).into_raw_mode().unwrap()
    };

    write!(out, "{}", clear::All).unwrap();

    // Convert SVG to lines and comute x,y,z and theta
    let svg_str: String = fs::read_to_string(&argv[2]).unwrap();
    let svg: Element = Element::parse(svg_str.as_bytes()).unwrap();

    let width = svg.attributes.get("width").unwrap().replace("px", "").parse::<f32>().unwrap();
    let height = svg.attributes.get("height").unwrap().replace("px", "").parse::<f32>().unwrap();

    let lines = collect_lines(svg, vec![]);

    // Initialize character grid containing [(score: u8, char: String)] for each voxel
    let size: (u16, u16) = terminal_size().unwrap();
    let mut canvas: Vec<Vec<HashMap<String, u16>>> = vec![vec![HashMap::new(); size.0 as usize]; size.1 as usize];

    let voxel_w = width / size.0 as f32;
    let voxel_h = height / size.1 as f32;
    let voxel_scale = SIZE / voxel_h;

    // For each line in svg, compute x, y, theta_norm and search root for nearest neighbors
    for line in lines.iter() {
        
        // Compute voxel intersections and search line segments, modify 
        // canvas while maintaining heuristic sort

        let voxel_start = (
            (line.x1 / voxel_w) as isize,
            (line.y1 / voxel_h) as isize
        );

        let voxel_end = (
            (line.x2 / voxel_w) as isize,
            (line.y2 / voxel_h) as isize
        );

        // Find voxels which intersect given line
        for (x, y) in Bresenham::new(voxel_start, voxel_end) {

            let pos = (y as usize, x as usize);

            if pos.0 >= size.1 as usize || pos.1 >= size.0 as usize {
                continue
            }

            let origin = (
                x as f32 * voxel_w,
                y as f32 * voxel_h
            );

            // Six possible endpoints for the line segment contained in this voxel
            let endpoints: Vec<(f32, f32)> = vec![
                (line.x1, line.y1),
                (line.x2, line.y2),
                // Intercepts left edge
                (origin.0, line.slope * (origin.0 - line.x1) + line.y1),
                // Intercepts right edge
                (origin.0 + voxel_w, line.slope * (origin.0 + voxel_w - line.x1) + line.y1),
                // Intercepts top edge
                (((origin.1 - line.y1) / line.slope) + line.x1, origin.1),
                // Intercepts bottom edge
                (((origin.1 + voxel_h - line.y1) / line.slope) + line.x1, origin.1 + voxel_h),
            ].into_iter().filter(|(x, y)| {
                *x >= origin.0 && 
                *x <= origin.0 + voxel_w &&
                *y >= origin.1 &&
                *y <= origin.1 + voxel_h
            }).collect();

            // Bresenham doesn't guarentee output voxels intersects line
            if endpoints.len() < 2 {
                continue
            }

            // Compute midpoint relative to voxel origin
            let (segment_x, segment_y) = (
                voxel_scale * (((endpoints[0].0 + endpoints[1].0) / 2.0) - origin.0),
                voxel_scale * (((endpoints[0].1 + endpoints[1].1) / 2.0) - origin.1)
            );

            let segment_window = Window::new(
                segment_x as u16,
                segment_y as u16,
                line.z, 
                SEARCH_DIM,
            );

            let mut nearest_neighbors = root.search(&segment_window);
            nearest_neighbors.sort_by(|a, b| heuristic(b).partial_cmp(&heuristic(a)).unwrap());

            for line in nearest_neighbors.iter() {
                // Insert in ascending order
                let h = heuristic(&line);
                let voxel: &mut HashMap<String, u16> = &mut canvas[pos.0][pos.1];

                // Get the sum of all 
                if let Some(heuristic) = voxel.remove(&line.character) {
                    voxel.insert(line.character.clone(), heuristic + h);
                } else {
                    voxel.insert(line.character.clone(), h);
                }
            }
        }
    }

    for x in 1..size.0 {
        for y in 1..size.1 {

            let voxel = &canvas[y as usize][x as usize];
            if voxel.len() > 0 {
                // Sort by heuristic value and print the largest
                let mut characters = voxel.iter().map(|(a, b)| (a, *b)).collect::<Vec<(&String, u16)>>();
                characters.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                write!(out, "{}{}", 
                    cursor::Goto(
                        x as u16, 
                        y as u16), 
                    characters[0].0,
                ).unwrap();
            }
        }
    }

    write!(out, "{}", cursor::Goto(1, size.1)).unwrap();

    write_svg("frame_lines.svg", lines); // For debugging line accuracy

    Ok(())
}