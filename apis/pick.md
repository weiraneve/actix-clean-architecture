# Pick API

All examples show cURL snippets.

## Picking a team by encryptCode

```shel
curl -X POST 'http://localhost:8034/' -H "Content-Type: application/json; charset=utf-8" -d '{"encryptCode": "asd"}'
```

You should pick a team lottery hero message by encryptCode.

```json
{
  "teamId": 9,
  "data": "[鲁班大师,白起]or[杨戬,雅典娜]",
  "time": "2023年11月17日 16时24分04秒",
  "logs": [
    {
      "teamId": 9,
      "pickGroup": "[鲁班大师,白起]or[杨戬,雅典娜]",
      "time": "2023-11-17T08:24:04.000+00:00"
    },
    {
      "teamId": 9,
      "pickGroup": "[苏烈,云缨]or[达摩,西施]",
      "time": "2023-11-17T08:17:40.000+00:00"
    },
    {
      "teamId": 9,
      "pickGroup": "[阿古朵,成吉思汗]or[沈梦溪,云中君]",
      "time": "2022-11-28T01:10:30.000+00:00"
    }
  ]
}
```
