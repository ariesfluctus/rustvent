use std::cmp::min;

fn main() {
    // let input = str::from_utf8(include_bytes!("2015-2.in")).expect("input is not valid utf-8");
    let input = include_str!("2015-2.in");

    let (paper, ribbon) = input.lines().fold((0, 0), |acc, line| {
        let size: Vec<u32> = line
            .split('x')
            .map(|x| x.parse().expect("malformed input"))
            .collect();
        let mut smallest_face = size.clone();
        smallest_face.sort();
        let (l, w, h) = (size[0], size[1], size[2]);
        let (s1, s2) = (smallest_face[0], smallest_face[1]);
        let (lw, wh, hl) = (l * w, w * h, h * l);
        (
            acc.0 + (2 * lw) + (2 * wh) + (2 * hl) + min(lw, min(wh, hl)),
            acc.1 + (s1 + s1 + s2 + s2) + (l * w * h),
        )
    });

    println!("{} {}", paper, ribbon);
}
