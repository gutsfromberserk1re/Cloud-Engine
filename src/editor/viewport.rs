use eframe::egui;
use std::f32::consts::TAU;

use crate::app::{Camera, KumoApp, Primitive, SceneObject, Selection, Tool, TransformDrag, Vec3};
use crate::theme;

#[derive(Clone)]
struct ProjectedObject { id: u32, bounds: egui::Rect, depth: f32 }

pub fn show(ui: &mut egui::Ui, app: &mut KumoApp) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("◈  SCENE").color(theme::PHOSPHOR).strong());
        ui.separator();
        ui.label(egui::RichText::new("Perspective · Shaded").small().color(theme::MUTED));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(format!("{}", app.active_tool.label())).small().color(theme::MUTED));
        });
    });
    ui.separator();
    let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
    if rect.width() < 10.0 || rect.height() < 10.0 { return; }

    handle_camera_input(ui.ctx(), rect, response.hovered(), app);
    handle_shortcuts(ui.ctx(), app);

    let projected: Vec<ProjectedObject> = app.objects.iter().filter_map(|object| {
        let bounds = object_bounds(object, app.camera, rect)?;
        let depth = project(object.position, app.camera, rect)?.1;
        Some(ProjectedObject { id: object.id, bounds, depth })
    }).collect();

    handle_mouse_selection_and_drag(ui.ctx(), rect, response.hovered(), &projected, app);

    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(31, 36, 43));
    if app.show_grid { draw_grid(&painter, rect, app.camera); }

    let mut draw_order: Vec<(f32, u32)> = projected.iter().map(|p| (p.depth, p.id)).collect();
    draw_order.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (_, id) in draw_order {
        if let Some(object) = app.objects.iter().find(|o| o.id == id) {
            draw_object(&painter, rect, app.camera, object, app.selection == Selection::Object(id));
        }
    }
    if app.show_gizmos {
        if let Some(object) = app.selected_object() {
            draw_gizmo(&painter, rect, app.camera, object, app.active_tool);
        }
    }

    painter.text(rect.left_top() + egui::vec2(12.0, 10.0), egui::Align2::LEFT_TOP,
        "SCENE VIEW  ·  RMB LOOK  ·  WASD MOVE  ·  WHEEL DOLLY", egui::FontId::monospace(10.5), egui::Color32::from_gray(175));
    if app.objects.is_empty() {
        painter.text(rect.center(), egui::Align2::CENTER_CENTER,
            "No geometry in scene\nUse Insert to add a primitive", egui::FontId::monospace(13.0), egui::Color32::from_gray(130));
    }
    if let Some(object) = app.selected_object() {
        painter.text(rect.left_bottom() + egui::vec2(12.0, -10.0), egui::Align2::LEFT_BOTTOM,
            format!("{}  ·  {:?}", object.name, app.active_tool), egui::FontId::monospace(10.0), egui::Color32::from_gray(200));
    }
}

fn handle_camera_input(ctx: &egui::Context, rect: egui::Rect, hovered: bool, app: &mut KumoApp) {
    let (pointer, secondary_pressed, secondary_released, secondary_down, delta, scroll, dt, w,a,s,d,q,e,up,down,left,right,shift) = ctx.input(|i| (
        i.pointer.interact_pos(), i.pointer.button_pressed(egui::PointerButton::Secondary),
        i.pointer.button_released(egui::PointerButton::Secondary), i.pointer.button_down(egui::PointerButton::Secondary),
        i.pointer.delta(), i.raw_scroll_delta.y, i.stable_dt, i.key_down(egui::Key::W), i.key_down(egui::Key::A),
        i.key_down(egui::Key::S), i.key_down(egui::Key::D), i.key_down(egui::Key::Q), i.key_down(egui::Key::E),
        i.key_down(egui::Key::ArrowUp), i.key_down(egui::Key::ArrowDown), i.key_down(egui::Key::ArrowLeft),
        i.key_down(egui::Key::ArrowRight), i.modifiers.shift
    ));
    if secondary_pressed && hovered { app.looking = true; app.drag = None; }
    if secondary_released { app.looking = false; }
    if app.looking && secondary_down {
        app.camera.yaw += delta.x * 0.0045_f32;
        app.camera.pitch = (app.camera.pitch - delta.y * 0.0045_f32).clamp(-1.52_f32, 1.52_f32);
    }
    if hovered && !app.looking {
        if scroll.abs() > 0.0 { app.camera.position += app.camera.forward() * (scroll * 0.008_f32); }
    }
    if (hovered || app.looking) && !ctx.wants_keyboard_input() {
        let forward = (w || up) as i32 as f32 - (s || down) as i32 as f32;
        let strafe = (d || right) as i32 as f32 - (a || left) as i32 as f32;
        let vertical = e as i32 as f32 - q as i32 as f32;
        if forward != 0.0 || strafe != 0.0 || vertical != 0.0 {
            let speed = app.camera.move_speed * if shift { 3.0_f32 } else { 1.0_f32 } * dt.min(0.05_f32);
            let mut movement = app.camera.forward() * forward + app.camera.right() * strafe + Vec3::Y * vertical;
            movement = movement.normalized() * speed;
            app.camera.position += movement;
        }
    }
    let _ = (pointer, rect);
}

