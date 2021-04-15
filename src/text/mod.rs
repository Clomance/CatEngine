//! # Рендеринг текста. Text rendering. `feature = "text_graphics"`, `default_features`
//! 
//! Как рендерятся символы:
//! 1. Первый вариант - с помощью обычных шрифтов:
//!    - 
//!     1. Создаётся контур символа
//!     2. Этот символ записывается в массив как изображение
//!     3. Изображение загружается в текстуру
//!     4. Текстура выводится на экран
//! 2. Второй вариант - с помощью хранилищ глифов
//!    - 
//!     1. Глиф достаётся из хранилища
//!     2. Он масштабируется
//!     3. Его текстура выводится на экран
//! ###
//! 
//! How characters are rendering:
//! 1. The first way - with common fonts:
//!    -
//!     1. Building glyph's outline
//!     2. Converting the glyph to an image
//!     3. Loading the image a texture
//!     4. Rendering the texture
//! 2. The second way - with glyph caches
//!    -
//!     1. Taking a glyph from a glyph cache
//!     2. Scaling the glyph
//!     3. Rendering it's texture
//! 
//! ### A simple example
//! ```
//! let mut window=PagedWindow::new(|_,s|{
//!     s.vsync=true;
//!     // Max size for glyph images
//!     s.graphics_base_settings.text.glyph_texture_size=[500,500];
//! }).unwrap();
//! 
//! let font=FontOwner::load("resources/font").unwrap();
//! let wfont=font.face_wrapper();
//! 
//! ... in the cycle:
//!     let base=TextBase::new([300f32,400f32],Scale::new(0.1,0.1),[1f32;4]);
//!     base.draw_str("HelloWorld$?",&wfont,p,g);
//! 
//! ```
//! 
//! ### A glyph cache example
//! ```
//! let mut window=PagedWindow::new(|_,s|{
//!     s.vsync=true;
//! }).unwrap();
//! 
//! let font=FontOwner::load("resources/font").unwrap();
//! 
//! let scale=Scale::new(0.4,0.4);
//! // Creating a new glyph cache for the given characters
//! let glyphs=GlyphCache::new_alphabet(&font,"HelloWorld?",scale,window.display());
//! 
//! ... in the cycle:
//!     let base=TextBase::new([300f32,400f32],Scale::new(0.1,0.1),[1f32;4]);
//!     base.draw_str_glyph_cache("HelloWorld?",&glyphs,p,g);
//! 
//! ```

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
// для точного определения точных размеров текста.

// Хранилище \\

// Все символы хранятся в вместе с глифами в хранилище (`GlyphCache`).
// Для каждого символа создаётся текстура и в неё загружается глиф.
// Поиск глифов по символам выполняется с помощью функций `HashMap`.

use crate::{
    // types
    Colour,
    // structs
    graphics::Graphics,
};

#[cfg(feature="colour_filter")]
use crate::graphics::ColourFilter;

mod glyph;
pub use glyph::*;

mod outline;
pub (crate) use outline::{
    OutlineCurve,
    OutlineCurveBuilder,
    GlyphImageBuilder,
};

pub use outline::{
    Scale,
};

mod glyph_cache;
pub use glyph_cache::{
    GlyphCache,
    // RawGlyphCache,
};

mod font;
pub use font::{
    FontOwner,
    FaceWrapper,
    CachedFont,
};

// re-export
pub use ttf_parser;
pub use ab_glyph_rasterizer;

// / Основа для рендеринга текста.
// / 
// / A base for text rendering.
// pub struct TextBase{
//     pub position:[f32;2],
//     pub scale:Scale,
//     pub colour:Colour,
// }

// impl TextBase{
//     pub const fn new(position:[f32;2],scale:Scale,colour:Colour)->TextBase{
//         Self{
//             scale,
//             colour,
//             position,
//         }
//     }

//     pub const fn zero_position(scale:Scale,colour:Colour)->TextBase{
//         Self{
//             scale,
//             colour,
//             position:[0f32;2],
//         }
//     }

//     #[inline(always)]
//     pub fn set_x(&mut self,x:f32){
//         self.position[0]=x
//     }

//     #[inline(always)]
//     pub fn set_y(&mut self,y:f32){
//         self.position[1]=y
//     }

