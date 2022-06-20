// AoC 2021 day 20
//
// The trick to this one is to notice that decoder[0] = 1 and decoder[511] = 0.  This means that
// on the first application of the decoder the entire universe outside of the image will light up
// then go dark again on the second application.

// function that returns the number of lit pixels in the passed image
fn count_lit_pixels(image: &[Vec<char>]) -> usize {
    image
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| if *c == '#' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

// function that puts a layer dark pixels around the passed image
fn buffer_image(im: &Vec<Vec<char>>, buf: usize) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];
    let linelen = im[0].len() + buf * 2;
    let imlen = im.len() + buf * 2;

    for l in 0..imlen {
        if l < buf || l >= imlen - buf {
            res.push((0..linelen).map(|_| '.').collect());
        } else {
            let mut nv: Vec<char> = vec![];
            for c in 0..linelen {
                if c < buf || c >= linelen - buf {
                    nv.push('.');
                } else {
                    nv.push(im[l - buf][c - buf]);
                }
            }
            res.push(nv);
        }
    }

    res
}

// function that uses the passed decoder array to enhance and return the passed image
// offscreen is the value for pixels that are outside the image borders
fn enhance(im: &Vec<Vec<char>>, dec: &[char], offscreen: usize) -> Result<Vec<Vec<char>>, String> {
    let mut res: Vec<Vec<char>> = vec![];

    for l in 0..im.len() {
        let mut nv: Vec<char> = vec![];
        for c in 0..im[l].len() {
            let mut idx = 0;
            for ll in 0..=2 {
                if ll == 0 && l == 0 || ll == 2 && l == im.len() - 1 {
                    idx = idx * 8 + offscreen * 4 + offscreen * 2 + offscreen;
                } else {
                    for cc in 0..=2 {
                        if cc == 0 && c == 0 || cc == 2 && c == im[l].len() - 1 {
                            idx = idx * 2 + offscreen;
                        } else {
                            match im[l + ll - 1][c + cc - 1] {
                                '.' => idx *= 2,
                                '#' => idx = idx * 2 + 1,
                                _ => return Err("bad image character found".to_string()),
                            }
                        }
                    }
                }
            }
            nv.push(dec[idx]);
        }
        res.push(nv);
    }

    Ok(res)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut image: Vec<Vec<char>> = vec![];
    let mut need_decoder = true;
    let mut decoder: Vec<char> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        let s = inputstr.trim();
        if need_decoder {
            decoder = s.chars().collect();
            need_decoder = false;
        } else if !s.is_empty() {
            image.push(s.chars().collect());
        }
        inputstr.clear();
    }
    image = buffer_image(&image, 50);
    image = enhance(&image, &decoder, 0)?;
    image = enhance(&image, &decoder, 1)?;

    println!("aoc20a: {}", count_lit_pixels(&image));

    for _ in 0..24 {
        image = enhance(&image, &decoder, 0)?;
        image = enhance(&image, &decoder, 1)?;
    }

    println!("aoc20b: {}", count_lit_pixels(&image));

    Ok(())
}
