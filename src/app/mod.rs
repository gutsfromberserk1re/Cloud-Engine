use eframe::egui;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use crate::editor::{explorer, inspector, toolbar, viewport};
use crate::theme;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool { Select, Move, Rotate, Scale }

impl Tool {
    pub fn label(self) -> &'static str {
        match self { Self::Select => "Select", Self::Move => "Move", Self::Rotate => "Rotate", Self::Scale => "Resize" }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Primitive { Cube, Sphere, Cylinder, Cone, Plane, Triangle }

impl Primitive {
    pub fn label(self) -> &'static str {
        match self {
            Self::Cube => "Cube", Self::Sphere => "Sphere", Self::Cylinder => "Cylinder",
            Self::Cone => "Cone", Self::Plane => "Plane", Self::Triangle => "Triangle",
        }
    }
    pub fn all() -> [Self; 6] { [Self::Cube, Self::Sphere, Self::Cylinder, Self::Cone, Self::Plane, Self::Triangle] }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }
impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn dot(self, rhs: Self) -> f32 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(self.y * rhs.z - self.z * rhs.y, self.z * rhs.x - self.x * rhs.z, self.x * rhs.y - self.y * rhs.x)
    }
    pub fn length(self) -> f32 { self.dot(self).sqrt() }
    pub fn normalized(self) -> Self { let l = self.length(); if l > 0.00001 { self / l } else { Self::ZERO } }
    pub fn component(self, axis: usize) -> f32 { match axis { 0 => self.x, 1 => self.y, _ => self.z } }
    pub fn set_component(&mut self, axis: usize, value: f32) { match axis { 0 => self.x = value, 1 => self.y = value, _ => self.z = value } }
}
impl Add for Vec3 { type Output = Self; fn add(self, r: Self) -> Self { Self::new(self.x+r.x,self.y+r.y,self.z+r.z) } }
impl AddAssign for Vec3 { fn add_assign(&mut self, r: Self) { *self = *self + r; } }
impl Sub for Vec3 { type Output = Self; fn sub(self, r: Self) -> Self { Self::new(self.x-r.x,self.y-r.y,self.z-r.z) } }
impl SubAssign for Vec3 { fn sub_assign(&mut self, r: Self) { *self = *self - r; } }
impl Mul<f32> for Vec3 { type Output = Self; fn mul(self, s: f32) -> Self { Self::new(self.x*s,self.y*s,self.z*s) } }
impl Div<f32> for Vec3 { type Output = Self; fn div(self, s: f32) -> Self { Self::new(self.x/s,self.y/s,self.z/s) } }
impl Neg for Vec3 { type Output = Self; fn neg(self) -> Self { self * -1.0_f32 } }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selection { None, Object(u32), Folder(u32) }

#[derive(Clone)]
pub struct Folder { pub id: u32, pub name: String, pub parent: Option<u32> }

#[derive(Clone)]
pub struct SceneObject {
    pub id: u32,
    pub name: String,
    pub primitive: Primitive,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub parent: Option<u32>,
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub move_speed: f32,
}
impl Default for Camera {
    fn default() -> Self { Self { position: Vec3::new(0.0, 4.0, 10.0), yaw: 0.0, pitch: -0.38, move_speed: 7.0 } }
}
impl Camera {
    pub fn forward(self) -> Vec3 {
        Vec3::new(self.yaw.sin() * self.pitch.cos(), self.pitch.sin(), -self.yaw.cos() * self.pitch.cos()).normalized()
    }
    pub fn right(self) -> Vec3 { Vec3::new(self.yaw.cos(), 0.0, self.yaw.sin()).normalized() }
    pub fn up(self) -> Vec3 { self.right().cross(self.forward()).normalized() }
}