//     #[inline(always)]
//     pub fn move_to(&mut self,position:[f32;2]){
//         self.position=position
//     }

//     #[inline(always)]
//     pub fn shift_x(&mut self,dx:f32){
//         self.position[0]+=dx
//     }

//     #[inline(always)]
//     pub fn shift_y(&mut self,dy:f32){
//         self.position[1]+=dy
//     }

//     #[inline(always)]
//     pub fn shift(&mut self,dx:f32,dy:f32){
//         self.position[0]+=dx;
//         self.position[1]+=dy;
//     }

//     #[inline(always)]
//     pub fn set_alpha_channel(&mut self,alpha:f32){
//         self.colour[3]=alpha
//     }

//     #[inline(always)]
//     pub fn set_colour(&mut self,colour:Colour){
//         self.colour=colour
//     }
// }





// impl TextBase{
//     /// Выводит символ.
//     /// 
//     /// Draws a character.
//     #[inline(always)]
//     pub fn draw_char<F:Font,S:Surface>(
//         &self,
//         character:char,
//         font:&F,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let glyph=if let Some(glyph)=font.build_raw_glyph(character){
//             glyph
//         }
//         else{
//             if character.is_whitespace(){
//                 return Ok(())
//             }
//             else{
//                 font.build_raw_undefined_glyph()
//             }
//         };

//         // Глиф для рендеринга
//         let outlined=glyph.outlined_glyph(self.scale);

//         // Позиция для глифа для рендеринга
//         let position={
//             let size=outlined.size();
//             let offset=outlined.offset();
//             [
//                 self.position[0],
//                 self.position[1]-offset[1]-size[1] as f32,
//             ]
//         };

//         graphics.draw_glyph(
//             &outlined,
//             self.colour,
//             position,
//             #[cfg(feature="colour_filter")]colour_filter,
//         )
//     }

//     /// Выводит сдвинутый символ.
//     /// 
//     /// Draws a shifted character.
//     #[inline(always)]
//     pub fn draw_shift_char<F:Font,S:Surface>(
//         &self,
//         character:char,
//         shift:[f32;2],
//         font:&F,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let glyph=if let Some(glyph)=font.build_raw_glyph(character){
//             glyph
//         }
//         else{
//             if character.is_whitespace(){
//                 return Ok(())
//             }
//             else{
//                 font.build_raw_undefined_glyph()
//             }
//         };

//         // Глиф для рендеринга
//         let outlined=glyph.outlined_glyph(self.scale);

//         // Позиция для глифа для рендеринга
//         let position={
//             let size=outlined.size();
//             let offset=outlined.offset();
//             [
//                 self.position[0],
//                 self.position[1]-offset[1]-size[1] as f32,
//             ]
//         };

//         graphics.draw_shift_glyph(
//             &outlined,
//             self.colour,
//             position,
//             shift,
//             #[cfg(feature="colour_filter")]colour_filter,
//         )
//     }

//     /// Выводит повёрнутый символ.
//     /// 
//     /// Draws a rotated character.
//     #[inline(always)]
//     pub fn draw_rotate_char<F:Font,S:Surface>(
//         &self,
//         character:char,
//         rotation_center:[f32;2],
//         angle:f32,
//         font:&F,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let glyph=if let Some(glyph)=font.build_raw_glyph(character){
//             glyph
//         }
//         else{
//             if character.is_whitespace(){
//                 return Ok(())
//             }
//             else{
//                 font.build_raw_undefined_glyph()
//             }
//         };

//         // Глиф для рендеринга
//         let outlined=glyph.outlined_glyph(self.scale);

//         // Позиция для глифа для рендеринга
//         let position={
//             let size=outlined.size();
//             let offset=outlined.offset();
//             [
//                 self.position[0],
//                 self.position[1]-offset[1]-size[1] as f32,
//             ]
//         };

//         graphics.draw_rotate_glyph(
//             &outlined,
//             self.colour,
//             position,
//             rotation_center,
//             angle,
//             #[cfg(feature="colour_filter")]colour_filter
//         )
//     }

//     /// Выводит строку.
//     /// 
//     /// Draws a string.
//     pub fn draw_str<F:Font,S:Surface>(
//         &self,
//         s:&str,
//         font:&F,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let mut position=self.position;

//         let mut glyph; // Немасштабированный глиф