fn handle_shortcuts(ctx: &egui::Context, app: &mut KumoApp) {
    let (n1,n2,n3,n4,delete,escape,focus,duplicate,save,new_scene,ctrl) = ctx.input(|i| (
        i.key_pressed(egui::Key::Num1),i.key_pressed(egui::Key::Num2),i.key_pressed(egui::Key::Num3),i.key_pressed(egui::Key::Num4),
        i.key_pressed(egui::Key::Delete),i.key_pressed(egui::Key::Escape),i.key_pressed(egui::Key::F),i.key_pressed(egui::Key::D),
        i.key_pressed(egui::Key::S),i.key_pressed(egui::Key::N),i.modifiers.ctrl
    ));
    if ctx.wants_keyboard_input() { return; }
    if n1 { app.set_tool(Tool::Select); }
    if n2 { app.set_tool(Tool::Move); }
    if n3 { app.set_tool(Tool::Rotate); }
    if n4 { app.set_tool(Tool::Scale); }
    if delete { app.delete_selection(); }
    if escape { app.selection = Selection::None; app.drag = None; }
    if focus { app.focus_selection(); }
    if duplicate && ctrl { app.duplicate_selection(); }
    if save && ctrl { app.save_scene(); }
    if new_scene && ctrl { app.new_scene(); }
}

