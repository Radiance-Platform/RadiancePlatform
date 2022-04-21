# Game.yaml Configuration

# Description
This is a required file that defines 4 basic attributes about the game. This should be located at the root of the game directory. 

# Required fields
| Field             | Required? | Valid Values                                                                                 | Description                                                                                                                         |
|-------------------|-----------|----------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| name              | Yes       | Strings shorter than min_screen_size.width - 2                                               | A short string that represents the name of the game                                                                                 |
| description       | Yes       | Strings shorter than (min_screen_size.width - 2) * min_screen_size.height                    | A short string that describes the game and anything the player might need to know                                                   |
| author            | Yes       | Strings shorter than min_screen_size.width - 2                                               | A short string that represents who created the game                                                                                 |
| min_screen_size   | Yes       | width: integer greater than or equal to 80, height: integer greater than or equal to 20      | Two values that represent the minimum width (terminal character columns) and height (terminal character rows) supported by the game |
| starting_map      | Yes       | An `id` of a predefined map                                                                  | The id of a map where you want the player to start the game in                                                                      |
| starting_position | Yes       | x: integer and y: integer, must be within the size of starting_map                           | The x and y coordinates where the player will be placed on the map at the start of the game                                         |

# Sample config
```yaml
---
name: "Name of Game"
description: "A short description about the game"
author: "Whoever wrote the game"
min_screen_size:
  width: 80
  height: 20
starting_map: "Main Room"
starting_position:
  x: 1
  y: 1
```