//         // Расстояние до следующего глифа для пробела
//         let whitespace_advance=font.whitespace_advance_width(self.scale.horizontal);

//         for character in s.chars(){
//             glyph=if let Some(glyph)=font.build_raw_glyph(character){
//                 glyph
//             }
//             else{
//                 if character==' '{
//                     position[0]+=whitespace_advance;
//                     continue
//                 }

//                 font.build_raw_undefined_glyph()
//             };

//             // Масштабированный глиф
//             let scaled=glyph.scale(self.scale);

//             let rect=scaled.positioned_bounding_box(position);

//             // Расстояние до следующего глифа по горизонтали
//             let advance_width=scaled.advance_width();

//             // Глиф для рендеринга
//             let outlined=scaled.outline();

//             graphics.draw_glyph(
//                 &outlined,
//                 self.colour,
//                 [rect[0],rect[1]],
//                 #[cfg(feature="colour_filter")]colour_filter,
//             )?;

//             position[0]+=advance_width;
//         }

//         Ok(())
//     }

//     /// Выводит сдвинутую строку.
//     /// 
//     /// Draws a shifted string.
//     pub fn draw_shift_str<F:Font,S:Surface>(
//         &self,
//         s:&str,
//         shift:[f32;2],
//         font:&F,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let mut position=self.position;

//         let mut glyph; // Немасштабированный глиф

//         // Расстояние до следующего глифа для пробела
//         let whitespace_advance=font.whitespace_advance_width(self.scale.horizontal);

//         for character in s.chars(){
//             glyph=if let Some(glyph)=font.build_raw_glyph(character){
//                 glyph
//             }
//             else{
//                 if character==' '{
//                     position[0]+=whitespace_advance;
//                     continue
//                 }

//                 font.build_raw_undefined_glyph()
//             };

//             // Масштабированный глиф
//             let scaled=glyph.scale(self.scale);

//             let rect=scaled.positioned_bounding_box(position);

//             // Расстояние до следующего глифа по горизонтали
//             let advance_width=scaled.advance_width();

//             // Глиф для рендеринга
//             let outlined=scaled.outline();

//             graphics.draw_shift_glyph(
//                 &outlined,
//                 self.colour,
//                 [rect[0],rect[1]],
//                 shift,
//                 #[cfg(feature="colour_filter")]colour_filter,
//             )?;

//             position[0]+=advance_width;
//         }

//         Ok(())
//     }

//     /// Выводит повёрнутую строку.
//     /// 
//     /// Draws a rotated string.
//     pub fn draw_rotate_str<F:Font,S:Surface>(
//         &self,
//         s:&str,
//         rotation_center:[f32;2],
//         angle:f32,
//         font:&F,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let mut position=self.position;

//         let mut glyph; // Немасштабированный глиф

//         // Расстояние до следующего глифа для пробела
//         let whitespace_advance=font.whitespace_advance_width(self.scale.horizontal);

//         for character in s.chars(){
//             glyph=if let Some(glyph)=font.build_raw_glyph(character){
//                 glyph
//             }
//             else{
//                 if character==' '{
//                     position[0]+=whitespace_advance;
//                     continue
//                 }

//                 font.build_raw_undefined_glyph()
//             };

//             // Масштабированный глиф
//             let scaled=glyph.scale(self.scale);

//             let rect=scaled.positioned_bounding_box(position);

//             // Расстояние до следующего глифа
//             let advance_width=scaled.advance_width();

//             // Глиф для рендеринга
//             let outlined=scaled.outline();

//             graphics.draw_rotate_glyph(
//                 &outlined,
//                 self.colour,
//                 [rect[0],rect[1]],
//                 rotation_center,
//                 angle,
//                 #[cfg(feature="colour_filter")]colour_filter
//             )?;

//             position[0]+=advance_width;
//         }

//         Ok(())
//     }
// }





// impl TextBase{
//     /// Строит и выводит один символ.
//     /// 
//     /// Берёт соответствующий глиф из данного хранилища.
//     /// 
//     /// Builds and draws a character.
//     /// 
//     /// Takes a corresponding glyph from the given cache.
//     #[inline(always)]
//     pub fn draw_char_glyph_cache<C:RawGlyphCache,S:Surface>(
//         &self,
//         character:char,
//         glyph_cache:&C,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let glyph=if let Some(glyph)=glyph_cache.scaled_glyph(character,self.scale){
//             glyph
//         }
//         else{
//             if character.is_whitespace(){
//                 return Ok(())
//             }
//             else{
//                 glyph_cache.scaled_undefined_glyph(self.scale)
//             }
//         };

