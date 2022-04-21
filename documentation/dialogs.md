# Dialog.yaml Configuration

# Description
This is a file that defines a list of dialog entities in the game. This should be located in a directory named `dialogs`, located in the root of the game directory. There should be exactly 1 dialog file.  

Dialogs can be added to characters by including their id in the dialog_id field in the character file. Dialog can start when the player interacts with an NPC.

# Required fields for each dialog:
| Field       | Required? | Valid Values                                                  | Description                                                                                              |
|-------------|-----------|---------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| id          | Yes       | A (ideally short) unique string                               | This is how sections of dialog are referenced by other game entities.                                    |
| npc_dialog  | Yes       | A (ideally short) string                                      | This is the dialog that the npc will display during character interaction.                               |
| option_0    | Yes       | A dialog option (see below)                                   | This is one of the dialog options that the player can choose during an interaction.                      |
| option_1    | Yes       | A dialog option (see below)                                   | This is one of the dialog options that the player can choose during an interaction.                      |

`option_0` and `option_1` fields:

| Field       | Required? | Valid Values                                                                  | Description                                                                 |
|-------------|-----------|-------------------------------------------------------------------------------|-----------------------------------------------------------------------------|
| dialog      | Yes       | A (ideally short) string                                                      | The dialog to be displayed for the player to select.                        |
| next        | Yes       | A dialog id or a keyword (see below)                                          | This describes where the dialog will progress to if this option is selected.|                                                                            |

`next` Keywords:
- "exit"
  - exits the dialog
- "inventory"
  - opens inventory so the player can use an item during dialog
- "fight"
  - starts combat with the npc


# Sample config
```yaml
 - id: "AA start"
   npc_dialog: "Why are you here! Go away!"
   option_0:
      dialog: "I'm sorry, I just wanted the key!"
      next: "d0"
   option_1:
      dialog: "Here, I brought you an item!\n(Open Inventory)"
      next: "inventory"
 - id: "d0"
   npc_dialog: "You can't have the key."
   option_0:
      dialog: "Okay"
      next: "exit"
   option_1:
      dialog: "I will fight you for the key"
      next: "fight"
```