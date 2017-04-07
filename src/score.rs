use draw::TextDrawer;
use std::cmp::Ordering;

pub const OFFSET: i32 = 100;

#[derive(RustcDecodable, RustcEncodable, Eq, PartialEq, Clone)]
pub struct Score {
    pub value: u32,
    pub name: String,
}

impl Score {
    pub fn new(value: u32, name: String) -> Self {
        Score {
            value: value,
            name: name,
        }
    }

    pub fn draw<'a, 'b>(&self, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b> {
        let name = if self.name.is_empty() {
            " "
        } else {
            &self.name
        };

        text.offset(-OFFSET, 0)
            .draw(name)
            .offset(OFFSET * 2, 0)
            .draw(&self.value.to_string())
            .under()
            .offset(-OFFSET, 10)
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value).reverse()
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
