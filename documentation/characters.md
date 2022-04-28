# Characters.yaml Configuration

# Description
This is a required file that defines an individual character entity in the game. This should be located in a directory named `characters`, located in the root of the game directory. There must be at least 1 character configuration file, but there can certainly be more to define other characters in the game, although only one player character file is supported. 

# Required fields:
| Field              | Required? | Valid Values                                                  | Description                                                                                                                                                   |
|--------------------|-----------|---------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|
| id                 | Yes       | Short unique (between characters) strings                     | This is how an individual character is uniquely identifiable within the game                                                                                  |
| name               | Yes       | Short string                                                  | This is the name shown for the character in the game                                                                                                          |
| icon               | Yes       | A single character                                            | This is how the character appears when displayed on the map                                                                                                   |
| inventory_size     | No        | width: integer greater than 1, height: integer greater than 1 | This defines the inventory size of a character (if any). Player characters most likely will have inventories, non-player characters probably won't, but could |
| attributes         | No        | Array of attributes (see below)                               | This defines any attributes that the character is able to have                                                                                                |
| interactions       | No        | Array of interactions (see below)                             | This defines any interactions that a character is able to have with another character                                                                         |
| dialog_id          | Yes       | A string (can be empty) representing a dialog id              | This defines the starting dialog that will be used by the character when the player interacts. The player should have an empty dialog string.                 |

`attributes` field:

| Field          | Required? | Valid Values                                                    | Description                                                                                                                               |
|----------------|-----------|-----------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| id             | Yes       | A unique (across all characters) string                         | This is a way to uniquely identify the attribute for interactions to modify                                                               |
| display_name   | Yes       | Any (short) string                                              | This is shown to the player (when it's their attribute) during interactions                                                               |
| starting_value | No        | An integer greater than or equal to 0                           | This is the starting value of the attribute, and will be assigned to 0 if not manually defined                                            |
| max_value      | Yes       | An integer greater than or equal to both 0 and `starting_value` | This is the maximum value of the attribute, and any attempts to increase the current value over the maximum will be clamped to this value |

`interactions` field:
Other types of interactions may be added in the future.

| Field        | Required? | Valid Values                      | Description                                                                                                           |
|--------------|-----------|-----------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| attacks      | No        | Array of interactions_attacks     | This defines any attack-based interactions between this character and another when fighting                           |
| object_use   | No        | Array of interactions_object_use  | This defines the effects of using objects on this character. If this list is empty, use: `object_use: []`             |

`interactions_attacks` field:
An attack is a type of interaction that applies when the character is attacking another character.  

| Field        | Required? | Valid Values                                                                                                                  | Description                                                                                                                                           |
|--------------|-----------|-------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
| id           | Yes       | A unique (across all characters) string                                                                                       | This is a way to uniquely identify the attack                                                                                                         |
| display_name | Yes       | Any (short) string                                                                                                            | This is shown to the player (when it's their turn to interact) during fights as a possible action                                                     |
| base_damage  | Yes       | Any integer greater than 0                                                                                                    | This is the minimum damage the attack will attempt to inflict on an opposing character, before being affected by any attributes from either character |
| affected_by  | No        | Both an "attribute_id" (`id` from one of the player's `attribute`s, and an "effect_per_point" value of (+,-,*,/) and a number | This defines any attribute effects on the interaction, applied once per point the character has of the attribute                                      |

`interactions_object_use` field:
An object_use is a type of interaction that applies when the player uses an item from their inventory on the character.

| Field        | Required? | Valid Values                                  | Description                                                                                                                                           |
|--------------|-----------|-----------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------|
| object_id    | Yes       | An object id                                  | The id of the object that can be used on this character                                                                                  |
| set_dialog   | Yes       | A dialog id (or "" if none)                   | This is the dialog id of the dialog that the character will have after the item is used on it. If this is "", the dialog is not changed. |
| consume_item | Yes       | Any boolean                                   | This indicates whether or not the item will be destroyed once it is used.                                                                |

# Sample config
```yaml
---
id: "player"
name: "You"
icon: "*"
inventory_size:
  width: 3
  height: 3
traits:
  - id: "health"
    display_name: "Health"
    starting_value: 100
    max_value: 100
  - id: "skill_1"
    display_name: "Skill 1"
    starting_value: 1
    max_value: 10
  - id: "skill_2"
    display_name: "Skill 2"
    starting_value: 5
    max_value: 10
  - id: "skill_3"
    display_name: "Skill 3"
    starting_value: 7
    max_value: 10
  - id: "super_skill_thing"
    display_name: "Super Skill Thing"
    starting_value: 1
    max_value: 10
interactions:
  attacks:
    - id: "throw_stick"
      display_name: "Throw stick"
      base_damage: 5
      affected_by: 
        - attribute_id: "skill_1"
          effect_per_point: "+1"
        - attribute_id: "skill_2"
          effect_per_point: "-1"
        - attribute_id: "skill_3"
          effect_per_point: "*1.2"
  object_use: []
dialog_id: ""
```

# Non-empty object_use Example
```yaml
  ...
  object_use:
    - object_id: "hat"
      set_dialog: "AA give hat"
      consume_item: true
  ...
```