fn handle_mouse_selection_and_drag(ctx: &egui::Context, rect: egui::Rect, hovered: bool, projected: &[ProjectedObject], app: &mut KumoApp) {
    let (pointer, pressed, down, released, secondary_down) = ctx.input(|i| (
        i.pointer.interact_pos(), i.pointer.button_pressed(egui::PointerButton::Primary),
        i.pointer.button_down(egui::PointerButton::Primary), i.pointer.button_released(egui::PointerButton::Primary),
        i.pointer.button_down(egui::PointerButton::Secondary)
    ));
    if secondary_down { return; }
    if pressed && hovered {
        if let Some(mouse) = pointer.filter(|p| rect.contains(*p)) {
            let selected_id = match app.selection { Selection::Object(id) => Some(id), _ => None };
            let selected_hit = selected_id.and_then(|id| projected.iter().find(|p| p.id == id && p.bounds.contains(mouse)).map(|_| id));
            let mut started_gizmo = false;
            if let Some(id) = selected_id {
                if let Some(object) = app.objects.iter().find(|o| o.id == id) {
                    if app.show_gizmos {
                        match app.active_tool {
                            Tool::Move => if let Some((axis_index, screen_axis, pixels_per_unit)) = pick_axis(mouse, object, app.camera, rect, 1.55_f32) {
                                app.drag = Some(TransformDrag::AxisMove { object_id: id, axis: axis_vector(axis_index), start_position: object.position, start_mouse: mouse, screen_axis, pixels_per_unit }); started_gizmo = true;
                            },
                            Tool::Scale => if let Some((axis_index, screen_axis, _)) = pick_axis(mouse, object, app.camera, rect, 1.55_f32) {
                                app.drag = Some(TransformDrag::AxisScale { object_id: id, axis_index, start_scale: object.scale, start_mouse: mouse, screen_axis }); started_gizmo = true;
                            },
                            Tool::Rotate => {
                                if let Some((center, _)) = project(object.position, app.camera, rect) {
                                    let distance = (mouse - center).length();
                                    let radii = [31.0_f32, 43.0_f32, 55.0_f32];
                                    if let Some((axis_index, nearest)) = radii.iter().enumerate().map(|(i,r)|(i,(distance-*r).abs())).min_by(|a,b|a.1.total_cmp(&b.1)) {
                                        if nearest <= 9.0_f32 {
                                            app.drag = Some(TransformDrag::Rotate { object_id: id, axis_index, start_rotation: object.rotation, start_mouse: mouse }); started_gizmo = true;
                                        }
                                    }
                                }
                            }
                            Tool::Select => {}
                        }
                    }
                }
            }
            if !started_gizmo {
                let hit = projected.iter().filter(|p| p.bounds.contains(mouse)).min_by(|a,b| a.depth.total_cmp(&b.depth)).map(|p| p.id);
                if let Some(id) = hit {
                    app.selection = Selection::Object(id);
                    if let Some(object) = app.objects.iter().find(|o| o.id == id) {
                        match app.active_tool {
                            Tool::Select | Tool::Move => {
                                if let Some(start_hit) = ray_plane_hit(mouse, rect, app.camera, object.position, app.camera.forward()) {
                                    app.drag = Some(TransformDrag::Plane { object_id: id, plane_point: object.position, plane_normal: app.camera.forward(), start_position: object.position, start_hit });
                                }
                            }
                            Tool::Rotate => app.drag = Some(TransformDrag::Rotate { object_id: id, axis_index: 1, start_rotation: object.rotation, start_mouse: mouse }),
                            Tool::Scale => app.drag = Some(TransformDrag::UniformScale { object_id: id, start_scale: object.scale, start_mouse: mouse }),
                        }
                    }
                    app.status = format!("Selected object {}", id);
                } else if !selected_hit.is_some() {
                    app.selection = Selection::None; app.drag = None;
                }
            }
        }
    }
    if down {
        if let Some(mouse) = pointer {
            if let Some(drag) = app.drag {
                match drag {
                    TransformDrag::Plane { object_id, plane_point, plane_normal, start_position, start_hit } => {
                        if let Some(hit) = ray_plane_hit(mouse, rect, app.camera, plane_point, plane_normal) {
                            if let Some(object) = app.objects.iter_mut().find(|o| o.id == object_id) { object.position = start_position + (hit - start_hit); }
                        }
                    }
                    TransformDrag::AxisMove { object_id, axis, start_position, start_mouse, screen_axis, pixels_per_unit } => {
                        let delta = (mouse - start_mouse).dot(screen_axis) / pixels_per_unit.max(0.0001_f32);
                        if let Some(object) = app.objects.iter_mut().find(|o| o.id == object_id) { object.position = start_position + axis * delta; }
                    }
                    TransformDrag::AxisScale { object_id, axis_index, start_scale, start_mouse, screen_axis } => {
                        let delta = (mouse - start_mouse).dot(screen_axis) * 0.01_f32;
                        if let Some(object) = app.objects.iter_mut().find(|o| o.id == object_id) {
                            object.scale = start_scale; object.scale.set_component(axis_index, (start_scale.component(axis_index) + delta).max(0.01_f32));
                        }
                    }
                    TransformDrag::Rotate { object_id, axis_index, start_rotation, start_mouse } => {
                        let delta = mouse - start_mouse;
                        let amount = if axis_index == 0 { delta.y } else { delta.x };
                        if let Some(object) = app.objects.iter_mut().find(|o| o.id == object_id) {
                            object.rotation = start_rotation;
                            object.rotation.set_component(axis_index, start_rotation.component(axis_index) + amount * 0.7_f32);
                        }
                    }
                    TransformDrag::UniformScale { object_id, start_scale, start_mouse } => {
                        let delta = mouse - start_mouse;
                        let factor = (1.0_f32 + (delta.x - delta.y) * 0.01_f32).max(0.01_f32);
                        if let Some(object) = app.objects.iter_mut().find(|o| o.id == object_id) { object.scale = start_scale * factor; }
                    }
                }
            }
        }
    }
    if released { app.drag = None; }
}

