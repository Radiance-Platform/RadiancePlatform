# Characters.yaml Configuration

# Description
This is a required file that defines an individual character entity in the game. This should be located in a directory named `characters`, located in the root of the game directory. There must be at least 1 character configuration file, but there can certainly be more to define other characters in the game, although only one player character file is supported. 

# Required fields:
| Field              | Required? | Valid Values                                                  | Description                                                                                                                                             |
|--------------------|-----------|---------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| id                 | Yes       | Short unique (between characters) strings                     | This is how an individual character is uniquely identifiable within the game                                                                            |
| inventory_size     | No        | width: integer greater than 1, height: integer greater than 1 | This defines the inventory size of a character (if any). Characters most likely will haven inventories, non-player characters probably won't, but could |
| inventory_contents | No        | Array of inventory contents (see below)                       | This defines any inventory contents that are given to the character on creation without the character having to acquire them through gameplay           |
| attributes         | No        | Array of attributes (see below)                               | This defines any attributes that the character is able to have                                                                                          |
| interactions       | No        | Array of interactions (see below)                             | This defines any interactions that a character is able to have with another character                                                                   |

`inventory_contents` field:

| Field     | Required? | Valid Values                                                                  | Description                                                                             |
|-----------|-----------|-------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------|
| object_id | Yes       | An `id` string from an `object` defined in the `objects` configuration folder | This is an `object`'s `id` that represents what would fill a particular inventory slot  |
| position  | Yes       | x: integer and y: integer, must be within the character's `inventory_size`    | This is the position within the character's inventory that the object will be placed in |

`attributes` field:

| Field          | Required? | Valid Values                                                    | Description                                                                                                                               |
|----------------|-----------|-----------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| id             | Yes       | A unique (across all characters) string                         | This is a way to uniquely identify the attribute for interactions to modify                                                               |
| display_name   | Yes       | Any (short) string                                              | This is shown to the player (when it's their attribute) during interactions                                                               |
| starting_value | No        | An integer greater than or equal to 0                           | This is the starting value of the attribute, and will be assigned to 0 if not manually defined                                            |
| max_value      | Yes       | An integer greater than or equal to both 0 and `starting_value` | This is the maximum value of the attribute, and any attempts to increase the current value over the maximum will be clamped to this value |

`interactions` field:
Other types of interactions may be added in the future.

| Field        | Required? | Valid Values                  | Description                                                                                 |
|--------------|-----------|-------------------------------|---------------------------------------------------------------------------------------------|
| attacks      | No        | Array of interactions_attacks | This defines any attack-based interactions between this character and another when fighting |

`interactions_attacks` field:
An attack is a type of interaction that applies when the character is attacking another character.  

| Field        | Required? | Valid Values                                                                                                                  | Description                                                                                                                                           |
|--------------|-----------|-------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
| id           | Yes       | A unique (across all characters) string                                                                                       | This is a way to uniquely identify the attack                                                                                                         |
| display_name | Yes       | Any (short) string                                                                                                            | This is shown to the player (when it's their turn to interact) during fights as a possible action                                                     |
| base_damage  | Yes       | Any integer greater than 0                                                                                                    | This is the minimum damage the attack will attempt to inflict on an opposing character, before being affected by any attributes from either character |
| affected_by  | No        | Both an "attribute_id" (`id` from one of the player's `attribute`s, and an "effect_per_point" value of (+,-,*,/) and a number | This defines any attribute effects on the interaction, applied once per point the character has of the attribute                                      |


# Sample config
```yaml
---
id: "player"
name: "You"
inventory_size:
  width: 3
  height: 3
inventory_contents:
  - object_id: "id_of_an_object"
    position:
      x: 0
      y: 0
traits:
  - name: "health"
    display_name: "Health"
    starting_value: 100
    max_value: 100
  - name: "skill_1"
    display_name: "Skill 1"
    starting_value: 1
    max_value: 10
  - name: "skill_2"
    display_name: "Skill 2"
    starting_value: 5
    max_value: 10
  - name: "skill_3"
    display_name: "Skill 3"
    starting_value: 7
    max_value: 10
  - name: "super_skill_thing"
    display_name: "Super Skill Thing"
    starting_value: 1
    max_value: 10
interactions:
  attacks:
    - name: "throw_stick"
      display_name: "Throw stick"
      base_damage: 5
      affected_by: 
        - attribute: "skill_1"
          effect_per_point: "+1"
        - attribute: "skill_2"
          effect_per_point: "-1"
        - attribute: "skill_3"
          effect_per_point: "*1.2"
```