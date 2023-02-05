use cat_engine::{
    App,
    AppAttributes,

    window::{
        Window,
        VirtualKeyCode,
    },

    system::{
        System,
        SystemEvent,
        SystemManager,
        SystemStatus,
        StartSystem, ComponentManager, ResourceManager,
    },

    object::{
        ObjectManager,
        ObjectReference,
        
    },

    graphics::{
        Colour,
        SimpleVertex,
        TexturedVertex,
        MeshAttributes,
        PrimitiveType,
        SimpleObject,
        ObjectEvent,
        Vertices,
        Indices,
        TextureObject,
        SimpleRenderDataInterface,
        TextureRenderDataInterface,
        GraphicsManager,
    },

    texture::{
        Texture2D,
        TextureMinFilter,
        TextureMagFilter,
    }
};

use image::GenericImageView;



const BLUE: Colour = [0f32, 0f32, 1f32, 1f32];

const PADDLE_SPEED: f32 = 6f32;



fn main() {
    let attributes = AppAttributes::new("WindowExample");
    let mut app = App::new::<Pong>(attributes, &mut ()).unwrap();
    app.run()
}



enum PaddleMoving {
    None,
    Up,
    Down,
}

pub struct Pong;

impl Pong {
    pub fn new() -> Pong {
        Self
    }
}

impl<'s, 'a> System<'s, 'a> for Pong {
    type Objects = (
        ObjectReference<'a, Paddle>,
        ObjectReference<'a, Paddle>,
        ObjectReference<'a, Ball>,
    );
    type SharedData = ();

    fn set_up(
        &mut self,
        _shared: &mut Self::SharedData,
        mut objects: ObjectManager,
        _resources: ResourceManager,
        mut components: ComponentManager
    ) -> Self::Objects {
        components.graphics.parameters.set_clear_colour(Some([1f32; 4]));

        let attributes = MeshAttributes::new(PrimitiveType::Triangles);
        let simple_layer = components.graphics.simple.create_layer(attributes).unwrap();

        components.graphics.push_simple_layer(simple_layer);

        let paddle = Paddle::new([0f32, 0f32], [20f32, 120f32], BLUE);
        let vertices = paddle.vertices();
        let indices = paddle.indices();
        let left_paddle = objects.graphics.push_simple_object(paddle, Vertices::new(&vertices), Indices::new(&indices), simple_layer, &mut components.graphics.simple).unwrap();

        let paddle = Paddle::new([0f32, 0f32], [20f32, 120f32], BLUE);
        let vertices = paddle.vertices();
        let indices = paddle.indices();
        let right_paddle = objects.graphics.push_simple_object(paddle, Vertices::new(&vertices), Indices::new(&indices), simple_layer, &mut components.graphics.simple).unwrap();

        let image = image::open("resources/ball.png").unwrap();
        let texture = Texture2D::new(TextureMinFilter::Linear, TextureMagFilter::Linear, [image.width(), image.height()], image.as_bytes());
        let texture = components.graphics.texture.push_texture(texture).unwrap();

        let attributes = MeshAttributes::new(PrimitiveType::Triangles);
        let textured_layer = components.graphics.texture.create_layer(attributes).unwrap();

        components.graphics.push_texture_layer(textured_layer, texture);

        let ball = Ball::new([100f32, 100f32], [40f32, 40f32]);
        let vertices = ball.vertices();
        let indices = ball.indices();
        let ball = objects.graphics.push_texture_object(ball, Vertices::new(&vertices), Indices::new(&indices), simple_layer, &mut components.graphics.texture).unwrap();

        (
            left_paddle,
            right_paddle,
            ball
        )
    }