#[derive(Clone, Copy)]
pub enum TransformDrag {
    Plane { object_id: u32, plane_point: Vec3, plane_normal: Vec3, start_position: Vec3, start_hit: Vec3 },
    AxisMove { object_id: u32, axis: Vec3, start_position: Vec3, start_mouse: egui::Pos2, screen_axis: egui::Vec2, pixels_per_unit: f32 },
    AxisScale { object_id: u32, axis_index: usize, start_scale: Vec3, start_mouse: egui::Pos2, screen_axis: egui::Vec2 },
    Rotate { object_id: u32, axis_index: usize, start_rotation: Vec3, start_mouse: egui::Pos2 },
    UniformScale { object_id: u32, start_scale: Vec3, start_mouse: egui::Pos2 },
}

pub struct KumoApp {
    pub active_tool: Tool,
    pub selection: Selection,
    pub objects: Vec<SceneObject>,
    pub folders: Vec<Folder>,
    pub expanded_folders: Vec<u32>,
    pub camera: Camera,
    pub drag: Option<TransformDrag>,
    pub looking: bool,
    pub next_id: u32,
    pub show_grid: bool,
    pub show_gizmos: bool,
    pub show_explorer: bool,
    pub show_inspector: bool,
    pub show_console: bool,
    pub renaming_folder: Option<u32>,
    pub rename_buffer: String,
    pub logs: Vec<String>,
    pub status: String,
}

impl Default for KumoApp {
    fn default() -> Self {
        Self {
            active_tool: Tool::Select,
            selection: Selection::None,
            objects: Vec::new(),
            folders: Vec::new(),
            expanded_folders: Vec::new(),
            camera: Camera::default(),
            drag: None,
            looking: false,
            next_id: 1,
            show_grid: true,
            show_gizmos: true,
            show_explorer: true,
            show_inspector: true,
            show_console: true,
            renaming_folder: None,
            rename_buffer: String::new(),
            logs: vec!["Kumo scene editor ready. Create a folder or insert a primitive to begin.".to_owned()],
            status: "Ready · Empty scene".to_owned(),
        }
    }
}

