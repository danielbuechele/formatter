mod parsers;
mod utils;

use crate::parsers::PARSERS;
use crate::utils::Formatting;

fn insert_if_fits(existing: &mut Vec<Formatting>, item: Formatting) -> bool {
    for formatting in existing.iter_mut() {
        for content_range in formatting.content_ranges.iter() {
            let fits_into =
                content_range.start <= item.range.start && item.range.end() <= content_range.end();

            let is_ovelap = !fits_into
                && item.range.end() > content_range.start
                && item.range.start < content_range.end();

            println!("fits_into: {}, is_ovelap: {}", fits_into, is_ovelap);

            if fits_into {
                return insert_if_fits(&mut formatting.children, item);
            } else if is_ovelap {
                return false;
            }
        }
    }

    existing.push(item);
    true
}

pub fn parse_message(text: &str) -> Vec<Formatting> {
    let mut formattings = PARSERS
        .iter()
        .flat_map(|parse| parse(text))
        .collect::<Vec<utils::Formatting>>();

    // sorting in reverse order
    formattings.sort_by(|a, b| b.range.start.cmp(&a.range.start));

    let mut processed_ranges: Vec<utils::Formatting> = vec![];
    while formattings.len() > 0 {
        let item = formattings.pop();
        insert_if_fits(&mut processed_ranges, item.unwrap());
    }

    processed_ranges
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let text = "test\n```js\nc*od*e1\n```\n\ntest\n```js\ncode2\n```\n\ntest";
        let ranges: Vec<crate::utils::Formatting> = crate::parse_message(text);
        println!("{:#?}", ranges);
    }
}
