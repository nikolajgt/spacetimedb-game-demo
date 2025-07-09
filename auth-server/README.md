
#### create user:
curl -X POST http://localhost:3010/register \
    -H "Content-Type: application/json" \
    -d '{"email":"test@example.com", "password":"secret"}'

#### authenticate:
curl -X POST http://localhost:3010/authenticate  \
    -H "Content-Type: application/json" \
    -d '{"email":"test@example.com", "password":"Secret354252"}'