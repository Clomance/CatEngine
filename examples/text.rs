use cat_engine::{
    App,
    AppAttributes,

    window::{
        Window,
    },

    graphics::{
        Graphics,
        TextVertex,
        MeshAttributes,
        PrimitiveType,
        ElementIndexType
    },

    system::{
        System,
        StartSystem,
        SystemManager,
        SystemEvent,
        SystemStatus,
    },

    object::{
        ObjectManager,
        TextObject,
        ObjectEvent,
        Vertices,
        Indices
    },

    text::{
        GlyphCache,
        FontOwner
    }
};



pub struct ExampleSystem;

impl<'s, 'a> System<'s, 'a> for ExampleSystem {
    type SharedData = ();
    type Objects = ();

    fn set_objects(&mut self, _shared: &mut Self::SharedData, mut object_manager: ObjectManager) -> Self::Objects {
        let graphics = object_manager.graphics();

        graphics.parameters.set_clear_colour(Some([1f32; 4]));

        let font = FontOwner::load("resources/font1").unwrap();
        let glyph_cache = GlyphCache::new("Something", 60f32, &font);
        let font = graphics.text.push_font(glyph_cache).unwrap();

        let attributes = MeshAttributes::new(PrimitiveType::Triangles);
        let layer = graphics.text.create_layer(attributes).unwrap();

        graphics.push_text_layer(layer, font);

        let text = TextView::new([100f32; 2], [0f32, 0f32, 0f32, 1f32], 60f32, "Something".to_string());
        let (vertices,indices)=text.attributes(object_manager.graphics().text.get_font(0).unwrap());
        object_manager.push_text_object(text, Vertices::new(&vertices), Indices::new(&indices), layer).unwrap();
    }

    fn handle(
        &mut self,
        _objects: &mut Self::Objects,
        _event: SystemEvent,
        _window: &Window,
        _shared: &mut Self::SharedData,
        _system_manager: SystemManager
    ) -> SystemStatus {
        SystemStatus::Next
    }

    fn destroy(
        &mut self,
        _shared:&mut Self::SharedData,
        _graphics:&mut Graphics
    ) {

    }
}

impl<'s, 'a> StartSystem<'s, 'a> for ExampleSystem {
    type CreateParameters = ();

    fn create(
        _create_parameters: &mut Self::CreateParameters,
        _window: &Window,
        _shared: &mut Self::SharedData
    ) -> ExampleSystem {
        ExampleSystem
    }

    fn create_shared_data(_create_parameters: &mut Self::CreateParameters) -> Self::SharedData {

    }
}

fn main() {
    let attributes = AppAttributes::new("ExampleWindow");

    let mut app = App::new::<ExampleSystem>(attributes, &mut ()).unwrap();

    app.run();
}



pub struct TextView {
    position: [f32; 2],
    colour: [f32; 4],
    font_size: f32,
    text: String,
}

impl TextView {
    pub fn new(
        position: [f32; 2],
        colour: [f32; 4],
        font_size: f32,
        text: String
    ) -> TextView {
        Self {
            position,
            colour,
            font_size,
            text,
        }
    }

    pub fn attributes(&self, font: &GlyphCache) -> (Vec<TextVertex>, Vec<ElementIndexType>){
        let mut position = self.position;

        let mut global_size = font.global_size();
        let mut global_offset = font.global_offset();

        let scale = self.font_size / global_size[0];

        global_size[0] *= scale;
        global_size[1] *= scale;
        global_offset[0] *= scale;
        global_offset[1] *= scale;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut i = 0;
        for character in self.text.chars() {
            if !character.is_whitespace() {
                let glyph = font.glyph(&character).unwrap();

                let x1 = position[0] + global_offset[0];
                let y1 = position[1] - global_offset[1];
                let x2 = position[0] + global_offset[0] + global_size[0];
                let y2 = position[1] - global_offset[1] + global_size[1];

                let glyph_texture = glyph.texture as f32;

                position[0] += glyph.horizontal_advance * scale;

                vertices.push(TextVertex::new([x1, y1, 1f32, 1f32], self.colour, [0f32, 1f32, glyph_texture]));
                vertices.push(TextVertex::new([x1, y2, 1f32, 1f32], self.colour, [0f32, 0f32, glyph_texture]));
                vertices.push(TextVertex::new([x2, y1, 1f32, 1f32], self.colour, [1f32, 1f32, glyph_texture]));
                vertices.push(TextVertex::new([x2, y2, 1f32, 1f32], self.colour, [1f32, 0f32, glyph_texture]));

                for _ in 0..3 {
                    indices.push(i);
                    i += 1;
                }

                i -= 2;

                for _ in 0..3 {
                    indices.push(i);
                    i += 1;
                }
            }
            else{
                position[0] += font.whitespace_advance() * scale;
            }
        }

        (vertices,indices)
    }
}

impl TextObject for TextView {
    fn event(&mut self, _event: ObjectEvent) {

    }
}