    fn handle(
        &mut self,
        event: SystemEvent,
        (paddle1, paddle2, ball): &mut Self::Objects,
        _shared: &mut Self::SharedData,
        system_manager: SystemManager
    ) -> SystemStatus {
        match event {
            SystemEvent::Update => {
                if ball.direction_x {
                    if ball.position[0] + ball.size[0] >= paddle2.position[0] {
                        if
                            ball.position[1] >= paddle2.position[1]
                            &&
                            ball.position[1] <= paddle2.position[1] + paddle2.size[1]
                            ||
                            ball.position[1] + ball.size[1] >= paddle2.position[1]
                            &&
                            ball.position[1] + ball.size[1] <= paddle2.position[1] + paddle2.size[1]
                        {
                            match paddle2.moving {
                                PaddleMoving::None => {}
                                PaddleMoving::Up => {
                                    if ball.direction_y {
                                        ball.direction_y = !ball.direction_y;
                                    }
                                    else {
                                        ball.direction_y = ball.direction_y;
                                    }
                                }
                                PaddleMoving::Down => {
                                    if ball.direction_y {
                                        ball.direction_y = ball.direction_y;
                                    }
                                    else {
                                        ball.direction_y = !ball.direction_y;
                                    }
                                }
                            }

                            ball.direction_x = !ball.direction_x;
                        }
                    }
                }
                else {
                    if ball.position[0] <= paddle1.position[0] + paddle1.size[0] {
                        if
                            ball.position[1] >= paddle1.position[1]
                            &&
                            ball.position[1] <= paddle1.position[1] + paddle1.size[1]
                            ||
                            ball.position[1] + ball.size[1] >= paddle1.position[1]
                            &&
                            ball.position[1] + ball.size[1] <= paddle1.position[1] + paddle1.size[1]
                        {
                            match paddle1.moving {
                                PaddleMoving::None => {}
                                PaddleMoving::Up => {
                                    if ball.direction_y {
                                        ball.direction_y = !ball.direction_y;
                                    }
                                    else {
                                        ball.direction_y = ball.direction_y;
                                    }
                                }
                                PaddleMoving::Down => {
                                    if ball.direction_y {
                                        ball.direction_y = ball.direction_y;
                                    }
                                    else {
                                        ball.direction_y = !ball.direction_y;
                                    }
                                }
                            }
                            ball.direction_x = !ball.direction_x;
                        }
                    }
                }

                let size = system_manager.window.client_size();
                let field = [size[0] as f32, size[1] as f32];

                paddle1.position[0] = 0f32;
                paddle2.position[0] = field[0] - paddle2.size[0];

                if ball.direction_y {
                    if ball.position[1] + ball.size[1] > field[1]{
                        ball.position[1] = field[1] - ball.size[1];
                        ball.direction_y = !ball.direction_y;
                    }
                }
                else {
                    if ball.position[1] < 0f32 {
                        ball.position[1] = 0f32;
                        ball.direction_y = !ball.direction_y;
                    }
                }

                if paddle1.position[1] < 0f32 {
                    paddle1.position[1] = 0f32;
                }
                else if paddle1.position[1] + paddle1.size[1] > field[1] {
                    paddle1.position[1] = field[1] - paddle2.size[1];
                }

                if paddle2.position[1] < 0f32 {
                    paddle2.position[1] = 0f32;
                }
                else if paddle2.position[1] + paddle2.size[1] > field[1] {
                    paddle2.position[1] = field[1] - paddle2.size[1];
                }
            }

            SystemEvent::Keyboard {state, key} => {
                match key {
                    VirtualKeyCode::W => {
                        if state {
                            paddle1.moving = PaddleMoving::Up
                        }
                        else {
                            paddle1.moving = PaddleMoving::None
                        }
                    }
                    VirtualKeyCode::S => {
                        if state {
                            paddle1.moving = PaddleMoving::Down
                        }
                        else {
                            paddle1.moving = PaddleMoving::None
                        }
                    }

                    VirtualKeyCode::UP => {
                        if state {
                            paddle2.moving = PaddleMoving::Up
                        }
                        else {
                            paddle2.moving = PaddleMoving::None
                        }
                    }
                    VirtualKeyCode::DOWN => {
                        if state {
                            paddle2.moving = PaddleMoving::Down
                        }
                        else {
                            paddle2.moving = PaddleMoving::None
                        }
                    }
                    _ => {}
                }
            }

            _ => {}
        }

        SystemStatus::Next
    }

    fn destroy(
        &mut self,
        _shared: &mut Self::SharedData,
        _graphics: GraphicsManager
    ) {
        
    }
}