fn draw_grid(painter: &egui::Painter, rect: egui::Rect, camera: Camera) {
    let limit = 60;
    for i in -limit..=limit {
        let major = i % 5 == 0;
        let color = if major { egui::Color32::from_rgba_unmultiplied(170,185,198,58) } else { egui::Color32::from_rgba_unmultiplied(145,160,175,28) };
        let x = i as f32;
        draw_world_line(painter, rect, camera, Vec3::new(x,0.0,-60.0), Vec3::new(x,0.0,60.0), egui::Stroke::new(if major {1.0_f32}else{0.65_f32},color));
        let z = i as f32;
        draw_world_line(painter, rect, camera, Vec3::new(-60.0,0.0,z), Vec3::new(60.0,0.0,z), egui::Stroke::new(if major {1.0_f32}else{0.65_f32},color));
    }
    draw_world_line(painter,rect,camera,Vec3::new(-60.0,0.0,0.0),Vec3::new(60.0,0.0,0.0),egui::Stroke::new(1.4_f32,egui::Color32::from_rgba_unmultiplied(80,145,220,110)));
    draw_world_line(painter,rect,camera,Vec3::new(0.0,0.0,-60.0),Vec3::new(0.0,0.0,60.0),egui::Stroke::new(1.4_f32,egui::Color32::from_rgba_unmultiplied(80,190,145,110)));
}

fn draw_world_line(painter: &egui::Painter, rect: egui::Rect, camera: Camera, mut a: Vec3, mut b: Vec3, stroke: egui::Stroke) {
    let forward = camera.forward();
    let depth_a = (a-camera.position).dot(forward);
    let depth_b = (b-camera.position).dot(forward);
    let near = 0.08_f32;
    if depth_a <= near && depth_b <= near { return; }
    if depth_a <= near {
        let t = (near-depth_a)/(depth_b-depth_a);
        a = a+(b-a)*t;
    } else if depth_b <= near {
        let t = (near-depth_b)/(depth_a-depth_b);
        b = b+(a-b)*t;
    }
    if let (Some((pa,_)),Some((pb,_))) = (project(a,camera,rect),project(b,camera,rect)) {
        painter.line_segment([pa,pb],stroke);
    }
}

fn draw_object(painter: &egui::Painter, rect: egui::Rect, camera: Camera, object: &SceneObject, selected: bool) {
    let local_faces = mesh_faces(object.primitive);
    let mut faces: Vec<(f32, Vec<egui::Pos2>, egui::Color32)> = Vec::new();
    for (local, shade) in local_faces {
        let world: Vec<Vec3> = local.into_iter().map(|v| transform_point(v, object)).collect();
        let mut screen = Vec::with_capacity(world.len()); let mut depth_sum = 0.0_f32; let mut valid = true;
        for point in world {
            if let Some((pos,depth)) = project(point,camera,rect) { screen.push(pos); depth_sum += depth; }
            else { valid = false; break; }
        }
        if valid && screen.len() >= 3 { faces.push((depth_sum / screen.len() as f32, screen, shade)); }
    }
    faces.sort_by(|a,b| b.0.total_cmp(&a.0));
    for (_, points, shade) in faces {
        let stroke = if selected { egui::Stroke::new(2.0_f32,egui::Color32::WHITE) } else { egui::Stroke::new(0.8_f32,egui::Color32::from_rgb(33,42,53)) };
        let fill = if selected { shade.gamma_multiply(1.25_f32) } else { shade };
        painter.add(egui::Shape::convex_polygon(points,fill,stroke));
    }
}

