use crate::directors::sce_director::{SceCommand, SceState};
use imgui::Ui;
use radiance::scene::SceneManager;

#[derive(Clone)]
pub struct SceCommandGoto {
    addr: u32,
}

impl SceCommand for SceCommandGoto {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        state.vm_context_mut().jump_to(self.addr);
        true
    }
}

impl SceCommandGoto {
    pub fn new(addr: u32) -> Self {
        Self { addr }
    }
}
