# Editor update notes

This update replaces the fixed-position mock viewport and hard-coded hierarchy with editable scene data.

## Included

- Free-fly perspective camera: right-click drag to look, WASD / arrow keys to move, Q/E to descend/ascend, mouse wheel to dolly, Shift to move faster.
- 3D scene object data with position, rotation and scale.
- CPU-projected primitive preview for cube, sphere, cylinder, cone, plane and triangle.
- Viewport object picking and white selection outlines.
- Select/Move/Rotate/Resize (Scale) tools with numeric shortcuts 1–4 and vector-drawn toolbar icons.
- Select and Move drag translation, axis-constrained Move/Scale gizmos, Rotate rings, and uniform scaling by dragging the selected object.
- Inspector editing for name, position, rotation, scale and parent folder.
- User-created root folders and nested folders, right-click menus, rename/delete, and object-to-folder organization.
- New / Save / Load scene actions using `scene.kumo`, plus duplicate/delete/focus shortcuts.
- Functional grid/gizmo/panel toggles and a clearable console.

## Intentionally not included yet

No built-in scripts or scripting language, game runtime, Play button, SpawnPoint, default Workspace, physics, lighting, material editor, asset importer, or full GPU 3D renderer. The viewport is an editor-side CPU projection and primitive drawing pass, not a production renderer.
