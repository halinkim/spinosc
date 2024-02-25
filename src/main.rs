use nannou::prelude::*;
use nannou_egui::{egui, Egui};

const PI: f32 = 3.14159265358979323846264338327950288;
fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    t: f32,
    dt: f32,
    omega: f32,
    oomega_0: f32,
    oomega_1: f32,
    traj: Vec<Vec3>,
    tail: usize,
    sphere: Vec<SphereFrame>,
    camera: Camera,
    egui: Egui
}

struct Camera {
    pos: Vec3,
    ahead: Vec3,
    canvas_x: Vec3,
    canvas_y: Vec3,
    r: f32,
    theta: f32,
    phi: f32
}
struct SphereFrame {
    points: Vec<Vec3>
}
impl Camera {
    fn to_canvas(&self, v: Vec3) -> Vec2 {
        let p = v - v.dot(self.pos) / self.pos.length().powi(2) * self.pos;
        vec2(p.dot(self.canvas_x), p.dot(self.canvas_y))
    }
    fn init_pos(&mut self) {
        self.pos = vec3(self.r * self.theta.to_radians().cos() * self.phi.to_radians().cos(),
            self.r * self.theta.to_radians().cos() * self.phi.to_radians().sin(),
            self.r * self.theta.to_radians().sin());
        let head = self.ahead - self.pos.dot(self.ahead) / self.pos.length().powi(2) * self.pos;
        self.canvas_x = head.cross(self.pos) / head.cross(self.pos).length();
        self.canvas_y = head / head.length();
    }
}

