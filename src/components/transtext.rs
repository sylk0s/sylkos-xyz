use dioxus::prelude::*;

// I don't actually need all these iters, I'm just having fun with types :)

#[derive(Clone, Copy)]
enum TransColor {
    BLUE,
    PINK,
    WHITE
}

struct TransIter {
    color: TransColor,
}

impl TransIter {
    fn new() -> Self {
        TransIter {
            color: TransColor::WHITE,
        }
    }
}

impl Iterator for TransIter {
    type Item = TransColor;

    fn next(&mut self) -> Option<Self::Item> {

        self.color = match self.color {
            TransColor::BLUE => TransColor::PINK,
            TransColor::PINK => TransColor::WHITE,
            TransColor::WHITE => TransColor::BLUE,
        };

        Some(self.color)
    }
}

// struct ZipIter<'a, 'b, T, U> {
//     a: &'a mut dyn Iterator<Item = T>,
//     b: &'b mut dyn Iterator<Item = U>,
// }

// impl<'a, 'b, T, U> ZipIter<'_, '_, T, U> {
//     fn from(a: &'a mut dyn Iterator<Item = T>, b: &'b mut dyn Iterator<Item = U>) -> Self {
//         ZipIter { a, b }
//     }
// }

// impl<'a, 'b, T, U> Iterator for ZipIter<'_, '_, T, U> {
//     type Item = (T, U);

//     fn next(&mut self) -> Option<Self::Item> {
//         let aaa = self.a.next()?;
//         let bbb = self.b.next()?;

//         Some((aaa, bbb))
//     }
// }

#[component]
pub fn TransText(text: String) -> Element {
    let mut color = TransIter::new();
    rsx! {
        div {
            class: "flex flex-row justify-center", // TODO: Fix this thing
            for chr in text.chars() {
                div {
                    class: {match color.next() {
                            Some(TransColor::BLUE) => "text-blue",
                            Some(TransColor::PINK) => "text-pink",
                            Some(TransColor::WHITE) => "text-text",
                            None => "",
                        }
                    },
                    {chr.to_string()}
                }
            }
        }
    }
}