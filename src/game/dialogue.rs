use bevy::prelude::*;

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

#[derive(Component)]
pub struct DialogueUi;

pub fn update_dialogue(
    mut queue: ResMut<DialogueQueue>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    existing_ui: Query<Entity, With<DialogueUi>>,
) {
    if !queue.is_active() {
        for e in &existing_ui {
            commands.entity(e).despawn();
        }
        return;
    }

    let advance = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::KeyE);

    if advance {
        queue.lines.pop_front();
    }
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
