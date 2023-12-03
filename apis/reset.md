# Reset API

All examples show cURL snippets.

## Reset a team by id

```shel
curl -X GET 'http://localhost:8034/reset/team?id=9' -H "Content-Type: application/json; charset=utf-8"
```

You should reset a team choice hero by id.

## Reset all teams

```shell
curl -X GET 'http://localhost:8034/reset/teams' -H "Content-Type: application/json; charset=utf-8"
```

You should reset all teams.

## Reset all heroes

```shell
curl -X GET 'http://localhost:8034/reset/heroes' -H "Content-Type: application/json; charset=utf-8"
```

You should reset all heroes.
