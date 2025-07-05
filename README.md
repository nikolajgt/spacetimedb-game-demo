
db instance: 
- name: game-demo
- spacetime call game-demo register_player "Player1"
- spacetime sql game-demo "SELECT * FROM player"

#### server commands:
- spacetime build
- spacetime publish --project-path server game-demo

#### client commands:
spacetime generate --lang rust --out-dir client/src/module_bindings --project-path server