//         let rect=glyph.positioned_bounding_box(self.position);

//         // Создание глифа для рендеринга
//         let textured=TexturedGlyph::raw(glyph.data(),[rect[2],rect[3]]);

//         graphics.draw_glyph_cache(
//             &textured,
//             self.colour,
//             [rect[0],rect[1]],
//             #[cfg(feature="colour_filter")]colour_filter
//         )
//     }

//     /// Выводит сдвинутый символ.
//     /// 
//     /// Берёт соответствующий глиф из данного хранилища.
//     /// 
//     /// Draws a shifted character.
//     /// 
//     /// Takes a corresponding glyph from the given cache.
//     #[inline(always)]
//     pub fn draw_shift_char_glyph_cache<C:RawGlyphCache,S:Surface>(
//         &self,
//         character:char,
//         shift:[f32;2],
//         glyph_cache:&C,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let glyph=if let Some(glyph)=glyph_cache.scaled_glyph(character,self.scale){
//             glyph
//         }
//         else{
//             if character.is_whitespace(){
//                 return Ok(())
//             }
//             else{
//                 glyph_cache.scaled_undefined_glyph(self.scale)
//             }
//         };

//         let rect=glyph.positioned_bounding_box(self.position);

//         // Создание глифа для рендеринга
//         let textured=TexturedGlyph::raw(glyph.data(),[rect[2],rect[3]]);

//         graphics.draw_shift_glyph_cache(
//             &textured,
//             self.colour,
//             [rect[0],rect[1]],
//             shift,
//             #[cfg(feature="colour_filter")]colour_filter
//         )
//     }

//     /// Выводит символ.
//     /// 
//     /// Берёт соответствующий глиф из данного хранилища.
//     /// 
//     /// Draws a character.
//     /// 
//     /// Takes a corresponding glyph from the given cache.
//     #[inline(always)]
//     pub fn draw_rotate_char_glyph_cache<C:RawGlyphCache,S:Surface>(
//         &self,
//         character:char,
//         rotation_center:[f32;2],
//         angle:f32,
//         glyph_cache:&C,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let glyph=if let Some(glyph)=glyph_cache.scaled_glyph(character,self.scale){
//             glyph
//         }
//         else{
//             if character.is_whitespace(){
//                 return Ok(())
//             }
//             else{
//                 glyph_cache.scaled_undefined_glyph(self.scale)
//             }
//         };

//         let rect=glyph.positioned_bounding_box(self.position);

//         // Создание глифа для рендеринга
//         let textured=TexturedGlyph::raw(glyph.data(),[rect[2],rect[3]]);

//         graphics.draw_rotate_glyph_cache(
//             &textured,
//             self.colour,
//             [rect[0],rect[1]],
//             rotation_center,
//             angle,
//             #[cfg(feature="colour_filter")]colour_filter
//         )
//     }

//     /// Выводит строку.
//     /// 
//     /// Берёт соответствующие глифы из данного хранилища.
//     /// 
//     /// Draws a string.
//     /// 
//     /// Takes corresponding glyphs from the given cache.
//     pub fn draw_str_glyph_cache<C:RawGlyphCache,S:Surface>(
//         &self,
//         s:&str,
//         glyph_cache:&C,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let mut position=self.position;

//         let mut glyph;

//         for character in s.chars(){
//             glyph=if let Some(glyph)=glyph_cache.scaled_glyph(character,self.scale){
//                 glyph
//             }
//             else{
//                 if character==' '{
//                     position[0]+=glyph_cache.whitespace_advance_width(self.scale.horizontal);
//                     continue
//                 }

//                 glyph_cache.scaled_undefined_glyph(self.scale)
//             };

//             let rect=glyph.positioned_bounding_box(position);

//             // Создание глифа для рендеринга
//             let textured=TexturedGlyph::raw(glyph.data(),[rect[2],rect[3]]);

//             graphics.draw_glyph_cache(
//                 &textured,
//                 self.colour,
//                 [rect[0],rect[1]],
//                 #[cfg(feature="colour_filter")]colour_filter
//             )?;

