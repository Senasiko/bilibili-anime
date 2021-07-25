# bilibili-anime
A cli tool to get bilibili anime data
## useage
```
USAGE:
    bilibili-anime.exe [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -s, --star       仅展示收藏番剧
    -t, --today      仅展示当天播出番剧
    -V, --version    Prints version information

OPTIONS:
    -d, --date <date>    展示特定日期番剧 YYYY-MM-DD

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    star    操作收藏番剧
```
### star
```
USAGE:
    bilibili-anime.exe star <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       收藏番剧
    clean     清空收藏番剧
    help      Prints this message or the help of the given subcommand(s)
    list      收藏番剧列表
    remove    移除收藏番剧
```
