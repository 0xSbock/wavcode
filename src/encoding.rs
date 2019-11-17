use std::i16;

// code 39 has two types of bars and two types of spaces
#[derive(Copy, Clone, Debug)]
pub enum Bar {
    WideBar,
    ThinBar,
    WideSpace,
    ThinSpace,
}

// each type of space and bar has a fixed amount of samples in the outgoing audio
pub fn generate_samples(bar: &Bar, sample_rate: usize) -> Vec<i16> {
    match bar {
        // the bars have to oscialte to generate a visible bar in the waveform
        Bar::WideBar => (0..sample_rate)
            .map(|x| if x % 2 == 0 { i16::MAX } else { 0 })
            .collect(),
        Bar::ThinBar => (0..sample_rate / 3)
            .map(|x| if x % 2 == 0 { i16::MAX } else { 0 })
            .collect(),
        // for the spaces we can just write a waveform with 0 amplitude to the file
        Bar::WideSpace => vec![0; sample_rate],
        Bar::ThinSpace => vec![0; sample_rate / 3],
    }
}

// since only the encoding is kept in the lookup table of encode, all the thin
// spaces have to be added to every encoded character
pub fn add_thin_space_padding(encoding: Vec<Bar>) -> Vec<Bar> {
    let mut tmp = encoding.windows(2).map(|bars| {
        match bars {
            [Bar::ThinBar, Bar::ThinBar] => vec![Bar::ThinBar, Bar::ThinSpace],
            [Bar::ThinBar, Bar::WideBar] => vec![Bar::ThinBar, Bar::ThinSpace],
            [Bar::ThinBar, Bar::WideSpace] => vec![Bar::ThinBar],
            [Bar::WideBar, Bar::WideBar] => vec![Bar::WideBar, Bar::ThinSpace],
            [Bar::WideBar, Bar::ThinBar] => vec![Bar::WideBar, Bar::ThinSpace],
            [Bar::WideBar, Bar::WideSpace] => vec![Bar::WideBar],
            [Bar::WideSpace, Bar::ThinBar] => vec![Bar::WideSpace],
            [Bar::WideSpace, Bar::WideBar] => vec![Bar::WideSpace],
            _ => vec![]

        }
    }).flatten().collect::<Vec<Bar>>();
    // because of the way we add the thin bars with the windows iterator,
    // the last element of the input vec has to be added manually
    let last_elem = *encoding.last().unwrap();
    tmp.push(last_elem);
    // add a last thin space to seperate characters
    tmp.push(Bar::ThinSpace);
    tmp
}

// encode a char into a vector of bars and spaces (barcode)
pub fn encode(character: &char) -> Vec<Bar> {
    // lookup table for each supported character
    let unpadded = match character {
        '1' => vec![
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
        ],
        '2' => vec![
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
        ],
        '3' => vec![
            Bar::WideBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        '4' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideBar,
        ],
        '5' => vec![
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        '6' => vec![
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        '7' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideBar,
        ],
        '8' => vec![
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
        ],
        '9' => vec![
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
        ],
        '0' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::WideBar,
            Bar::ThinBar,
        ],
        'A' => vec![
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideBar,
        ],
        'B' => vec![
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideBar,
        ],
        'C' => vec![
            Bar::WideBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        'D' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideBar,
        ],
        'E' => vec![
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        'F' => vec![
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        'G' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::WideBar,
        ],
        'H' => vec![
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::ThinBar,
        ],
        'I' => vec![
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::ThinBar,
        ],
        'J' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::ThinBar,
        ],
        'K' => vec![
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
        ],
        'L' => vec![
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
        ],
        'M' => vec![
            Bar::WideBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
        ],
        'N' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
        ],
        'O' => vec![
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
        ],
        'P' => vec![
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
        ],
        'Q' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::WideBar,
        ],
        'R' => vec![
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
        ],
        'S' => vec![
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
        ],
        'T' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
        ],
        'U' => vec![
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
        ],
        'V' => vec![
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
        ],
        'W' => vec![
            Bar::WideBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        'X' => vec![
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideBar,
        ],
        'Y' => vec![
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        'Z' => vec![
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        '-' => vec![
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideBar,
        ],
        '.' => vec![
            Bar::WideBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
        ],
        ' ' => vec![
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::WideBar,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::ThinBar,
        ],
        '*' => vec![
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideBar,
            Bar::WideBar,
            Bar::ThinBar,
        ],
        '$' => vec![
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
        ],
        '/' => vec![
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
        ],
        '+' => vec![
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
        ],
        '%' => vec![
            Bar::ThinBar,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
            Bar::WideSpace,
            Bar::ThinBar,
        ],
        _ => vec![],
    };
    // before returning the vec, pad it with thin spaces
    add_thin_space_padding(unpadded)
}