//             position[0]+=glyph.advance_width();
//         }

//         Ok(())
//     }

//     /// Выводит сдвинутую строку.
//     /// 
//     /// Берёт соответствующие глифы из данного хранилища.
//     /// 
//     /// Draws a shifted string.
//     /// 
//     /// Takes corresponding glyphs from the given cache.
//     pub fn draw_shift_str_glyph_cache<C:RawGlyphCache,S:Surface>(
//         &self,
//         s:&str,
//         shift:[f32;2],
//         glyph_cache:&C,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let mut position=self.position;

//         let mut glyph; // Мастабированный глиф

//         for character in s.chars(){
//             glyph=if let Some(glyph)=glyph_cache.scaled_glyph(character,self.scale){
//                 glyph
//             }
//             else{
//                 if character==' '{
//                     position[0]+=glyph_cache.whitespace_advance_width(self.scale.horizontal);
//                     continue
//                 }

//                 glyph_cache.scaled_undefined_glyph(self.scale)
//             };

//             let rect=glyph.positioned_bounding_box(position);

//             // Создание глифа для рендеринга
//             let textured=TexturedGlyph::raw(glyph.data(),[rect[2],rect[3]]);

//             graphics.draw_shift_glyph_cache(
//                 &textured,
//                 self.colour,
//                 [rect[0],rect[1]],
//                 shift,
//                 #[cfg(feature="colour_filter")]colour_filter
//             )?;

//             position[0]+=glyph.advance_width();
//         }

//         Ok(())
//     }

    

//     /// Выводит повёрнутую строку.
//     /// 
//     /// Берёт соответствующие глифы из данного хранилища.
//     /// 
//     /// Draws a rotated string.
//     /// 
//     /// Takes corresponding glyphs from the given cache.
//     pub fn draw_rotate_str_glyph_cache<C:RawGlyphCache,S:Surface>(
//         &self,
//         s:&str,
//         rotation_center:[f32;2],
//         angle:f32,
//         glyph_cache:&C,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<(),DrawError>{
//         let mut position=self.position;

//         let mut glyph; // Мастабированный глиф

//         for character in s.chars(){
//             glyph=if let Some(glyph)=glyph_cache.scaled_glyph(character,self.scale){
//                 glyph
//             }
//             else{
//                 if character==' '{
//                     position[0]+=glyph_cache.whitespace_advance_width(self.scale.horizontal);
//                     continue
//                 }

//                 glyph_cache.scaled_undefined_glyph(self.scale)
//             };

//             let rect=glyph.positioned_bounding_box(position);

//             // Создание глифа для рендеринга
//             let textured=TexturedGlyph::raw(glyph.data(),[rect[2],rect[3]]);

//             graphics.draw_rotate_glyph_cache(
//                 &textured,
//                 self.colour,
//                 [rect[0],rect[1]],
//                 rotation_center,
//                 angle,
//                 #[cfg(feature="colour_filter")]colour_filter
//             )?;

//             position[0]+=glyph.advance_width();
//         }

//         Ok(())
//     }

//     /// Выводит часть строки.
//     /// Если текст выведен полностью, возвращает `true`.
//     /// 
//     /// Берёт соответствующие глифы из данного хранилища.
//     /// 
//     /// Draws a part of a string.
//     /// Returns `true`, if the whole string is drawn.
//     /// 
//     /// Takes corresponding glyphs from the given cache.
//     pub fn draw_str_part_glyph_cache<C:RawGlyphCache,S:Surface>(
//         &self,
//         s:&str,
//         chars:usize,
//         glyph_cache:&C,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<bool,DrawError>{
//         let mut whole=true; // Флаг вывода всего текста

//         let mut position=self.position;

//         let mut glyph; // Мастабированный глиф

//         for (i,character) in s.chars().enumerate(){
//             // Выход из цикла при достижении лимита символов
//             if i==chars{
//                 whole=false;
//                 break
//             }

//             glyph=if let Some(glyph)=glyph_cache.scaled_glyph(character,self.scale){
//                 glyph
//             }
//             else{
//                 if character==' '{
//                     position[0]+=glyph_cache.whitespace_advance_width(self.scale.horizontal);
//                     continue
//                 }

