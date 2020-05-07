use xmltree::Element;
use std::fs;

const SIZE: u16 = 512;
const TREE_SRC: &str = "./linetree.xml";

#[derive(Clone, Debug)]
struct Line {
    character: String,
    x: u16,
    y: u16,
    z: u16,
    length: u16,
}

struct Window {
    x: u16,
    y: u16,
    z: u16,
    half_dim: u16,
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
            x, y, z,
            half_dim: dim,
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
                character: unescaped_char,
                x: line_x,
                y: line_y,
                z: line_z,
                length: line_len,
            });
        }

        let children: Vec<OctNode> = el.children.into_iter().map(|child_el| OctNode::new(child_el)).collect();

        if children.len() == 4 {
            println!("OOGABOOGA");
        }

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
            return lines
        }

        for child in self.children.iter() {
            lines.append(&mut child.search(window.clone()))
        }

        return lines;
    }
}

fn main() -> std::io::Result<()> {
    println!("Hello World!");

    let db_str: String = fs::read_to_string(TREE_SRC).unwrap();
    let doc: Element = Element::parse(db_str.as_bytes()).unwrap();

    let root = OctNode::new(doc);

    // Convert SVG to all straight lines

    // Initialize character grid of size h * w * [(score: u16, char: String)]

    // For each line in svg, search root for nearest neighbor 

    let search_area = Window::new(329, 319, 130, 100);
    let nearest = root.search(&search_area);

    println!("{:?}", nearest); // should be &quot

    //  Score the similarity of the lines by angle (z), length, then distance

    //  Accumulate the score for the overlapping character in the grid 

    // For each char in grid, print first element

    Ok(())
}