fn model(app: &App) -> Model {
    let window_id = app.new_window()
        .size(900, 600)
        .raw_event(raw_window_event)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let mut sphere = Vec::new();
    let sphere_poly = 100;
    for i in -4..=4 {
        let mut points = Vec::new();
        let phi = PI / 9.0 * i as f32;
        let r = phi.cos();
        for j in 0..=sphere_poly {
            let theta = 2.0 * PI * (j as f32 / sphere_poly as f32);
            points.push(vec3(r * theta.cos(), r * theta.sin(), phi.sin()));
        }
        sphere.push(SphereFrame {
            points
        });
    }
    {
        let mut points = Vec::new();
        for j in 0..=sphere_poly {
            let theta = 2.0 * PI * (j as f32 / sphere_poly as f32);
            points.push(vec3(0.0, theta.cos(), theta.sin()));
        }
        sphere.push(SphereFrame {
            points
        });
    }
    {
        let mut points = Vec::new();
        for j in 0..=sphere_poly {
            let theta = 2.0 * PI * (j as f32 / sphere_poly as f32);
            points.push(vec3(theta.cos(), 0.0, theta.sin()));
        }
        sphere.push(SphereFrame {
            points
        });
    }


    let pos = vec3(10.0, 10.0, 10.0);
    let ahead = vec3(0.0, 0.0, 1.0);
    let head = ahead - pos.dot(ahead) / pos.length().powi(2) * pos;
    let canvas_x = head.cross(pos) / head.cross(pos).length();
    let canvas_y = head / head.length();


    Model {
        t: 0.0,
        dt: 0.01, 
        omega: 0.5,
        oomega_0: 0.5,
        oomega_1: 0.05,
        traj: Vec::new(),
        tail: 1000,
        sphere,
        camera: Camera {
            pos,
            ahead,
            canvas_x,
            canvas_y,
            r: 200.0,
            theta: 45.0,
            phi: 45.0
        },
        egui: Egui::from_window(&window)
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("dt:");
        ui.add(egui::Slider::new(&mut model.dt, 0.0..=0.1));
        ui.label("tail");
        ui.add(egui::Slider::new(&mut model.tail, 1..=5000));
        let clicked = ui.button("reset traj").clicked();
        if clicked {
            model.traj.clear();
        }
        ui.label("omega");
        ui.add(egui::Slider::new(&mut model.omega, 0.01..=1.0));
        ui.label("Omega_0");
        ui.add(egui::Slider::new(&mut model.oomega_0, 0.01..=1.0));
        ui.label("Omega_1");
        ui.add(egui::Slider::new(&mut model.oomega_1, 0.01..=1.0));
        ui.label("camera");
        ui.label("scale");
        ui.add(egui::Slider::new(&mut model.camera.r, 100.0..=400.0));
        ui.label("theta");
        ui.add(egui::Slider::new(&mut model.camera.theta, -89.99..=89.99));
        ui.label("phi");
        ui.add(egui::Slider::new(&mut model.camera.phi, 0.0..=360.0));
        
    });

    model.camera.init_pos();
    
    let spin = {
        let (omega, oomega_0, oomega_1) = (model.omega, model.oomega_0, model.oomega_1);
        let delta = omega - oomega_0;
        let oomega = (delta * delta + oomega_1 * oomega_1).sqrt();
        let (omega_1, omega_2) = (oomega / oomega_1, delta / oomega_1);
        let t = model.t;
        let a_t_r = (omega_1 + omega_2) / 2.0 / omega_1 * (0.5 * t * oomega).cos()
            + (omega_1 - omega_2) / 2.0 / omega_1 * (-0.5 * t * oomega).cos();
        let a_t_i = (omega_1 + omega_2) / 2.0 / omega_1 * (0.5 * t * oomega).sin()
        + (omega_1 - omega_2) / 2.0 / omega_1 * (-0.5 * t * oomega).sin();
        let b_t_r = -(omega_1 - omega_2) * (omega_1 + omega_2) / 2.0 / omega_1 * (0.5 * t * oomega).cos()
            + (omega_1 + omega_2) * (omega_1 - omega_2) / 2.0 / omega_1 * (-0.5 * t * oomega).cos();
        let b_t_i = -(omega_1 - omega_2) * (omega_1 + omega_2) / 2.0 / omega_1 * (0.5 * t * oomega).sin()
            + (omega_1 + omega_2) * (omega_1 - omega_2) / 2.0 / omega_1 * (-0.5 * t * oomega).sin();
        let aa_t_r = a_t_r * (-0.5 * omega * t).cos() - a_t_i * (-0.5 * omega * t).sin();
        let aa_t_i = a_t_i * (-0.5 * omega * t).cos() + a_t_r * (-0.5 * omega * t).sin();
        let bb_t_r = b_t_r * (0.5 * omega * t).cos() - b_t_i * (0.5 * omega * t).sin();
        let bb_t_i = b_t_i * (0.5 * omega * t).cos() + b_t_r * (0.5 * omega * t).sin();
        let s_x = (aa_t_r * bb_t_r + aa_t_i * bb_t_i) * 2.0;
        let s_y = (aa_t_r * bb_t_i - aa_t_i * bb_t_r) * 2.0;
        let s_z = aa_t_r * aa_t_r + aa_t_i * aa_t_i - bb_t_r * bb_t_r - bb_t_i * bb_t_i;
        vec3(s_x, s_y, s_z)
    };
    model.traj.push(spin);
    if model.traj.len() > model.tail {
        model.traj = model.traj[model.traj.len() - model.tail..].to_vec();
    }
    model.t += model.dt;
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let scale = model.camera.r;

    for sphere_frame in model.sphere.iter() {
        for i in 0..sphere_frame.points.len() - 1 {
            let dist = (sphere_frame.points[i] - model.camera.pos).length() - model.camera.pos.length() + 1.0;
            draw.line()
                .start(scale * model.camera.to_canvas(sphere_frame.points[i]))
                .end(scale * model.camera.to_canvas(sphere_frame.points[i + 1]))
                .rgba(0.0, 0.0, 0.0, 1.0 - dist / 2.0);
        }
    }

    for i in 0..model.traj.len() - 1 {
        draw.line()
            .start(scale * model.camera.to_canvas(model.traj[i]))
            .end(scale * model.camera.to_canvas(model.traj[i + 1]))
            .color(BLUE);
    }
    let dist_size = (model.traj[model.traj.len() - 1] - model.camera.pos).length() - model.camera.pos.length();

    draw.ellipse()
        .xy(scale * model.camera.to_canvas(model.traj[model.traj.len() - 1]))
        .radius(2.0 - dist_size)
        .color(BLUE);


    draw.to_frame(app, &frame).unwrap();

    model.egui.draw_to_frame(&frame).unwrap();
}
