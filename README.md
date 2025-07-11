



#### db info:
- name: game-demo

#### auth server:
- sudo systemctl start postgresql

#### server commands:
- spacetime build
- spacetime publish --project-path server game-demo
- IDENTITY="ce564457d9260325feaf98136399948a82b69779e2a7b4e1a1668a7218983ff32"
  TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6..."
  AUTH=$(echo -n "$IDENTITY:$TOKEN" | base64)

curl -i -H "Authorization: Basic $AUTH" \
http://localhost:3000/v1/identity/$IDENTITY/verify

#### client commands:
spacetime generate --lang rust --out-dir client/src/module_bindings --project-path server
"SELECT * FROM character_movement WHERE character_id IN (
SELECT character_id FROM identity_binding WHERE identity = CURRENT_IDENTITY()
)
"