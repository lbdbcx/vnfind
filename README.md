# Vnfind

A personal game data manage application.

## todo

- [x] sort
- [ ] tag/property set
- [x] search
- [x] comment
- [x] configurable
- [ ] list support
    - [ ] save/create/edit
    - [ ] opts
- [ ] connect to websites
    - [ ] sync comment
    - [ ] fetch information

## API

- GET `/`
    home page
- GET `/search`
    params:
    - `query` : String, used for search. Only return games contain the query string.
    - `key` : String, used for sorting result, default is `结束时间`
    - `rev` : bool, if `true` then reverse the result's order, default is `false`
    - `num` : int, how many games is returned, default is `500`
    - `page` : int, return games ranked in $(num*(page-1), num*page]$, default is `1`
    - `columns` : String, return what columns, splited by `|` or `｜`. default is `剧情|画面|角色|感情|玩法|日常|色情|声音|结束时间`. (`id` and `标题` columns are always returned)

    example: `/search?key=剧情&rev=true&num=2&page=2&columns=剧情|结束时间`, this request will return 2 games with the third and forth smallest score in 剧情 of all games.
    response is a json like below:
    ```json
    {
        "column": ["id", "标题", "剧情", "结束时间"],
        "row": [
            ["3",  "Symphonic Rain", "9",   "2024-01-01"],
            ["24", "Steins;Gate",    "9.1", "2011-08-17"],
        ]
    }
    ```
- GET `/get_game`
- POST `/add_game`
- POST `/edit_game`
- GET `/get_comment`
- POST `/set_comment`

## Config

Create `config.toml` at the same path of the executable file.
It will be like:
```toml
address = "127.0.0.1"
port = 8000
data_path = "/path/to/data/dir"
web_path = "/path/to/frontend/dir"
default_column = ["剧情", "结束时间"]
```