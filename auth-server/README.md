


#### create user:
curl -X POST http://localhost:3010/api/register \
    -H "Content-Type: application/json" \
    -d '{"email":"foo@example.com", "password":"Password0"}'

#### authenticate:
curl -X POST http://localhost:3010/api/authenticate \
    -H "Content-Type: application/json" \
    -H "User-Agent: tui-launcher/1.0" \
    -d '{"email":"foo@example.com", "password":"Password0"}'



#### create character:
curl -X POST http://localhost:3010/api/character/create \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnYW1lLWNsaWVudCIsInN1YiI6ImUzYmY4MTQyLTIzMWQtNDFjOC05ZDdhLWQ1YzYyMTAyZTY0OCIsImlhdCI6MTc1MjYxNzg5NiwiZW1haWwiOiJmb29AZXhhbXBsZS5jb20iLCJpc19wcmVtaXVtIjpmYWxzZSwiZXhwIjoxNzUyNzA0Mjk2fQ.TmEIgKPZ5zyeEKuoFSouis4SOSNwWP_8LMHl0frl4-s" \
    -d '{"name":"test_char"}'



#### select character:
curl -X POST http://localhost:3010/api/character/select \
-H "Content-Type: application/json" \
-H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnYW1lLWNsaWVudCIsInN1YiI6ImUzYmY4MTQyLTIzMWQtNDFjOC05ZDdhLWQ1YzYyMTAyZTY0OCIsImlhdCI6MTc1MjYxNzg5NiwiZW1haWwiOiJmb29AZXhhbXBsZS5jb20iLCJpc19wcmVtaXVtIjpmYWxzZSwiZXhwIjoxNzUyNzA0Mjk2fQ.TmEIgKPZ5zyeEKuoFSouis4SOSNwWP_8LMHl0frl4-s" \
-d '{"character_id":"94418bd8-98f2-4477-8c14-8d65cf8a6858"}'