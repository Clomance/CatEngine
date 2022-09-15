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
        TextRenderData,
        Vertices,
        Indices,
    },

    text::{
        GlyphCache,
        FontOwner
    }
};

use std::time::{
    Instant,
    Duration
};

pub struct ExampleSystem;

impl<'a> System<'a> for ExampleSystem {
    type CreateParameters = ();
    type SharedData = ();
    type Objects = ();

    fn create(
        _create_parameters: &mut Self::CreateParameters,
        _window: &Window,
        _shared: &mut Self::SharedData
    ) -> ExampleSystem {
        ExampleSystem
    }

    fn set_objects(&mut self, _shared: &mut Self::SharedData, mut object_manager: ObjectManager) -> Self::Objects {
        let graphics = object_manager.graphics();

        graphics.parameters.set_clear_colour(Some([1f32; 4]));

        let font = FontOwner::load("resources/font1").unwrap();
        let glyph_cache = GlyphCache::new("9876543210", 60f32, &font);
        let font = graphics.text.push_font(glyph_cache).unwrap();

        let attributes = MeshAttributes::new(PrimitiveType::Triangles);
        let layer = graphics.text.create_layer(attributes).unwrap();

        graphics.push_text_layer(layer, font);

        let object = FpsCounter::new();
        let indices=[
            0,1,2,
            1,2,3,
            4,5,6,
            5,6,7,
            8,9,10,
            9,10,11
        ];
        object_manager.push_text_object(object, Vertices::empty(12), Indices::new(&indices), layer).unwrap();
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

    fn destroy(&mut self, _shared:&mut Self::SharedData, _graphics:&mut Graphics) {

    }
}

impl<'a> StartSystem<'a> for ExampleSystem {
    fn create_shared_data(_create_parameters: &mut Self::CreateParameters) -> Self::SharedData {

    }
}

fn main() {
    let attributes = AppAttributes::new("ExampleWindow");

    let mut app = App::new::<ExampleSystem>(attributes, &mut ()).unwrap();

    app.run();
}

pub struct FpsCounter {
    last_redraw: Instant,
    frames: u32,
}

impl FpsCounter {
    pub fn new() -> FpsCounter{
        Self{
            last_redraw: Instant::now(),
            frames: 0
        }
    }
}

impl TextObject for FpsCounter{
    fn event(&mut self, event: ObjectEvent, render_data: &mut TextRenderData) {
        match event {
            ObjectEvent::Prerender => {
                let colour = [0f32, 0f32, 0f32, 1f32];

                let redraw_start = Instant::now();

                self.frames += 1;

                if redraw_start.duration_since(self.last_redraw) >= Duration::from_secs(1) {
                    self.last_redraw = redraw_start;

                    let font = render_data.glyph_cache;

                    let mut position = [0f32; 2];

                    let global_size = font.global_size();
                    let global_offset = font.global_offset();

                    let mut vertices = Vec::with_capacity(12);

                    for n in self.frames.to_string().chars(){
                        let glyph = font.glyph(&n).unwrap();

                        let x1 = position[0] + global_offset[0];
                        let y1 = position[1] - global_offset[1];
                        let x2 = position[0] + global_offset[0] + global_size[0];
                        let y2 = position[1] - global_offset[1] + global_size[1];

                        let glyph_texture = glyph.texture as f32;

                        position[0] += glyph.horizontal_advance;

                        vertices.push(TextVertex::new([x1, y1, 1f32, 1f32], colour, [0f32, 1f32, glyph_texture]));
                        vertices.push(TextVertex::new([x1, y2, 1f32, 1f32], colour, [0f32, 0f32, glyph_texture]));
                        vertices.push(TextVertex::new([x2, y1, 1f32, 1f32], colour, [1f32, 1f32, glyph_texture]));
                        vertices.push(TextVertex::new([x2, y2, 1f32, 1f32], colour, [1f32, 0f32, glyph_texture]));
                    }

                    render_data.render.set_index_render_bounds(0, vertices.len() / 2 * 3).unwrap();

                    render_data.render.write_vertices(0, &vertices).unwrap();

                    self.frames = 0;
                }
            }

            _ => {}
        }
    }
}