impl<'s, 'a> StartSystem<'s, 'a> for Pong {
    type CreateParameters = ();

    fn create(
        _create_parameters: &mut Self::CreateParameters,
        _window: &Window,
        _shared: &mut Self::SharedData,
    ) -> Self {
        Pong::new()
    }

    fn create_shared_data(_create_parameters: &mut Self::CreateParameters) -> Self::SharedData {
        
    }
}



pub struct Paddle {
    position: [f32; 2],
    size: [f32; 2],
    colour: Colour,
    moving: PaddleMoving
}

impl Paddle {
    pub fn new(position: [f32; 2], size: [f32; 2], colour: Colour) -> Paddle {
        Self {
            position,
            size,
            colour,
            moving: PaddleMoving::None
        }
    }

    pub fn vertices(&self) -> [SimpleVertex; 4] {
        let x1 = self.position[0];
        let y1 = self.position[1];
        let x2 = self.position[0] + self.size[0];
        let y2 = self.position[1] + self.size[1];
        [
            SimpleVertex::new([x1, y1, 1f32, 1f32], self.colour),
            SimpleVertex::new([x1, y2, 1f32, 1f32], self.colour),
            SimpleVertex::new([x2, y1, 1f32, 1f32], self.colour),
            SimpleVertex::new([x2, y2, 1f32, 1f32], self.colour),
        ]
    }

    fn indices(&self) -> [u16; 6] {
        [
            0, 1, 2,
            1, 2, 3
        ]
    }
}

impl SimpleObject for Paddle {
    fn event(&mut self, event: ObjectEvent, render_data: SimpleRenderDataInterface) {
        match event {
            ObjectEvent::Update => {
                match self.moving {
                    PaddleMoving::None => {}

                    PaddleMoving::Up => {
                        self.position[1] -= PADDLE_SPEED;
                    }

                    PaddleMoving::Down => {
                        self.position[1] += PADDLE_SPEED;
                    }
                }
            }

            ObjectEvent::Prerender => {
                let vertices = self.vertices();

                render_data.get_render_data().write_vertices(0, &vertices).unwrap();
            }
        }
    }
}



const BALL_START_SPEED: f32= 20f32;

pub struct Ball {
    position: [f32; 2],
    size: [f32; 2],
    direction_x: bool,
    direction_y: bool,
    speed: f32,
}

impl Ball {
    pub fn new(position: [f32; 2], size: [f32; 2]) -> Ball {
        Self {
            position,
            size,
            direction_x: true,
            direction_y: true,
            speed: BALL_START_SPEED,
        }
    }

    fn vertices(&self) -> [TexturedVertex; 4] {
        let colour = [1f32; 4];

        let x1 = self.position[0];
        let y1 = self.position[1];
        let x2 = self.position[0] + self.size[0];
        let y2 = self.position[1] + self.size[1];
        [
            TexturedVertex::new([x1, y1, 1f32, 1f32], colour, [0f32, 1f32]),
            TexturedVertex::new([x1, y2, 1f32, 1f32], colour, [0f32, 0f32]),
            TexturedVertex::new([x2, y1, 1f32, 1f32], colour, [1f32, 1f32]),
            TexturedVertex::new([x2, y2, 1f32, 1f32], colour, [1f32, 0f32]),
        ]
    }

    fn indices(&self) -> [u16; 6] {
        [
            0, 1, 2,
            1, 2, 3
        ]
    }
}

impl TextureObject for Ball {
    fn event(&mut self, event: ObjectEvent, render_data: TextureRenderDataInterface){
        match event {
            ObjectEvent::Prerender => {
                let vertices = self.vertices();

                render_data.get_render_data().write_vertices(0, &vertices).unwrap();
            }

            ObjectEvent::Update => {
                if self.direction_y {
                    self.position[1] += self.speed / 2f32;
                }
                else {
                    self.position[1] -= self.speed / 2f32;
                }

                if self.direction_x {
                    self.position[0] += self.speed;
                }
                else {
                    self.position[0] -= self.speed;
                }
            }
        }
    }
}