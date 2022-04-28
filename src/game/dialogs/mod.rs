use serde::Deserialize;

// Dialog data structure. Contains a single dialog object consisting of
// some NPC dialog and two dialog options for the player.

#[derive(Debug, Clone, Deserialize)]
pub struct Dialog {
    pub id: String,
    pub npc_dialog: String,
    pub option_0: DialogOption,
    pub option_1: DialogOption,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DialogOption {
    pub dialog: String,
    pub next: String,
}