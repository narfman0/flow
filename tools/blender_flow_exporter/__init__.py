"""
FOLD Blender Auto-Exporter
Watches the .blend file for unsaved changes and auto-exports to glTF on save.
Install: Edit > Preferences > Add-ons > Install > select this folder (zip it first)
"""

bl_info = {
    "name": "flow Auto-Exporter",
    "author": "FOLD Project",
    "version": (0, 1, 0),
    "blender": (4, 0, 0),
    "category": "Import-Export",
    "description": "Auto-export glTF to assets/levels/ when the .blend file is saved",
}

import bpy
import os

TIMER_INTERVAL = 2.0  # seconds between dirty checks
_timer = None
_last_dirty = False


class FlowExporterProperties(bpy.types.PropertyGroup):
    export_path: bpy.props.StringProperty(
        name="Export Path",
        default="../assets/levels/",
        description="Directory to export glTF files into",
        subtype="DIR_PATH",
    )
    filename: bpy.props.StringProperty(
        name="Filename",
        default="level.glb",
        description="Output filename (include .glb or .gltf extension)",
    )
    auto_export: bpy.props.BoolProperty(
        name="Enable Auto-Export",
        default=False,
        description="Auto-export whenever the file is saved",
    )


class FLOW_OT_ExportNow(bpy.types.Operator):
    """Export the current scene to glTF immediately"""
    bl_idname = "flow.export_now"
    bl_label = "Export Now"

    def execute(self, context):
        do_export(context)
        self.report({"INFO"}, "FOLD: exported to glTF")
        return {"FINISHED"}


class FLOW_PT_Panel(bpy.types.Panel):
    bl_label = "flow Exporter"
    bl_idname = "FLOW_PT_Panel"
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"
    bl_category = "flow"

    def draw(self, context):
        layout = self.layout
        props = context.scene.flow_exporter

        layout.prop(props, "export_path")
        layout.prop(props, "filename")
        layout.prop(props, "auto_export")
        layout.operator("flow.export_now")


def do_export(context):
    props = context.scene.flow_exporter
    blend_dir = os.path.dirname(bpy.data.filepath) if bpy.data.filepath else ""
    out_dir = bpy.path.abspath(props.export_path, start=blend_dir)
    os.makedirs(out_dir, exist_ok=True)
    out_path = os.path.join(out_dir, props.filename)

    bpy.ops.export_scene.gltf(
        filepath=out_path,
        export_format="GLB" if out_path.endswith(".glb") else "GLTF_SEPARATE",
        export_extras=True,          # include custom properties as glTF extras
        export_apply=True,
        export_animations=True,
    )


def _timer_callback():
    global _last_dirty
    if not bpy.context:
        return TIMER_INTERVAL

    props = bpy.context.scene.flow_exporter if hasattr(bpy.context.scene, "flow_exporter") else None
    if props is None or not props.auto_export:
        return TIMER_INTERVAL

    currently_dirty = bpy.data.is_dirty
    if _last_dirty and not currently_dirty:
        # File was just saved (dirty → clean transition)
        do_export(bpy.context)
    _last_dirty = currently_dirty
    return TIMER_INTERVAL


def register():
    bpy.utils.register_class(FlowExporterProperties)
    bpy.utils.register_class(FLOW_OT_ExportNow)
    bpy.utils.register_class(FLOW_PT_Panel)
    bpy.types.Scene.flow_exporter = bpy.props.PointerProperty(type=FlowExporterProperties)
    bpy.app.timers.register(_timer_callback, first_interval=TIMER_INTERVAL, persistent=True)


def unregister():
    if bpy.app.timers.is_registered(_timer_callback):
        bpy.app.timers.unregister(_timer_callback)
    del bpy.types.Scene.flow_exporter
    bpy.utils.unregister_class(FLOW_PT_Panel)
    bpy.utils.unregister_class(FLOW_OT_ExportNow)
    bpy.utils.unregister_class(FlowExporterProperties)


if __name__ == "__main__":
    register()
