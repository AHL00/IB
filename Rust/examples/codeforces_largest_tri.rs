// n number of points
// n is a coordinate
// largest possible triangle

use painless_input::input;

fn main() {
    let n = input::<usize>("Enter n: ");
    println!();

    let mut points: Vec<(i128, i128)> = Vec::with_capacity(n);

    for _ in 0..n {
        let (x, y) = (
            rand::random::<i128>() % 1000,
            rand::random::<i128>() % 1000,
        );

        points.push((x as i128, y as i128));
    }

    let start = std::time::Instant::now();

    let mut max_area = 0;

    let furthest_from_origin = points.iter().max_by_key(|(x, y)| x.pow(2) + y.pow(2)).unwrap();

    println!("Furthest from origin: {:?}", furthest_from_origin);

    let furthest_from_first = points.iter().max_by_key(|(x, y)| (x - furthest_from_origin.0).pow(2) + (y - furthest_from_origin.1).pow(2)).unwrap();
    
    println!("Furthest from first: {:?}", furthest_from_first);

    // Now we have two points, find max area
    for point in points.iter() {
        let area = area(*point, *furthest_from_origin, *furthest_from_first);

        if area > max_area {
            max_area = area;
        }
    }

    let end = std::time::Instant::now();

    println!("Elapsed time: {:?}", end - start);

    println!("Points: {:?}", points);

    println!("Max area: {}", max_area);

    // Brute force
    let start = std::time::Instant::now();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                let area = area(points[i], points[j], points[k]);

                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    let end = std::time::Instant::now();
    
    println!("Elapsed time: {:?}", end - start);

    println!("Max area: {}", max_area);
}


fn area(a: (i128, i128), b: (i128, i128), c: (i128, i128)) -> i128 {
    let (x1, y1) = (a.0, a.1);
    let (x2, y2) = (b.0, b.1);
    let (x3, y3) = (c.0, c.1);

    let area = (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs() / 2;

    area
}