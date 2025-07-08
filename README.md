



#### db info:
- name: game-demo

#### auth server:
- sudo systemctl start postgresql

#### server commands:
- spacetime build
- spacetime publish --project-path server game-demo

#### client commands:
spacetime generate --lang rust --out-dir client/src/module_bindings --project-path server
"SELECT * FROM character_movement WHERE character_id IN (
SELECT character_id FROM identity_binding WHERE identity = CURRENT_IDENTITY()
)
"