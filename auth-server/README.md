


#### create user:
curl -X POST http://localhost:3010/api/register \
    -H "Content-Type: application/json" \
    -d '{"email":"foo@example.com", "password":"Password0"}'

#### authenticate:
curl -X POST http://localhost:3010/api/authenticate \
    -H "Content-Type: application/json" \
    -H "User-Agent: tui-launcher/1.0" \
    -d '{"email":"foo@example.com", "password":"Password0"}'