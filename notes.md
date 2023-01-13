### Notes:
Need to use `npm link` to create a symbolic link between /www and /pkg for the dependency, otherwise it won't update.

Need to import memory like this
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";