impl KumoApp {
    pub fn log(&mut self, message: impl Into<String>) {
        self.logs.push(message.into());
        if self.logs.len() > 250 { self.logs.remove(0); }
    }
    pub fn insert_part(&mut self, primitive: Primitive) {
        let parent = match self.selection { Selection::Folder(id) => Some(id), _ => None };
        let id = self.next_id;
        self.next_id += 1;
        let name = format!("{}_{:03}", primitive.label(), id);
        let placement = self.objects.len() as f32 * 1.5_f32;
        let position = Vec3::new(placement, if primitive == Primitive::Plane { 0.0 } else { 0.5 }, 0.0);
        self.objects.push(SceneObject { id, name: name.clone(), primitive, position, rotation: Vec3::ZERO, scale: Vec3::new(1.0,1.0,1.0), parent });
        self.selection = Selection::Object(id);
        self.status = format!("Created {name}");
        self.log(format!("Created {name}"));
    }
    pub fn create_folder(&mut self, parent: Option<u32>) {
        let id = self.next_id;
        self.next_id += 1;
        let base = "New Folder";
        let mut suffix = 1;
        let name = loop {
            let candidate = if suffix == 1 { base.to_owned() } else { format!("{base} {suffix}") };
            if !self.folders.iter().any(|f| f.parent == parent && f.name == candidate) { break candidate; }
            suffix += 1;
        };
        self.folders.push(Folder { id, name: name.clone(), parent });
        if let Some(parent_id) = parent { if !self.expanded_folders.contains(&parent_id) { self.expanded_folders.push(parent_id); } }
        self.expanded_folders.push(id);
        self.selection = Selection::Folder(id);
        self.status = format!("Created folder {name}");
        self.log(format!("Created folder {name}"));
    }
    pub fn selected_object(&self) -> Option<&SceneObject> {
        match self.selection { Selection::Object(id) => self.objects.iter().find(|o| o.id == id), _ => None }
    }
    pub fn selected_object_mut(&mut self) -> Option<&mut SceneObject> {
        let id = match self.selection { Selection::Object(id) => id, _ => return None };
        self.objects.iter_mut().find(|o| o.id == id)
    }
    pub fn selected_folder(&self) -> Option<&Folder> {
        match self.selection { Selection::Folder(id) => self.folders.iter().find(|f| f.id == id), _ => None }
    }
    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
        self.drag = None;
        self.status = format!("{} tool", tool.label());
    }
    pub fn delete_selection(&mut self) {
        match self.selection {
            Selection::Object(id) => {
                if let Some(index) = self.objects.iter().position(|o| o.id == id) {
                    let name = self.objects.remove(index).name;
                    self.log(format!("Deleted {name}")); self.status = format!("Deleted {name}");
                }
            }
            Selection::Folder(id) => {
                let mut remove = vec![id];
                let mut cursor = 0;
                while cursor < remove.len() {
                    let parent = remove[cursor];
                    for child in self.folders.iter().filter(|f| f.parent == Some(parent)) {
                        if !remove.contains(&child.id) { remove.push(child.id); }
                    }
                    cursor += 1;
                }
                self.objects.retain(|o| !o.parent.map(|p| remove.contains(&p)).unwrap_or(false));
                self.folders.retain(|f| !remove.contains(&f.id));
                self.expanded_folders.retain(|id| !remove.contains(id));
                self.log("Deleted folder and its contents"); self.status = "Deleted folder".to_owned();
            }
            Selection::None => return,
        }
        self.selection = Selection::None;
        self.drag = None;
    }
    pub fn duplicate_selection(&mut self) {
        let source = match self.selection { Selection::Object(id) => self.objects.iter().find(|o| o.id == id).cloned(), _ => None };
        if let Some(mut object) = source {
            object.id = self.next_id; self.next_id += 1;
            object.name = format!("{}_copy", object.name);
            object.position.x += 1.0;
            let name = object.name.clone(); let id = object.id;
            self.objects.push(object); self.selection = Selection::Object(id);
            self.log(format!("Duplicated {name}")); self.status = format!("Duplicated {name}");
        }
    }
    pub fn focus_selection(&mut self) {
        let target = self.selected_object().map(|object| (object.position, object.name.clone()));
        if let Some((position, name)) = target {
            self.camera.position = position + Vec3::new(0.0, 3.0, 8.0);
            self.camera.yaw = 0.0; self.camera.pitch = -0.30;
            self.status = format!("Focused {name}");
        }
    }
    pub fn new_scene(&mut self) {
        self.objects.clear(); self.folders.clear(); self.expanded_folders.clear(); self.selection = Selection::None;
        self.drag = None; self.looking = false; self.renaming_folder = None; self.rename_buffer.clear(); self.next_id = 1; self.camera = Camera::default();
        self.log("New empty scene"); self.status = "New empty scene".to_owned();
    }
    pub fn save_scene(&mut self) {
        let mut out = String::from("KUMO_SCENE|1\n");
        let c = self.camera;
        out.push_str(&format!("C|{}|{}|{}|{}|{}\n", c.position.x,c.position.y,c.position.z,c.yaw,c.pitch));
        for f in &self.folders {
            out.push_str(&format!("F|{}|{}|{}\n", f.id, f.parent.map(|v| v.to_string()).unwrap_or_else(|| "-".to_owned()), encode_field(&f.name)));
        }
        for o in &self.objects {
            out.push_str(&format!("O|{}|{}|{}|{}|{},{},{}|{},{},{}|{},{},{}\n", o.id,
                o.parent.map(|v| v.to_string()).unwrap_or_else(|| "-".to_owned()), o.primitive.label(), encode_field(&o.name),
                o.position.x,o.position.y,o.position.z,o.rotation.x,o.rotation.y,o.rotation.z,o.scale.x,o.scale.y,o.scale.z));
        }
        match std::fs::write("scene.kumo", out) {
            Ok(()) => { self.status = "Saved scene.kumo".to_owned(); self.log("Saved scene.kumo"); }
            Err(e) => { self.status = format!("Save failed: {e}"); self.log(format!("Save failed: {e}")); }
        }
    }
    pub fn load_scene(&mut self) {
        let contents = match std::fs::read_to_string("scene.kumo") {
            Ok(v) => v,
            Err(e) => { let message = format!("Could not load scene.kumo: {e}"); self.status = message.clone(); self.log(message); return; }
        };
        let mut objects = Vec::new(); let mut folders = Vec::new(); let mut camera = Camera::default(); let mut max_id = 0_u32;
        for line in contents.lines().skip(1) {
            let p: Vec<&str> = line.split('|').collect();
            if p.first() == Some(&"C") && p.len() == 6 {
                if let (Ok(x),Ok(y),Ok(z),Ok(yaw),Ok(pitch)) = (p[1].parse(),p[2].parse(),p[3].parse(),p[4].parse(),p[5].parse()) {
                    camera.position = Vec3::new(x,y,z); camera.yaw = yaw; camera.pitch = pitch;
                }
            } else if p.first() == Some(&"F") && p.len() == 4 {
                if let Ok(id) = p[1].parse::<u32>() {
                    let parent = if p[2] == "-" { None } else { p[2].parse().ok() };
                    folders.push(Folder { id, name: decode_field(p[3]), parent }); max_id = max_id.max(id);
                }
            } else if p.first() == Some(&"O") && p.len() == 8 {
                if let Ok(id) = p[1].parse::<u32>() {
                    let parent = if p[2] == "-" { None } else { p[2].parse().ok() };
                    let primitive = match p[3] { "Cube"=>Primitive::Cube,"Sphere"=>Primitive::Sphere,"Cylinder"=>Primitive::Cylinder,"Cone"=>Primitive::Cone,"Plane"=>Primitive::Plane,"Triangle"=>Primitive::Triangle,_=>continue };
                    let parse_vec = |s: &str| -> Option<Vec3> { let a: Vec<f32> = s.split(',').map(str::parse).collect::<Result<_,_>>().ok()?; if a.len()==3 { Some(Vec3::new(a[0],a[1],a[2])) } else { None } };
                    if let (Some(position),Some(rotation),Some(scale)) = (parse_vec(p[5]),parse_vec(p[6]),parse_vec(p[7])) {
                        objects.push(SceneObject { id, name: decode_field(p[4]), primitive, position, rotation, scale, parent }); max_id = max_id.max(id);
                    }
                }
            }
        }
        self.objects = objects; self.folders = folders; self.expanded_folders = self.folders.iter().map(|f| f.id).collect(); self.camera = camera; self.next_id = max_id + 1;
        self.selection = Selection::None; self.drag = None; self.looking = false; self.renaming_folder = None;
        self.status = "Loaded scene.kumo".to_owned(); self.log("Loaded scene.kumo");
    }
}

