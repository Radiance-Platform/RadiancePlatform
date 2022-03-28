# Maps.yaml Configuration

# Description
This is a required file that defines an individual map entity in the game. This should be located in a directory named `maps`, located in the root of the game directory. There must be at least 1 map defined for the game to open into.  

Maps can be transitioned between using doors (a `type` of `object`). Object references can be placed into the map files to add them to the game.

# Required fields:
| Field       | Required? | Valid Values                                                  | Description                                                                                                                                                                                   |
|-------------|-----------|---------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| id          | Yes       | A (ideally short) unique string                               | This is how maps are referenced by other game entities                                                                                                                                        |
| description | No        | A (ideally short) unique string                               | This is a message shown at the top of the map to give players information about the map, such as helpful objective hints                                                                      |
| size        | Yes       | width: integer greater than 2, height: integer greater than 2 | Two values that represent the width (terminal character columns) and height (terminal character rows) of the room. This includes the walls, which are automatically added and are 1x1 in size |
| objects     | Yes       | An array of objects (see below)                               | This defines what objects are placed into the map, and where they will be located                                                                                                             |

`objects` field:

| Field       | Required? | Valid Values                                                                  | Description                                                                 |
|-------------|-----------|-------------------------------------------------------------------------------|-----------------------------------------------------------------------------|
| id          | Yes       | An `id` string from an `object` defined in the `objects` configuration folder | This is how maps reference other game entities for inclusion in the map     |
| position    | Yes       | x: integer and y: integer, must be within the map's `size`                    | This is how maps known where to locate individual objects within themselves |



# Sample config
```yaml
id: "A unique identifying reference"
description: "A short description shown at the top of the screen"
size:
  width: 20
  height: 20
objects:
  - id: "door_1"
    position:
      x: 19
      y: 10
  - id: "id_of_another_object"
    position:
      x: 12
      y: 5
```