//                 glyph_cache.scaled_undefined_glyph(self.scale)
//             };

//             let rect=glyph.positioned_bounding_box(position);

//             // Создание глифа для рендеринга
//             let textured=TexturedGlyph::raw(glyph.data(),[rect[2],rect[3]]);

//             graphics.draw_glyph_cache(
//                 &textured,
//                 self.colour,
//                 [rect[0],rect[1]],
//                 #[cfg(feature="colour_filter")]colour_filter
//             )?;

//             position[0]+=glyph.advance_width();
//         }

//         Ok(whole)
//     }

//     /// Выводит часть строки.
//     /// Если текст выведен полностью, возвращает `true`.
//     /// 
//     /// Берёт соответствующие глифы из данного хранилища.
//     /// 
//     /// Draws a part of a string.
//     /// Returns `true`, if the whole string is drawn.
//     /// 
//     /// Takes corresponding glyphs from the given cache.
//     pub fn draw_shift_str_part_glyph_cache<C:RawGlyphCache,S:Surface>(
//         &self,
//         s:&str,
//         chars:usize,
//         shift:[f32;2],
//         glyph_cache:&C,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<bool,DrawError>{
//         let mut whole=true; // Флаг вывода всего текста

//         let mut position=self.position;

//         let mut glyph; // Мастабированный глиф

//         for (i,character) in s.chars().enumerate(){
//             // Выход из цикла при достижении лимита символов
//             if i==chars{
//                 whole=false;
//                 break
//             }

//             glyph=if let Some(glyph)=glyph_cache.scaled_glyph(character,self.scale){
//                 glyph
//             }
//             else{
//                 if character==' '{
//                     position[0]+=glyph_cache.whitespace_advance_width(self.scale.horizontal);
//                     continue
//                 }

//                 glyph_cache.scaled_undefined_glyph(self.scale)
//             };

//             let rect=glyph.positioned_bounding_box(position);

//             // Создание глифа для рендеринга
//             let textured=TexturedGlyph::raw(glyph.data(),[rect[2],rect[3]]);

//             graphics.draw_shift_glyph_cache(
//                 &textured,
//                 self.colour,
//                 [rect[0],rect[1]],
//                 shift,
//                 #[cfg(feature="colour_filter")]colour_filter
//             )?;

//             position[0]+=glyph.advance_width();
//         }

//         Ok(whole)
//     }

//     /// Выводит часть повёрнутой строки.
//     /// Если текст выведен полностью, возвращает true.
//     /// 
//     /// Берёт соответствующие глифы из данного хранилища.
//     /// 
//     /// Draws a part of a rotated string.
//     /// Returns true, if the whole string is drawn.
//     /// 
//     /// Takes corresponding glyphs from the given cache.
//     pub fn draw_rotate_str_part_glyph_cache<C:RawGlyphCache,S:Surface>(
//         &self,
//         s:&str,
//         chars:usize,
//         rotation_center:[f32;2],
//         angle:f32,
//         glyph_cache:&C,
//         #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
//         graphics:&mut Graphics<S>
//     )->Result<bool,DrawError>{
//         let mut whole=true; // Флаг вывода всего текста

//         let mut position=self.position;

//         let mut glyph;

//         for (i,character) in s.chars().enumerate(){
//             // Выход из цикла при достижении лимита символов
//             if i==chars{
//                 whole=false;
//                 break
//             }

//             glyph=if let Some(glyph)=glyph_cache.scaled_glyph(character,self.scale){
//                 glyph
//             }
//             else{
//                 if character==' '{
//                     position[0]+=glyph_cache.whitespace_advance_width(self.scale.horizontal);
//                     continue
//                 }

//                 glyph_cache.scaled_undefined_glyph(self.scale)
//             };

//             let rect=glyph.positioned_bounding_box(position);

//             // Создание глифа для рендеринга
//             let textured=TexturedGlyph::raw(glyph.data(),[rect[2],rect[3]]);

//             graphics.draw_rotate_glyph_cache(
//                 &textured,
//                 self.colour,
//                 [rect[0],rect[1]],
//                 rotation_center,
//                 angle,
//                 #[cfg(feature="colour_filter")]colour_filter
//             )?;

//             position[0]+=glyph.advance_width();
//         }

//         Ok(whole)
//     }
// }