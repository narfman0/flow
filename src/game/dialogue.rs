use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Debug, Clone)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
    pub portrait: Option<String>,
}

#[derive(Resource, Default)]
pub struct DialogueQueue {
    pub lines: std::collections::VecDeque<DialogueLine>,
}

impl DialogueQueue {
    pub fn push(&mut self, line: DialogueLine) {
        self.lines.push_back(line);
    }

    pub fn is_active(&self) -> bool {
        !self.lines.is_empty()
    }
}

pub fn speaker_color(speaker: &str) -> Color {
    match speaker {
        "Kai" => Color::srgb(1.0, 0.5, 0.0),
        "Sela" => Color::srgb(0.6, 0.4, 0.9),
        "Daven" => Color::srgb(0.6, 0.5, 0.3),
        "Pell" => Color::srgb(0.4, 0.9, 0.3),
        _ => Color::WHITE,
    }
}

pub fn update_dialogue(
    mut queue: ResMut<DialogueQueue>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !queue.is_active() {
        return;
    }

    let advance = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::KeyE);

    if advance {
        queue.lines.pop_front();
    }
}

/// Renders the dialogue overlay as a bottom-center egui panel while the queue
/// has items. Does nothing (no panel) when the queue is empty.
pub fn dialogue_ui(mut contexts: EguiContexts, queue: Res<DialogueQueue>) {
    let Some(line) = queue.lines.front() else {
        return;
    };

    let ctx = contexts.ctx_mut();
    let color = speaker_color(&line.speaker);
    let [r, g, b, _] = color.to_srgba().to_u8_array();
    let speaker_egui_color = egui::Color32::from_rgb(r, g, b);

    egui::TopBottomPanel::bottom("dialogue_panel")
        .show_separator_line(false)
        .frame(
            egui::Frame::default()
                .fill(egui::Color32::from_rgba_unmultiplied(10, 10, 16, 230))
                .inner_margin(egui::Margin::symmetric(24, 16)),
        )
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_max_width(900.0);
                ui.label(
                    egui::RichText::new(&line.speaker)
                        .color(speaker_egui_color)
                        .size(22.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(&line.text)
                        .color(egui::Color32::WHITE)
                        .size(18.0),
                );
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Press E / Space / Enter to continue")
                        .color(egui::Color32::from_gray(160))
                        .italics()
                        .size(13.0),
                );
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_starts_empty() {
        let q = DialogueQueue::default();
        assert!(!q.is_active());
    }

    #[test]
    fn push_activates_queue() {
        let mut q = DialogueQueue::default();
        q.push(DialogueLine { speaker: "Kai".into(), text: "Hello".into(), portrait: None });
        assert!(q.is_active());
    }

    #[test]
    fn pop_deactivates_when_empty() {
        let mut q = DialogueQueue::default();
        q.push(DialogueLine { speaker: "Sela".into(), text: "Hi".into(), portrait: None });
        q.lines.pop_front();
        assert!(!q.is_active());
    }

    #[test]
    fn speaker_colors_assigned() {
        assert_ne!(speaker_color("Kai"), speaker_color("Sela"));
        assert_ne!(speaker_color("Daven"), speaker_color("Pell"));
        assert_eq!(speaker_color("Unknown"), Color::WHITE);
    }
}
