use heapless;



type Letter<'a> = [&'a [u8]; 8];


fn to_letter<'a>(letter: [[u8; 5]; 8]) -> Letter<'a> {
    // letter
    todo!();
}


use heapless::*;
use heapless::consts::*;

fn get_alphabet<'a>() -> FnvIndexMap<char, Letter<'a>, U4> {
    let mut alphabet: FnvIndexMap<char, Letter, U4> = heapless::FnvIndexMap::new();

    alphabet.insert('H', [
        &[0xF, 0x0, 0x0, 0x0, 0xF][..],
        &[0xF, 0x0, 0x0, 0x0, 0xF][..],
        &[0xF, 0x0, 0x0, 0x0, 0xF][..],
        &[0xF, 0xF, 0xF, 0xF, 0xF][..],
        &[0xF, 0x0, 0x0, 0x0, 0xF][..],
        &[0xF, 0x0, 0x0, 0x0, 0xF][..],
        &[0xF, 0x0, 0x0, 0x0, 0xF][..],
        &[0x0, 0x0, 0x0, 0x0, 0x0][..],
    ]);

    alphabet.insert('I', [
        &[0xF, 0xF, 0xF, 0xF, 0xF][..],
        &[0x0, 0x0, 0xF, 0x0, 0x0][..],
        &[0x0, 0x0, 0xF, 0x0, 0x0][..],
        &[0x0, 0x0, 0xF, 0x0, 0x0][..],
        &[0x0, 0x0, 0xF, 0x0, 0x0][..],
        &[0x0, 0x0, 0xF, 0x0, 0x0][..],
        &[0xF, 0xF, 0xF, 0xF, 0xF][..],
        &[0x0, 0x0, 0x0, 0x0, 0x0][..],
    ]);

    alphabet.insert('\u{1f}', [
        &[0xF][..],
        &[0xF][..],
        &[0xF][..],
        &[0xF][..],
        &[0xF][..],
        &[0xF][..],
        &[0xF][..],
        &[0xF][..],
    ]);

    alphabet
}

pub fn spell(word: &str, buffer: &mut [&mut [u8]; 8]) -> Result<(), &'static str> {
    let alphabet = get_alphabet();

    let mut offset = 0;
    for chr in word.chars() {
        let image = alphabet.get(&chr).ok_or("Unknown letter")?;
        // make sure we have enough room in the output
        if offset + image[0].len() >= buffer[0].len() {
            return Err("buffer not long enough");
        }

        for row in 0..buffer.len() {
            for &pixel in image[row] {
                buffer[row][offset] = pixel;
                offset += 1;
            }
        }
    }

    Ok(())
}