fn encode_field(value: &str) -> String { value.replace('%', "%25").replace('|', "%7C").replace('\n', "%0A") }
fn decode_field(value: &str) -> String { value.replace("%0A", "\n").replace("%7C", "|").replace("%25", "%") }

impl eframe::App for KumoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::apply_visuals(ctx);
        toolbar::top_bar(ctx, self);

        if self.show_explorer {
            egui::SidePanel::left("explorer_panel").resizable(true).default_width(225.0)
                .min_width(155.0).max_width(380.0).show(ctx, |ui| explorer::show(ui, self));
        }
        if self.show_inspector {
            egui::SidePanel::right("inspector_panel").resizable(true).default_width(285.0)
                .min_width(220.0).max_width(430.0).show(ctx, |ui| inspector::show(ui, self));
        }
        egui::TopBottomPanel::bottom("status_bar").exact_height(23.0).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(egui::RichText::new(&self.status).color(theme::MUTED));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("{} objects  ·  {} folders", self.objects.len(), self.folders.len())).color(theme::MUTED));
                });
            });
        });
        if self.show_console {
            egui::TopBottomPanel::bottom("console_panel").resizable(true).default_height(145.0)
                .min_height(65.0).max_height(300.0).show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.strong("⌘  CONSOLE");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("×").on_hover_text("Hide console").clicked() { self.show_console = false; }
                            if ui.small_button("Clear").clicked() { self.logs.clear(); }
                        });
                    });
                    ui.separator();
                    egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                        for line in &self.logs { ui.monospace(line); }
                    });
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| viewport::show(ui, self));
        ctx.request_repaint();
    }
}
