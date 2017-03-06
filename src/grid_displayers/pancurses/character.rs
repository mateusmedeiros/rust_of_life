use pancurses::{ chtype, ToChtype };

pub struct Character {
    pub color: Option<chtype>,
    pub character: Option<char>,
    pub attributes: Option<&'static [chtype]>
}

impl<'a> ToChtype for &'a Character {
    fn to_chtype(&self) -> chtype {
        let color = self.color.unwrap_or(0 as chtype);
        let character = self.character.unwrap_or(0 as char);
        let empty_attributes: &'static [chtype] = &[];
        let attributes = self.attributes.unwrap_or(empty_attributes);
        let attributes_or = attributes.iter().fold(0 as chtype, |ch, &attribute| {
            ch | attribute
        });

        color | character as chtype | attributes_or
    }
}
