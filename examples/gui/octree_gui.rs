use nse::core::System;

use imgui::*;

mod support;

pub struct OctreeGui {
    system: support::System
}

impl OctreeGui {
    pub fn new() -> Self {
        let system = support::init(file!());
        system.main_loop(move |_, ui| {
            Window::new(im_str!("Hello world"))
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(ui, || {
                    ui.text(im_str!("Hello world!"));
                    ui.text(im_str!("こんにちは世界！"));
                    ui.text(im_str!("This...is...imgui-rs!"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
        });

        OctreeGui{
            system
        }
    }
}

impl System for OctreeGui {

}