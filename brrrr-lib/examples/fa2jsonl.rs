// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::io::stdout;

use brrrr_lib::json_writer::fa2jsonl;

fn main() {
    let example_input = b">A\nATCG\n>B\nGCTA" as &[u8];
    fa2jsonl(example_input, &mut stdout()).expect("Error... :(");
}
