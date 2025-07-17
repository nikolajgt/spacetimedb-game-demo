




#### create character:
curl -X POST http://localhost:3010/api/character/create \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnYW1lLWNsaWVudCIsInN1YiI6IjI4Y2RiZjdlLTZhMzEtNGQ3My1hZWRlLTA0YzRhMGIxYWRiMSIsImlhdCI6MTc1MjY5MTU3OCwiZW1haWwiOiJmb29AZXhhbXBsZS5jb20iLCJpc19wcmVtaXVtIjpmYWxzZSwiZXhwIjoxNzUyNzc3OTc4fQ.Pku4KBfNRrLqbMcB3eDkIJch2BSfXFsngbRXlByMBKA" \
    -d '{"name":"test_char"}'



#### select character:
curl -X POST http://localhost:3010/api/character/select \
-H "Content-Type: application/json" \
-H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnYW1lLWNsaWVudCIsInN1YiI6IjI4Y2RiZjdlLTZhMzEtNGQ3My1hZWRlLTA0YzRhMGIxYWRiMSIsImlhdCI6MTc1MjY5MTU3OCwiZW1haWwiOiJmb29AZXhhbXBsZS5jb20iLCJpc19wcmVtaXVtIjpmYWxzZSwiZXhwIjoxNzUyNzc3OTc4fQ.Pku4KBfNRrLqbMcB3eDkIJch2BSfXFsngbRXlByMBKA" \
-d '{"character_id":"cc662bf5-0785-4896-9d7e-ca963c38a849"}'



