# Board Game Logger
An app to log your board game plays.

## Games
- `curlj -X POST -H "Content-Type: application/json" -d '{"name": "Wingspan", "creator_name": "Elizabeth Hargrave"}' http://localhost:3000/api/games`
- `curlj -X POST -H "Content-Type: application/json" -d '{"name": "Catan", "creator_name": "Klaus Teuber"}' http://localhost:3000/api/games`
- `curlj -X POST -H "Content-Type: application/json" -d '{"name": "Dune: Imperium", "creator": "Paul Dennen", "plays": 5}' http://localhost:3000/api/games`
- `curlj -X GET http://localhost:3000/api/games`
- `curlj -X GET http://localhost:3000/api/games/37c52e03-c758-483c-905c-c948ce774b05`
- `curlj -X DELETE http://localhost:3000/api/games/37c52e03-c758-483c-905c-c948ce774b05`
- `curlj -X PATCH -H "Content-Type: application/json" -d '{"creator_name": "Someone else"}' http://localhost:3000/api/games/37c52e03-c758-483c-905c-c948ce774b05`

- `psql -U admin -d bglogger`
- `\dt` -> Show Tables
- `\d games` -> Describe games
- `SELECT * from games;` -> works for both DBs

## Players
- `curlj -X POST -H "Content-Type: application/json" -d '{"name": "yoyo", "is_owner": true}' http://localhost:3000/api/players`
- `curlj -X POST -H "Content-Type: application/json" -d '{"name": "gladius"}' http://localhost:3000/api/players`
- `curlj -X POST -H "Content-Type: application/json" -d '{"name": "tod"}' http://localhost:3000/api/players`
- `curlj -X GET http://localhost:3000/api/players`
- `curlj -X GET http://localhost:3000/api/players/1f184958-9e65-41e1-9422-1cfc2704ba36`
- `curlj -X DELETE http://localhost:3000/api/players/1f184958-9e65-41e1-9422-1cfc2704ba36`
- `curlj -X PATCH -H "Content-Type: application/json" -d '{"name": "nottod"}' http://localhost:3000/api/players/24944a63-dd8a-4912-b26b-04d7cee92c60`