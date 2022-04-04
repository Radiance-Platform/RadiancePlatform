# Objects.yaml Configuration

# Description
This is a required file that defines an individual object entity in the game. This should be located in a directory named `objects`, located in the root of the game directory. There does not need to be any objects defined for the game to run, but that wouldn't make much sense. These can be world-placeable, or inventory-only.

# Required fields:
| Field           | Required? | Valid Values                  | Description                                                                     |
|-----------------|-----------|-------------------------------|---------------------------------------------------------------------------------|
| id              | Yes       | Any (short) string            | A way of uniquely (across all objects) identifying this object                  |
| name            | Yes       | Any (short) string            | A small piece of text shown to the player when they find the object in the game |
| category        | Yes       | Any ObjectCategory            | A category that defines how the object behaves in the game                      |
| icon            | Yes       | A single character            | This is how the object will be visually shown in the game                       |
| world_placeable | Yes       | A boolean                     | This defines whether or not an item can be placed on the map                    |
| state           | No        | An array of ObjectState       | This defines any state values that an object will have                          |
| interactions    | No        | An array of ObjectInteraction | This defines how the object can interact with other things in the game          |

ObjectCategory:
These are different categories of objects, which can have different features depending on what the category is
- Collidable
  - Something in world that the player is not able to walk over, and generally performs no function and is purely decorative
- Simple
  - Something in world that the player is able to walk over, and generally performs no function and is purely decorative
- Collectable
  - Something in world that the player is able to walk over, and can be picked up and put into their inventory on interaction 
- Door
  - Something in world that the player is able to walk over, and will take the player to another map if it has the state of `unlocked` set to `true`, or able to be unlocked if used with a "key"

ObjectState:
In the future, object states may be adapted to support strings or integers, but currently are only able to be booleans. 

| Field   | Required? | Valid Values       | Description                                                   |
|---------|-----------|--------------------|---------------------------------------------------------------|
| id      | Yes       | Any (short) string | A way of uniquely (across this object) identifying this state |
| default | Yes       | Any boolean        | The state is either true or false (may be expanded later)     |

ObjectInteraction:
Currently, the only supported interaction types are `activate` and `object_use`. 

The fields for the `activate` interaction category is shown below. Note that this field has an array of these types.

| Field       | Required? | Valid Values                          | Description                                                           |
|-------------|-----------|---------------------------------------|-----------------------------------------------------------------------|
| category    | Yes       | Any ObjectInteractionActivateCategory | Allows different functionality based on the category                  |
| prereqs     | No        | A `state` id and a state `value`      | The state is either true or false (may be expanded later)             |
| destination | Sometimes | A map `id`                            | If the category is `travel`, this is a map `id` of where to travel to |

The fields for the `object_use` interaction category is shown below. Note that this field has an array of these types.
In the future, other_action could also include changing the state of the other object. 

| Field              | Required? | Valid Values                                                      | Description                                                                                                                                   |
|--------------------|-----------|-------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|
| foreign_objects_id | Yes       | Any object `id`                                                   | An object who, when used on this object, will cause something to happen to this object                                                        |
| self_action        | Yes       | An array of state `id`s as keys and new state values as values    | An array of state ids and new values to set them to upon using the foreign object on this object                                              |
| other_action       | Yes       | Currently, only consume_item as a key, and a boolean as its value | An array of just consume_item as a key and a boolean determining whether or not the foreign object should be used up when used on this object |


# Sample config
```yaml
---
id: "door_1"
name: "Door"
type: "door"
icon: "X"
state:
  - id: "unlocked"
    default: false
interactions:
  activate:
    - category: "travel"
      prereqs:
        - state: "unlocked"
          value: true
      destination: "basement"
  object_use:
    - foreign_objects_id: "key"
      self_action:
        - unlocked: true
      other_action:
        consume_item: true
```