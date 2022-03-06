# Overview

All configuration files should exist in the same "root" folder. Within that folder, there should be a game.yaml file, which will determine information about the game (see game.md). 

# Configuration File Types
A "maps" folder is required, with at least 1 yaml file defining a room. A "characters" folder is required, with at least a file called "player.yaml" describing the player, and files for any other characters in the game. Lastly, an "objects" folder is required, which defines any objects that might appear in the game. Technically, this folder could be omitted if the game designer doesn't want to define any objects, but that wouldn't make much sense in a real game, so Radiance requires it.

# Example Hierarchy
```
root_game_folder
root_game_folder/game.yaml
root_game_folder/maps
root_game_folder/maps/room_1.yaml
root_game_folder/maps/basement.yaml
root_game_folder/maps/...
root_game_folder/characters
root_game_folder/characters/player.yaml
root_game_folder/characters/enemy_1.yaml
root_game_folder/characters/angry_antagonist.yaml
root_game_folder/characters/...
root_game_folder/objects
root_game_folder/objects/key.yaml
root_game_folder/objects/door_1.yaml
root_game_folder/objects/door_2.yaml
root_game_folder/objects/table.yaml
root_game_folder/objects/hat.yaml
root_game_folder/objects/...
...
```