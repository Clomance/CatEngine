use crate::{
    Colour,
    text::{
        OutlinedGlyph,
        TexturedGlyph,
    },
    texture::Texture,
};

#[cfg(feature="colour_filter")]
use crate::graphics::ColourFilter;

use super::two_dimensions::{
    Vertex2D,
    TexturedVertex2D,
    Graphics2D
};

#[cfg(feature="3D")]
use super::three_dimensions::Graphics3D;

use super::{DependentObject,ObjectType,DrawType};

use glium::{
    Frame,
    DrawParameters,
    DrawError,
    Surface,
};





/// Простой интерфейс для связи кадра и графических функций.
/// Simple interface to connect graphics fuctions to the frame.
pub struct Graphics<'g,'p,'f,S:Surface>{
    pub graphics2d:&'g Graphics2D,

    pub draw_parameters:DrawParameters<'p>,

    pub frame:&'f mut S,
}





impl<'g,'p,'f,S:Surface> Graphics<'g,'p,'f,S>{
    #[inline(always)]
    pub (crate) fn new(
        graphics2d:&'g Graphics2D,
        draw_parameters:DrawParameters<'p>,
        frame:&'f mut S
    )->Graphics<'g,'p,'f,S>{
        Self{
            graphics2d,
            draw_parameters,
            frame
        }
    }

    /// Заполняет окно данным цветом.
    /// 
    /// Fills a window with the given colour.
    #[inline(always)]
    pub fn clear_colour(&mut self,[r,g,b,a]:[f32;4]){
        self.frame.clear_color(r,g,b,a)
    }

    /// Рисует сохранённый объект.
    /// 
    /// Draws an object.
    #[cfg(any(feature="simple_graphics",feature="texture_graphics",feature="text_graphics"))]
    pub fn draw_object(
        &mut self,
        index:usize,
        object_type:ObjectType,
        draw_type:DrawType,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        match object_type{
            #[cfg(feature="simple_graphics")]
            ObjectType::Simple=>{
                match draw_type{
                    DrawType::Common=>self.draw_simple_object(
                        index,
                        #[cfg(feature="colour_filter")]colour_filter,
                    ),

                    DrawType::Shifting(shift)=>self.draw_shift_simple_object(
                        index,
                        shift,
                        #[cfg(feature="colour_filter")]colour_filter,
                    ),

                    DrawType::Rotating((angle,position))=>self.draw_rotate_simple_object(
                        index,
                        position,
                        angle,
                        #[cfg(feature="colour_filter")]colour_filter,
                    ),
                }
            }

            #[cfg(feature="texture_graphics")]
            ObjectType::Textured=>{
                match draw_type{
                    DrawType::Common=>self.draw_textured_object(
                        index,
                        #[cfg(feature="colour_filter")]colour_filter,
                    ),

                    DrawType::Shifting(shift)=>self.draw_shift_textured_object(
                        index,
                        shift,
                        #[cfg(feature="colour_filter")]colour_filter,
                    ),

                    DrawType::Rotating((angle,position))=>self.draw_rotate_textured_object(
                        index,
                        position,
                        angle,
                        #[cfg(feature="colour_filter")]colour_filter,
                    )
                }
            }

            #[cfg(feature="text_graphics")]
            ObjectType::Text=>{
                match draw_type{
                    DrawType::Common=>self.draw_text_object(
                        index,
                        #[cfg(feature="colour_filter")]colour_filter
                    ),

                    DrawType::Shifting(shift)=>self.draw_shift_text_object(
                        index,
                        shift,
                        #[cfg(feature="colour_filter")]colour_filter
                    ),

                    DrawType::Rotating((angle,position))=>self.draw_rotate_text_object(
                        index,
                        position,
                        angle,
                        #[cfg(feature="colour_filter")]colour_filter
                    ),
                }
            }

            #[cfg(not(all(feature="simple_graphics",feature="texture_graphics",feature="text_graphics")))]
            _=>Ok(())
        }
    }
}





