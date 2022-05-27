use sfml::graphics::RenderWindow;
use sfml::graphics::*;
use cgmath::Vector2;
use cgmath::*;
use sfml::system::Vector2f;
use sfml::window::Event;

struct Object {
    pos : Vector2 <f32>,
    prev_pos : Vector2 <f32>,
    force : Vector2<f32>,
    mass : f32,
    radius : f32,
    render_object : CircleShape<'static>,
}

static RES_X : f32 = 800.0;
static RES_Y : f32 = 600.0;

static FPS : u32 = 1000;
static GRAVITY: Vector2<f32> = Vector2::new(0.0, 10000.0 as f32 * 9.8);
impl Object{
    fn get_velocity(&mut self) -> Vector2<f32> {
        return self.pos - self.prev_pos;
    }
    fn calc_collision(&mut self, other : &mut Object) {
        // check if the object is colliding with itself
        let mut distance = other.pos - self.pos;
        let max_radius = &other.radius + &self.radius;
        if distance.x.abs() < max_radius && distance.y.abs() < max_radius {
            let mut distance_length = distance.magnitude();
            if distance_length < max_radius {
                // incase the object is directly at the other objects position, extrude it up
                if distance_length == 0.0 {
                    distance_length = 0.0001;
                    distance = Vector2::new(0.0, 0.0001);
                }
                // new position calculation
                let distance_normalized = distance / distance_length;
                let distance_length_diff = other.radius + &self.radius - distance_length;
                let distance_length_diff_normalized = distance_normalized * distance_length_diff * 0.5;
                other.pos += distance_length_diff_normalized;
                self.pos -= distance_length_diff_normalized;
            }
        }
    }
    fn calc_physics(&mut self, dt: f32) {
        let temp_pos = self.pos;
        self.pos = self.pos * 2.0 - self.prev_pos + (GRAVITY + self.force / self.mass) * dt * dt;
        self.prev_pos = temp_pos;

        // box that bitch in
        self.pos = Vector2::new(self.pos.x.clamp(self.radius, RES_X - self.radius), self.pos.y.clamp(self.radius, RES_Y - self.radius));
        self.force = Vector2::zero();
    }
    fn new(pos : Vector2<f32>, mass : f32, radius : f32) -> Object {
        return Object {
            pos : pos,
            prev_pos : pos,
            force : Vector2::new(0.0, 0.0),
            mass : mass,
            radius : radius,
            render_object : CircleShape::new(radius as f32, 16),
        }
    }
}

fn main() {
    //let physics_iterations = 1;
    let inv_fps = 1.0 / FPS as f32;
    let mut window = RenderWindow::new((800, 600), "epic physics simulation v0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.1.0", sfml::window::Style::CLOSE, &Default::default());
    let mut objects : Vec<Object> = vec![];
    for x in 0..10 {
        for y in 0..10 {
            objects.push(Object::new(Vector2::new(x as f32, y as f32) * 14.0, x as f32 + 1.0, x as f32 + 1.0));
        }
    }

    objects.push(Object::new(Vector2::new(315.0, 20.0), 1.0, 5.0));

    let mut mouse_down = false;
    let mut mouse_pos = (0, 0);
    loop {
        window.clear(Color::BLACK);
        let objects_len = objects.len();

        // loop through all objects
        for object_index in 0 .. objects_len {
            // loop through all objects and see if they are colliding with this one
            //for _ in 0..physics_iterations {
                for collision_object_index in 0 .. objects_len {
                    if object_index == collision_object_index {continue}

                    // get both mutable objects by splitting the mutable
                    // if the object index is over the collision object index we must split it at the object index
                    // if the collision object has a greater index we must split it there
                    if object_index > collision_object_index {
                        let (head, tail) = objects.split_at_mut(object_index);
                        let object = &mut tail[0];
                        let collision_object = &mut head[collision_object_index];

                        object.calc_collision(collision_object);
                    } else {
                        let (head, tail) = objects.split_at_mut(collision_object_index);
                        let object = &mut head[object_index];
                        let collision_object = &mut tail[0];

                        object.calc_collision(collision_object);
                    }
                }
            //}
        }

        // calculate velocity from collision and apply
        for object in &mut objects {
            object.calc_physics(inv_fps);
            object.render_object.set_position(Vector2f::new(object.pos.x, object.pos.y) - Vector2f::new(object.radius, object.radius));
            object.render_object.set_fill_color(Color::rgb(255 - (object.pos.x / RES_X * 255.0) as u8, 255 - (object.pos.y / RES_Y * 255.0) as u8, 255));

            // finally, draw the object
            window.draw(&object.render_object);
        }

        window.display();
        match window.poll_event() {
            // kill loop on close event
            Some(Event::Closed) => {
                return;
            }
            Some(Event::MouseButtonPressed{..}) => {
                mouse_down = true;
            }
            Some(Event::MouseButtonReleased{..}) => {
                mouse_down = false;
            }
            Some(Event::MouseMoved{x, y}) => {
                mouse_pos = (x, y);
            }
            _ => {}     // ignore other events
        }

        if mouse_down {
            for object in &mut objects {
                let force = object.pos - Vector2::new(mouse_pos.0 as f32, mouse_pos.1 as f32);
                object.force = force * (1.0 / force.magnitude2());
                object.force *= 100000000.0;
            }
        }

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / FPS));
    }
}