fn mesh_faces(primitive: Primitive) -> Vec<(Vec<Vec3>,egui::Color32)> {
    let dark = egui::Color32::from_rgb(91,103,116);
    let mid = egui::Color32::from_rgb(129,141,153);
    let light = egui::Color32::from_rgb(165,176,186);
    match primitive {
        Primitive::Cube => {
            let v = [Vec3::new(-0.5,-0.5,-0.5),Vec3::new(0.5,-0.5,-0.5),Vec3::new(0.5,0.5,-0.5),Vec3::new(-0.5,0.5,-0.5),
                Vec3::new(-0.5,-0.5,0.5),Vec3::new(0.5,-0.5,0.5),Vec3::new(0.5,0.5,0.5),Vec3::new(-0.5,0.5,0.5)];
            let faces = [[0,1,2,3],[5,4,7,6],[4,0,3,7],[1,5,6,2],[3,2,6,7],[4,5,1,0]];
            let shades = [dark,mid,dark,mid,light,egui::Color32::from_rgb(105,115,126)];
            faces.iter().enumerate().map(|(i,f)|(f.iter().map(|ix|v[*ix]).collect(),shades[i])).collect()
        }
        Primitive::Plane => vec![(vec![Vec3::new(-0.5,0.0,-0.5),Vec3::new(0.5,0.0,-0.5),Vec3::new(0.5,0.0,0.5),Vec3::new(-0.5,0.0,0.5)],egui::Color32::from_rgb(102,118,132))],
        Primitive::Triangle => vec![(vec![Vec3::new(0.0,0.0,-0.6),Vec3::new(0.6,0.0,0.5),Vec3::new(-0.6,0.0,0.5)],mid)],
        Primitive::Sphere => {
            let mut out = Vec::new(); let latitudes = 8; let longitudes = 12;
            for lat in 0..latitudes {
                let a0 = -std::f32::consts::FRAC_PI_2 + std::f32::consts::PI * lat as f32 / latitudes as f32;
                let a1 = -std::f32::consts::FRAC_PI_2 + std::f32::consts::PI * (lat+1) as f32 / latitudes as f32;
                for lon in 0..longitudes {
                    let b0 = TAU * lon as f32 / longitudes as f32; let b1 = TAU * (lon+1) as f32 / longitudes as f32;
                    let point = |a:f32,b:f32| Vec3::new(a.cos()*b.cos()*0.5_f32,a.sin()*0.5_f32,a.cos()*b.sin()*0.5_f32);
                    out.push((vec![point(a0,b0),point(a0,b1),point(a1,b1),point(a1,b0)],if lat>4 {light}else{mid}));
                }
            }
            out
        }
        Primitive::Cylinder | Primitive::Cone => {
            let mut out = Vec::new(); let sides = 12; let mut ring = Vec::new();
            for i in 0..sides { let a = TAU*i as f32/sides as f32; ring.push(Vec3::new(a.cos()*0.5_f32,-0.5_f32,a.sin()*0.5_f32)); }
            if primitive == Primitive::Cylinder {
                let top: Vec<Vec3> = ring.iter().map(|v|Vec3::new(v.x,0.5_f32,v.z)).collect();
                for i in 0..sides { let n=(i+1)%sides; out.push((vec![ring[i],ring[n],top[n],top[i]],if i%3==0 {light}else{mid})); }
                out.push((ring.clone(),dark)); out.push((top,light));
            } else {
                let tip=Vec3::new(0.0,0.65_f32,0.0);
                for i in 0..sides { let n=(i+1)%sides; out.push((vec![ring[i],ring[n],tip],if i%3==0 {light}else{mid})); }
                out.push((ring,dark));
            }
            out
        }
    }
}

fn transform_point(mut point: Vec3, object: &SceneObject) -> Vec3 {
    point.x *= object.scale.x; point.y *= object.scale.y; point.z *= object.scale.z;
    let (sx,cx)=object.rotation.x.to_radians().sin_cos(); let (sy,cy)=object.rotation.y.to_radians().sin_cos(); let (sz,cz)=object.rotation.z.to_radians().sin_cos();
    point = Vec3::new(point.x,point.y*cx-point.z*sx,point.y*sx+point.z*cx);
    point = Vec3::new(point.x*cy+point.z*sy,point.y,-point.x*sy+point.z*cy);
    point = Vec3::new(point.x*cz-point.y*sz,point.x*sz+point.y*cz,point.z);
    object.position + point
}

fn object_bounds(object: &SceneObject, camera: Camera, rect: egui::Rect) -> Option<egui::Rect> {
    let mut points=Vec::new();
    for (face,_) in mesh_faces(object.primitive) {
        for local in face {
            if let Some((screen,_)) = project(transform_point(local,object),camera,rect) { points.push(screen); }
        }
    }
    if points.is_empty() { return None; }
    let min_x=points.iter().map(|p|p.x).fold(f32::INFINITY,f32::min); let max_x=points.iter().map(|p|p.x).fold(f32::NEG_INFINITY,f32::max);
    let min_y=points.iter().map(|p|p.y).fold(f32::INFINITY,f32::min); let max_y=points.iter().map(|p|p.y).fold(f32::NEG_INFINITY,f32::max);
    Some(egui::Rect::from_min_max(egui::pos2(min_x-6.0,min_y-6.0),egui::pos2(max_x+6.0,max_y+6.0)))
}

fn project(point: Vec3, camera: Camera, rect: egui::Rect) -> Option<(egui::Pos2,f32)> {
    let relative=point-camera.position; let forward=camera.forward(); let depth=relative.dot(forward);
    if depth <= 0.05_f32 { return None; }
    let focal=rect.height()*0.92_f32;
    let x=relative.dot(camera.right())*focal/depth;
    let y=relative.dot(camera.up())*focal/depth;
    Some((egui::pos2(rect.center().x+x,rect.center().y-y),depth))
}

