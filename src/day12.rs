use std::collections::VecDeque;

fn read_file() -> String {
    std::fs::read_to_string("src/day_12_input.txt").expect("The file should be there")
}
#[derive(Debug)]
struct Shape {
    rows: Vec<Vec<bool>>,
}
#[derive(Debug)]
struct Region{
    width : usize,
    height : usize,
    shape_amounts : Vec<usize>
}

fn parse_file(src: String) -> (Vec<Shape>, Vec<Region>) {
    let mut lines = src.lines().peekable();

    let shapes = {
        let mut shapes = Vec::new();
        while let Some(line) = lines.peek()
            &&  let Some(_) = line.trim().strip_suffix(":")
        {
            lines.next();
            shapes.push(Shape {
                rows: lines
                    .by_ref()
                    .map(str::trim)
                    .take_while(|line| line.ends_with(&['.', '#']))
                    .map(|line| {
                        line.chars()
                            .map(|c| match c {
                                '#' => true,
                                '.' => false,
                                _ => unreachable!(),
                            })
                            .collect()
                    })
                    .collect(),
            });
        }
        shapes
    };

    let regions = {
        lines.skip_while(|line|{ 
            line.trim().is_empty()
        }).map(|line|{
            let mut iter = line.split(':');
            let mut width_and_height = iter.next().expect("There should be a left side").split('x');
            let width = width_and_height.next().expect("There should be a width").parse::<usize>().expect("It should be a numeric width");
            let height = width_and_height.next().expect("There should be a height").parse::<usize>().expect("It should be a numeric height");
            let shapes_to_fit = iter.next().expect("There should be a left side").split_whitespace();
            let shape_amounts = shapes_to_fit.map(|shape| shape.parse::<usize>().expect("It should be an index")).collect::<Vec<_>>();
            Region{
                width,
                height,
                shape_amounts
            }
        }).collect()
    };
    (shapes, regions)
}

fn does_region_fit_presents(region: Region, shapes: &[Shape]) -> bool{
    let area = region.height * region.width;
    region.shape_amounts.iter().copied().enumerate().map(|(shape,count)|{
        9 * count
    }).sum::<usize>() <= area
}
pub fn regions_that_fit() -> usize {
    let src = read_file();
    let (shapes, regions) = parse_file(src);
    regions.into_iter().map(|region| does_region_fit_presents(region, &shapes) as usize).sum()
}
