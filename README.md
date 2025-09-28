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

## Plays
- `curlj -X POST -H "Content-Type: application/json" -d '{"game_id": "e33037d1-62ad-4a62-952e-c80d4db7b8a9"}' http://localhost:3000/api/plays`
- `curlj -X GET http://localhost:3000/api/plays`
- `curlj -X POST -H "Content-Type: application/json" -d '{"game_id": "35ed1910-e3ea-4d31-9236-aeb66f5e8182"}' http://localhost:3000/api/plays`
- `curlj -X PATCH -H "Content-Type: application/json" -d '{"game_id": "35ed1910-e3ea-4d31-9236-aeb66f5e8182"}' http://localhost:3000/api/plays/06f1c4f1-b354-41af-adc6-cd8e42c427a5`
- `curlj -X DELETE http://localhost:3000/api/plays/d2989174-2efc-44a5-a599-b685d9e6466e`

## Play Participants
- `curlj -X POST -H "Content-Type: application/json" -d '{"play_id": "06f1c4f1-b354-41af-adc6-cd8e42c427a5", "player_id": "2d6a400f-1df8-4860-9e39-28df163bc2e2"}' http://localhost:3000/api/playparticipants`

- `curlj -X POST -H "Content-Type: application/json" -d '{"play_id": "06f1c4f1-b354-41af-adc6-cd8e42c427a5", "player_id": "f16ab1ac-76a9-423a-b553-d083d1858a8a"}' http://localhost:3000/api/playparticipants`

- `curlj -X POST -H "Content-Type: application/json" -d '{"play_id": "71293963-a790-4d0c-ad82-a039d2cb8196", "player_id": "24944a63-dd8a-4912-b26b-04d7cee92c60"}' http://localhost:3000/api/playparticipants`

- `curlj -X GET http://localhost:3000/api/playparticipants`

- `curlj -X GET http://localhost:3000/api/plays/06f1c4f1-b354-41af-adc6-cd8e42c427a5/participants`

- `curlj -X PATCH -H "Content-Type: application/json" -d '{"player_id": "24944a63-dd8a-4912-b26b-04d7cee92c60"}' http://localhost:3000/api/plays/06f1c4f1-b354-41af-adc6-cd8e42c427a5/participants/f16ab1ac-76a9-423a-b553-d083d1858a8a`

- `curlj -X DELETE http://localhost:3000/api/plays/06f1c4f1-b354-41af-adc6-cd8e42c427a5/participants/f16ab1ac-76a9-423a-b553-d083d1858a8a`