/// # Функции для работы с простыми объектами. Functions to work with simple objects.
#[cfg(feature="simple_graphics")]
impl<'g,'p,'f,S:Surface> Graphics<'g,'p,'f,S>{
    /// Рисует простой объект.
    /// 
    /// Draws a simple object.
    #[inline(always)]
    pub fn draw_simple<O,V,I>(
        &mut self,
        object:&O,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        
    )->Result<(),DrawError>
        where
            O:DependentObject<
                Vertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[Vertex2D]>,
            I:AsRef<[u8]>
    {
        self.graphics2d.simple.draw(
            object,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует сдвинутый простой объект.
    /// 
    /// Draws a shifted simple object.
    #[inline(always)]
    pub fn draw_shift_simple<O,V,I>(
        &mut self,
        object:&O,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>
        where
            O:DependentObject<
                Vertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[Vertex2D]>,
            I:AsRef<[u8]>
    {
        self.graphics2d.simple.draw_shift(
            object,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует повёрнутый простой объект.
    /// 
    /// Draws a rotated simple object.
    #[inline(always)]
    pub fn draw_rotate_simple<O,V,I>(
        &mut self,
        object:&O,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        
    )->Result<(),DrawError>
        where
            O:DependentObject<
                Vertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[Vertex2D]>,
            I:AsRef<[u8]>
    {
        self.graphics2d.simple.draw_rotate(
            object,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует простой объект.
    /// 
    /// Draws a simple object.
    #[inline(always)]
    pub fn draw_simple_general<O,V,I>(
        &mut self,
        object:&O,
        draw_type:DrawType,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>
        where
            O:DependentObject<
                Vertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[Vertex2D]>,
            I:AsRef<[u8]>
    {
        match draw_type{
            DrawType::Common=>self.graphics2d.simple.draw(
                object,
                #[cfg(feature="colour_filter")]colour_filter,
                &self.draw_parameters,
                self.frame
            ),

            DrawType::Shifting(shift)=>self.graphics2d.simple.draw_shift(
                object,
                shift,
                #[cfg(feature="colour_filter")]colour_filter,
                &self.draw_parameters,
                self.frame
            ),

            DrawType::Rotating((angle,position))=>self.graphics2d.simple.draw_rotate(
                object,
                position,
                angle,
                #[cfg(feature="colour_filter")]colour_filter,
                &self.draw_parameters,
                self.frame
            ),
        }
    }
}



/// # Функции для отрисовки текстур. Texture rendering functions.
#[cfg(feature="texture_graphics")]
impl<'g,'p,'f,S:Surface> Graphics<'g,'p,'f,S>{
    /// Рисует изображение.
    /// 
    /// Draws a texture.
    #[inline(always)]
    pub fn draw_texture<O,V,I>(
        &mut self,
        base:&O,
        texture:&Texture,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>
        where
            O:DependentObject<
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>,
            I:AsRef<[u8]>
    {
        self.graphics2d.texture.draw(
            base,
            texture,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует сдвинутую текстуру.
    /// 
    /// Draws a shifted texture.
    #[inline(always)]
    pub fn draw_shift_texture<O,V,I>(
        &mut self,
        base:&O,
        texture:&Texture,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>
        where
            O:DependentObject<
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>,
            I:AsRef<[u8]>
    {
        self.graphics2d.texture.draw_shift(
            base,
            texture,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует повёрнутую текстуру.
    /// 
    /// Draws a rotated texture.
    /// 
    /// angle - radians
    #[inline(always)]
    pub fn draw_rotate_texture<O,V,I>(
        &mut self,
        base:&O,
        texture:&Texture,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>
        where
            O:DependentObject<
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>,
            I:AsRef<[u8]>
    {
        self.graphics2d.texture.draw_rotate(
            base,
            texture,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame,
        )
    }

    /// Рисует текстуру.
    /// 
    /// Draws a texture.
    #[inline(always)]
    pub fn draw_texture_general<O,V,I>(
        &mut self,
        base:&O,
        texture:&Texture,
        draw_type:DrawType,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>
        where
            O:DependentObject<
                TexturedVertex2D,
                u8,
                Vertices=V,
                Indices=I
            >,
            V:AsRef<[TexturedVertex2D]>,
            I:AsRef<[u8]>
    {
        match draw_type{
            DrawType::Common=>self.graphics2d.texture.draw(
                base,
                texture,
                #[cfg(feature="colour_filter")]colour_filter,
                &self.draw_parameters,
                self.frame
            ),

            DrawType::Shifting(shift)=>self.graphics2d.texture.draw_shift(
                base,
                texture,
                shift,
                #[cfg(feature="colour_filter")]colour_filter,
                &self.draw_parameters,
                self.frame
            ),

            DrawType::Rotating((angle,position))=>self.graphics2d.texture.draw_rotate(
                base,
                texture,
                position,
                angle,
                #[cfg(feature="colour_filter")]colour_filter,
                &self.draw_parameters,
                self.frame
            )
        }
    }
}



/// # Фукнции для отрисовки глифов. Glyph rendering functions.
#[cfg(feature="text_graphics")]
impl<'g,'p,'f,S:Surface> Graphics<'g,'p,'f,S>{
    /// Строит и выводит глиф.
    /// 
    /// Builds and draws a glyph.
    #[inline(always)]
    pub fn draw_glyph(
        &mut self,
        glyph:&OutlinedGlyph,
        colour:Colour,
        [x,y]:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        // Отрисовывает глиф на текстуру
        self.graphics2d.text.write_glyph(glyph);

        // Получение размера глифа
        let [w,h]=glyph.size();
        let [w,h]=[w as f32,h as f32];


        // Получение размера текстуры
        let [tw,th]=self.graphics2d.text.texture_size();

        // Получение относительного размера глифа
        let uv=[
            w/tw,
            h/th
        ];

        // Строит вершины и загружает их в буфер
        self.graphics2d.text.write_vertices(
            [x,y,w,h],
            uv
        );

        let texture=self.graphics2d.text.texture();

        self.graphics2d.text.draw_glyph(
            texture,
            colour,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Строит и выводит сдвинутый глиф.
    /// 
    /// Builds and draws a shifted glyph.
    #[inline(always)]
    pub fn draw_shift_glyph(
        &mut self,
        glyph:&OutlinedGlyph,
        colour:Colour,
        [x,y]:[f32;2],
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        // Отрисовывает глиф на текстуру
        self.graphics2d.text.write_glyph(glyph);

        // Получение размера глифа
        let [w,h]=glyph.size();
        let [w,h]=[w as f32,h as f32];

        // Получение размера текстуры
        let [tw,th]=self.graphics2d.text.texture_size();

        // Получение относительного размера глифа
        let uv=[
            w/tw,
            h/th
        ];

        // Строит вершины и загружает их в буфер
        self.graphics2d.text.write_vertices(
            [x,y,w,h],
            uv
        );

        let texture=self.graphics2d.text.texture();

        self.graphics2d.text.draw_shift_glyph(
            texture,
            colour,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Строит и выводит повёрнутый глиф.
    /// 
    /// Builds and draws a rotated glyph.
    #[inline(always)]
    pub fn draw_rotate_glyph(
        &mut self,
        glyph:&OutlinedGlyph,
        colour:Colour,
        [x,y]:[f32;2],
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        // Отрисовывает глиф на текстуру
        self.graphics2d.text.write_glyph(glyph);

        // Получение размера глифа
        let [w,h]=glyph.size();
        let [w,h]=[w as f32,h as f32];

        // Получение размера текстуры
        let [tw,th]=self.graphics2d.text.texture_size();

        // Получение относительного размера глифа
        let uv=[
            w/tw,
            h/th
        ];

        // Строит вершины и загружает их в буфер
        self.graphics2d.text.write_vertices(
            [x,y,w,h],
            uv
        );

        let texture=self.graphics2d.text.texture();

        self.graphics2d.text.draw_rotate_glyph(
            texture,
            colour,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Выводит уже построенный глиф.
    /// 
    /// Draws an already built glyph.
    #[inline(always)]
    pub fn draw_glyph_cache(
        &mut self,
        glyph:&TexturedGlyph,
        colour:Colour,
        [x,y]:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        let size=glyph.size();
        // Строит вершины и загружает их в буфер
        self.graphics2d.text.write_vertices([x,y,size[0],size[1]],[1f32,1f32]);

        self.graphics2d.text.draw_glyph(
            glyph.texture(),
            colour,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Выводит сдвинутый уже построенный глиф.
    /// 
    /// Draws a shifted already built glyph.
    #[inline(always)]
    pub fn draw_shift_glyph_cache(
        &mut self,
        glyph:&TexturedGlyph,
        colour:Colour,
        [x,y]:[f32;2],
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        let size=glyph.size();
        // Строит вершины и загружает их в буфер
        self.graphics2d.text.write_vertices([x,y,size[0],size[1]],[1f32,1f32]);

        self.graphics2d.text.draw_shift_glyph(
            glyph.texture(),
            colour,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Выводит повёрнутый уже построенный глиф.
    /// 
    /// Draws a rotated already built glyph.
    #[inline(always)]
    pub fn draw_rotate_glyph_cache(
        &mut self,
        glyph:&TexturedGlyph,
        colour:Colour,
        [x,y]:[f32;2],
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        let size=glyph.size();
        // Строит вершины и загружает их в буфер
        self.graphics2d.text.write_vertices([x,y,size[0],size[1]],[1f32,1f32]);

        self.graphics2d.text.draw_rotate_glyph(
            glyph.texture(),
            colour,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Выводит уже построенный глиф.
    /// 
    /// Draws a already built glyph.
    pub fn draw_glyph_cache_general(
        &mut self,
        glyph:&TexturedGlyph,
        colour:Colour,
        [x,y]:[f32;2],
        draw_type:DrawType,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        let size=glyph.size();
        // Строит вершины и загружает их в буфер
        self.graphics2d.text.write_vertices([x,y,size[0],size[1]],[1f32,1f32]);

        match draw_type{
            DrawType::Common=>self.graphics2d.text.draw_glyph(
                glyph.texture(),
                colour,
                #[cfg(feature="colour_filter")]colour_filter,
                &self.draw_parameters,
                self.frame
            ),

            DrawType::Shifting(shift)=>self.graphics2d.text.draw_shift_glyph(
                glyph.texture(),
                colour,
                shift,
                #[cfg(feature="colour_filter")]colour_filter,
                &self.draw_parameters,
                self.frame
            ),

            DrawType::Rotating((angle,position))=>self.graphics2d.text.draw_rotate_glyph(
                glyph.texture(),
                colour,
                position,
                angle,
                #[cfg(feature="colour_filter")]colour_filter,
                &self.draw_parameters,
                self.frame
            ),
        }
    }
}





/// # Функции для работы с сохранёнными простыми объектами. Functions to work with saved simple objects.
#[cfg(feature="simple_graphics")]
impl<'g,'p,'f,S:Surface> Graphics<'g,'p,'f,S>{
    /// Рисует простой объект.
    /// 
    /// Draws the saved simple object.
    #[inline(always)]
    pub fn draw_simple_object(
        &mut self,
        index:usize,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        
    )->Result<(),DrawError>{
        self.graphics2d.simple.draw_object(
            index,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует сдвинутый простой объект.
    /// 
    /// Draws the shifted saved simple object.
    #[inline(always)]
    pub fn draw_shift_simple_object(
        &mut self,
        index:usize,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        
    )->Result<(),DrawError>{
        self.graphics2d.simple.draw_shift_object(
            index,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует повёрнутый простой объект.
    /// 
    /// Draws the rotated saved simple object.
    #[inline(always)]
    pub fn draw_rotate_simple_object(
        &mut self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        
    )->Result<(),DrawError>{
        self.graphics2d.simple.draw_rotate_object(
            index,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует простой объект.
    /// 
    /// Draws the saved simple object.
    #[inline(always)]
    pub fn draw_simple_object_general(
        &mut self,
        index:usize,
        draw_type:DrawType,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        match draw_type{
            DrawType::Common=>self.draw_simple_object(
                index,
                #[cfg(feature="colour_filter")]colour_filter,
            ),

            DrawType::Shifting(shift)=>self.draw_shift_simple_object(
                index,
                shift,
                #[cfg(feature="colour_filter")]colour_filter,
            ),

            DrawType::Rotating((angle,position))=>self.draw_rotate_simple_object(
                index,
                position,
                angle,
                #[cfg(feature="colour_filter")]colour_filter,
            ),
        }
    }
}



/// # Функции для работы с сохранёнными текстурными объектами. Functions to work with saved textured objects.
#[cfg(feature="texture_graphics")]
impl<'g,'p,'f,S:Surface> Graphics<'g,'p,'f,S>{
    /// Рисует сохранённый текстурный объект.
    /// 
    /// Draws a saved textured object.
    #[inline(always)]
    pub fn draw_textured_object(
        &mut self,
        index:usize,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_object(
            index,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame,
        )
    }

    /// Рисует сдвинутый сохранённый текстурный объект.
    /// 
    /// Draws a shifted saved textured object.
    #[inline(always)]
    pub fn draw_shift_textured_object(
        &mut self,
        index:usize,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_shift_object(
            index,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует повёрнутый сохранённый текстурный объект.
    /// 
    /// Draws a rotated saved textured object.
    /// 
    /// rotation_center - [x, y]
    /// angle - radians
    #[inline(always)]
    pub fn draw_rotate_textured_object(
        &mut self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        self.graphics2d.texture.draw_rotate_object(
            index,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame
        )
    }

    /// Рисует текстурный объект.
    /// 
    /// Draws a textured object.
    #[inline(always)]
    pub fn draw_textured_object_general(
        &mut self,
        index:usize,
        draw_type:DrawType,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        match draw_type{
            DrawType::Common=>self.draw_textured_object(
                index,
                #[cfg(feature="colour_filter")]colour_filter,
            ),

            DrawType::Shifting(shift)=>self.draw_shift_textured_object(
                index,
                shift,
                #[cfg(feature="colour_filter")]colour_filter,
            ),

            DrawType::Rotating((angle,position))=>self.draw_rotate_textured_object(
                index,
                position,
                angle,
                #[cfg(feature="colour_filter")]colour_filter,
            )
        }
    }
}



/// # Функции для работы с сохранёнными текстовыми объектами. Functions to work with saved text objects.
#[cfg(feature="text_graphics")]
impl<'g,'p,'f,S:Surface> Graphics<'g,'p,'f,S>{
    /// Рисует сохранённый текстовой объект.
    /// 
    /// Draws a saved text object.
    #[inline(always)]
    pub fn draw_text_object(
        &mut self,
        index:usize,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
        
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_object(
            index,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame,
        )
    }

    /// Рисует сохранённый текстовой объект.
    /// 
    /// Draws a saved text object.
    #[inline(always)]
    pub fn draw_shift_text_object(
        &mut self,
        index:usize,
        shift:[f32;2],
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_shift_object(
            index,
            shift,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame,
        )
    }

    /// Рисует сохранённый текстовой объект.
    /// 
    /// Draws a saved text object.
    #[inline(always)]
    pub fn draw_rotate_text_object(
        &mut self,
        index:usize,
        rotation_center:[f32;2],
        angle:f32,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        self.graphics2d.text.draw_rotate_object(
            index,
            rotation_center,
            angle,
            #[cfg(feature="colour_filter")]colour_filter,
            &self.draw_parameters,
            self.frame,
        )
    }

    /// Рисует сохранённый текстовой объект.
    /// 
    /// Draws a saved text object.
    pub fn draw_text_object_general(
        &mut self,
        index:usize,
        draw_type:DrawType,
        #[cfg(feature="colour_filter")]colour_filter:ColourFilter,
    )->Result<(),DrawError>{
        match draw_type{
            DrawType::Common=>self.draw_text_object(
                index,
                #[cfg(feature="colour_filter")]colour_filter,
            ),

            DrawType::Shifting(shift)=>self.draw_shift_text_object(
                index,
                shift,
                #[cfg(feature="colour_filter")]colour_filter,
            ),

            DrawType::Rotating((angle,position))=>self.draw_rotate_text_object(
                index,
                position,
                angle,
                #[cfg(feature="colour_filter")]colour_filter,
            ),
        }
    }
}