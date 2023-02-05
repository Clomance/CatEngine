use cat_engine::{
    App,
    AppAttributes,

    window::{
        Window,
    },

    graphics::{
        TextVertex,
        MeshAttributes,
        PrimitiveType,
        TextObject,
        ObjectEvent,
        Vertices,
        Indices,
        TextRenderDataInterface, GraphicsManager,
    },

    system::{
        System,
        StartSystem,
        SystemManager,
        SystemEvent,
        SystemStatus, ResourceManager, ComponentManager,
    },

    object::{
        ObjectManager,
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

impl<'s, 'a> System<'s, 'a> for ExampleSystem {
    type SharedData = ();
    type Objects = ();

    fn set_up(
        &mut self,
        _shared: &mut Self::SharedData,
        mut objects: ObjectManager,
        _resources: ResourceManager,
        mut components: ComponentManager
    ) -> Self::Objects {
        components.graphics.parameters.set_clear_colour(Some([1f32; 4]));

        let font = FontOwner::load("resources/arial.ttf").unwrap();
        let glyph_cache = GlyphCache::new("9876543210", 60f32, &font);
        let font = components.graphics.text.manager().glyphs.push_glyphs(glyph_cache).unwrap();

        let attributes = MeshAttributes::new(PrimitiveType::Triangles);
        let layer = components.graphics.text.create_layer(attributes).unwrap();

        components.graphics.push_text_layer(layer, font);

        let object = FpsCounter::new();
        let indices=[
            0,1,2,
            1,2,3,
            4,5,6,
            5,6,7,
            8,9,10,
            9,10,11
        ];
        objects.graphics.push_text_object(object, Vertices::empty(12), Indices::new(&indices), layer, &mut components.graphics.text).unwrap();
    }

    fn handle(
        &mut self,
        _event: SystemEvent,
        _objects: &mut Self::Objects,
        _shared: &mut Self::SharedData,
        _system_manager: SystemManager
    ) -> SystemStatus {
        SystemStatus::Next
    }

    fn destroy(
        &mut self,
        _shared: &mut Self::SharedData,
        _graphics: GraphicsManager
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
    fn event(&mut self, event: ObjectEvent, render_data: TextRenderDataInterface) {
        match event {
            ObjectEvent::Prerender => {
                let colour = [0f32, 0f32, 0f32, 1f32];

                let redraw_start = Instant::now();

                self.frames += 1;

                if redraw_start.duration_since(self.last_redraw) >= Duration::from_secs(1) {
                    self.last_redraw = redraw_start;

                    let font = render_data.get_font();

                    let mut position = [0f32; 2];

                    let global_size = font.cache().global_size();
                    let global_offset = font.cache().global_offset();

                    let mut vertices = Vec::with_capacity(12);

                    for n in self.frames.to_string().chars(){
                        let glyph = font.cache().glyph(&n).unwrap();

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

                    render_data.get_render_data().set_index_render_bounds(0, vertices.len() / 2 * 3).unwrap();

                    render_data.get_render_data().write_vertices(0, &vertices).unwrap();

                    self.frames = 0;
                }
            }

            _ => {}
        }
    }
}