fn ray_plane_hit(mouse: egui::Pos2, rect: egui::Rect, camera: Camera, plane_point: Vec3, plane_normal: Vec3) -> Option<Vec3> {
    let focal=rect.height()*0.92_f32;
    let ndc_x=(mouse.x-rect.center().x)/focal; let ndc_y=-(mouse.y-rect.center().y)/focal;
    let direction=(camera.forward()+camera.right()*ndc_x+camera.up()*ndc_y).normalized();
    let denominator=direction.dot(plane_normal);
    if denominator.abs()<0.0001_f32 { return None; }
    let t=(plane_point-camera.position).dot(plane_normal)/denominator;
    if t<0.0 { return None; }
    Some(camera.position+direction*t)
}

fn axis_vector(index: usize) -> Vec3 { match index {0=>Vec3::X,1=>Vec3::Y,_=>Vec3::Z} }
fn axis_color(index: usize) -> egui::Color32 { match index {0=>egui::Color32::from_rgb(244,91,91),1=>egui::Color32::from_rgb(104,216,145),_=>egui::Color32::from_rgb(92,160,255)} }
fn axis_projection(object: &SceneObject, index: usize, camera: Camera, rect: egui::Rect, length: f32) -> Option<(egui::Pos2,egui::Pos2,egui::Vec2,f32)> {
    let start=project(object.position,camera,rect)?.0; let end=project(object.position+axis_vector(index)*length,camera,rect)?.0;
    let delta=end-start; let pixels=delta.length()/length;
    if pixels<0.01_f32 { return None; }
    Some((start,end,delta.normalized(),pixels))
}
fn distance_to_segment(point: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> f32 {
    let ab=b-a; let denom=ab.dot(ab).max(0.0001_f32); let t=((point-a).dot(ab)/denom).clamp(0.0_f32,1.0_f32); (point-(a+ab*t)).length()
}
fn pick_axis(mouse: egui::Pos2, object: &SceneObject, camera: Camera, rect: egui::Rect, length: f32) -> Option<(usize,egui::Vec2,f32)> {
    let mut best: Option<(f32,usize,egui::Vec2,f32)> = None;
    for i in 0..3 {
        if let Some((start,end,screen_axis,pixels))=axis_projection(object,i,camera,rect,length) {
            let handle_start=start+(end-start)*0.24_f32;
            let distance=distance_to_segment(mouse,handle_start,end);
            if distance<13.0_f32 && best.map(|b|distance<b.0).unwrap_or(true) { best=Some((distance,i,screen_axis,pixels)); }
        }
    }
    best.map(|(_,i,screen,pixels)|(i,screen,pixels))
}

fn draw_gizmo(painter: &egui::Painter, rect: egui::Rect, camera: Camera, object: &SceneObject, tool: Tool) {
    let Some((center,_))=project(object.position,camera,rect) else { return; };
    match tool {
        Tool::Select => {
            painter.circle_stroke(center,4.0_f32,egui::Stroke::new(1.0_f32,egui::Color32::WHITE));
        }
        Tool::Move | Tool::Scale => {
            for i in 0..3 {
                if let Some((start,end,_,_))=axis_projection(object,i,camera,rect,1.55_f32) {
                    let color=axis_color(i);
                    painter.line_segment([start,end],egui::Stroke::new(2.6_f32,color));
                    if tool==Tool::Move {
                        let direction=(end-start).normalized(); let perpendicular=egui::vec2(-direction.y,direction.x);
                        painter.add(egui::Shape::convex_polygon(vec![end,end-direction*9.0_f32+perpendicular*4.0_f32,end-direction*9.0_f32-perpendicular*4.0_f32],color,egui::Stroke::NONE));
                    } else {
                        painter.rect_filled(egui::Rect::from_center_size(end,egui::vec2(9.0,9.0)),1.0_f32,color);
                    }
                }
            }
        }
        Tool::Rotate => {
            for radius in [31.0_f32,43.0_f32,55.0_f32] {
                let points: Vec<egui::Pos2>=(0..64).map(|i| {let a=TAU*i as f32/64.0_f32;center+egui::vec2(a.cos()*radius,a.sin()*radius)}).collect();
                painter.add(egui::Shape::closed_line(points,egui::Stroke::new(1.8_f32,if radius==31.0 {axis_color(0)} else if radius==43.0 {axis_color(1)} else {axis_color(2)})));
            }
        }
    }
}
