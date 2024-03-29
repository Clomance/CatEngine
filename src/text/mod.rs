// Определения \\

// Глиф (glyph) - здесь изображение (текстура) символа.

// Ascender (выносной элемент) - в типографике часть строчной буквы,
// выходящая за пределы линии строчных знаков или базовой линии шрифта.
// Здесь расстояние от строки до верхней границы этой части.
// Примеры: загагулина у буквы f, палочка у букв h и b, крышка у буквы А.

// Размер шрифта (font size) - здесь высота,
// под которую выравниваются все текстуры символов.
// Таким будет размер самого большого глифа при рендеринге.
// Но определить точный размер этого глифа не всегда удобно, поэтому
// чаще всего все символы будут чуть меньше. Используйте функцию `text_size`
// для точного определения размеров текста.

// Хранилище \\

// Все символы хранятся вместе с глифами в хранилище (`GlyphCache`).
// Для каждого символа создаётся текстура и в неё загружается глиф.
// Поиск глифов по символам выполняется с помощью функций `HashMap`.

mod glyph;
pub use glyph::TexturedGlyph;

mod glyph_cache;
pub use glyph_cache::GlyphCache;

mod font;
pub use font::{
    FontOwner,
    FaceWrapper,
    CachedFont,
};

// re-export
pub use ttf_parser;

#[derive(Clone,Copy,Debug)]
pub struct Scale{
    pub horizontal:f32,
    pub vertical:f32,
}

impl Scale{
    pub fn new(h:f32,v:f32)->Scale{
        Self{
            horizontal:h,
            vertical:v
        }
    }
}

impl std::ops::Div for Scale{
    type Output=Scale;
    fn div(self,rhs:Scale)->Scale{
        Self{
            horizontal:self.horizontal/rhs.horizontal,
            vertical:self.vertical/rhs.vertical,
        }
    }
}

impl std::ops::Mul for Scale{
    type Output=Scale;
    fn mul(self,rhs:Scale)->Scale{
        Self{
            horizontal:self.horizontal*rhs.horizontal,
            vertical:self.vertical*rhs.vertical,
        }
    }
}