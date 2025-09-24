# Board Game Logger
An app to log your board game plays.

## Players
- `curl -X POST -H "Content-Type: application/json" -d '{"name": "Wingspan", "creator_name": "Elizabeth Hargrave"}' http://localhost:3000/api/games`
- `curlj -X POST -H "Content-Type: application/json" -d '{"name": "Catan", "creator_name": "Klaus Teuber"}' http://localhost:3000/api/games`
- `curlj -X POST -H "Content-Type: application/json" -d '{"name": "Dune: Imperium", "creator": "Paul Dennen", "plays": 5}' http://localhost:3000/api/games`
- `curlj -X GET http://localhost:3000/api/games`
- `curlj -X GET http://localhost:3000/api/games/1f184958-9e65-41e1-9422-1cfc2704ba36`
- `curlj -X DELETE http://localhost:3000/api/games/1f184958-9e65-41e1-9422-1cfc2704ba36`
- `curlj -X PATCH -H "Content-Type: application/json" -d '{"creator_name": "Someone else"}' http://localhost:3000/api/games/1f184958-9e65-41e1-9422-1cfc2704ba36`

- `psql -U admin -d wingstats`
- `\dt` -> Show Tables
- `\d players` -> Describe players
- `SELECT * from players;` -> works for both DBs