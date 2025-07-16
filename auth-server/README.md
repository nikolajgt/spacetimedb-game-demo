


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


curl -X POST http://127.0.0.1:3000/v1/database/game-demo/sql
-H "Content-Type: application/json" \
-H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJlM2JmODE0Mi0yMzFkLTQxYzgtOWQ3YS1kNWM2MjEwMmU2NDgiLCJpc3MiOiJodHRwOi8vbG9jYWxob3N0OjMwMTAiLCJhdWQiOlsic3BhY2V0aW1lZGIiXSwiaWF0IjoxNzUyNjYyODY1LCJleHAiOjE3NTI3NDkyNjUsImNoYXJfaWQiOiI5NDQxOGJkOC05OGYyLTQ0NzctOGMxNC04ZDY1Y2Y4YTY4NTgifQ.qp61JdKTrZimNYxPBDrK6F7NRV-2fUUL-OSg5GfUxjxwkukgZ7mxY16MfAxsAt42wpUWVKImvk2mDN8Vdbqe-B55YGyEA5A8GV65k8FnV1OWPWaz70ygWjQ8HnDftoG-gh_kXE2yPCKJJY5Bb6Iqoni6RGxQlK9jpYeWhGJ1U5PUb39arNopst4KgxiJ0NuMXllJlaaFkQSf0_OrlQbc_dDMljIGXkpnNk7Ri_TKDmy9GOnmVd0QZe6iH2u3hFdDVMWGkb2ow6z-BhBexSAFe24fXxvxsiDy1g2R4yFYoRLUH1UozCbP-1Mva_9ABSYrgd49tPvK9Glk6EfWI6NJ8w" \
-d '{"INSERT INTO characters;"}'

curl -X POST http://127.0.0.1:3000/v1/database/game-demo/sql \
-H "Content-Type: text/plain" \
-H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJlM2JmODE0Mi0yMzFkLTQxYzgtOWQ3YS1kNWM2MjEwMmU2NDgiLCJpc3MiOiJodHRwOi8vbG9jYWxob3N0OjMwMTAiLCJhdWQiOlsic3BhY2V0aW1lZGIiXSwiaWF0IjoxNzUyNjYyODY1LCJleHAiOjE3NTI3NDkyNjUsImNoYXJfaWQiOiI5NDQxOGJkOC05OGYyLTQ0NzctOGMxNC04ZDY1Y2Y4YTY4NTgifQ.qp61JdKTrZimNYxPBDrK6F7NRV-2fUUL-OSg5GfUxjxwkukgZ7mxY16MfAxsAt42wpUWVKImvk2mDN8Vdbqe-B55YGyEA5A8GV65k8FnV1OWPWaz70ygWjQ8HnDftoG-gh_kXE2yPCKJJY5Bb6Iqoni6RGxQlK9jpYeWhGJ1U5PUb39arNopst4KgxiJ0NuMXllJlaaFkQSf0_OrlQbc_dDMljIGXkpnNk7Ri_TKDmy9GOnmVd0QZe6iH2u3hFdDVMWGkb2ow6z-BhBexSAFe24fXxvxsiDy1g2R4yFYoRLUH1UozCbP-1Mva_9ABSYrgd49tPvK9Glk6EfWI6NJ8w" \
--data-raw "INSERT INTO characters (character_id, identity, name, level) VALUES (12345678901234567890, 'abcdef1234567890abcdef1234567890abcdef12abcdef1234567890abcdef12', 'TestChar', 1);"
