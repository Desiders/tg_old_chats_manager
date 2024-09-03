# tg_old_chats_manager
A utility to get Telegram chats, join or delete them. Popular clients don't allow you to find private chats that you left and join them.

## Installation and preparing
- Clone this repository: `git clone https://github.com/Desiders/tg_old_chats_manager.git`
- Create your Telegram application [following instructions](https://core.telegram.org/api/obtaining_api_id)
- Copy `configs/config.toml.example` to `configs/config.toml` and fill it with your data

## Usage
Output of `help` command:
```bash
$ tg_old_chats_manager help
Usage: tg_old_chats_manager [OPTIONS] <COMMAND>

Commands:
  analyze  Analyze chats
  join     Join chat
  delete   Delete chat
  help     Print this message or the help of the given subcommand(s)

Options:
  -c, --config-path <CONFIG_FILE_PATH>  [default: configs/config.toml]
  -h, --help                            Print help
  -V, --version                         Print version
```

Here you can see `analyze` command:
```bash
$ tg_old_chats_manager analyze --help
Analyze chats

Usage: tg_old_chats_manager analyze [OPTIONS]

Options:
  -j, --joined  Analyze joined chats
  -l, --left    Analyze left chats
  -h, --help    Print help
```
You can specify and combine `-j` flag to analyze the chats you are a member of, or `-l` for chats that you're left.
By default, each of them are `false`.

The analysis process is a search for chats based on the following criteria:
* **Inactive** chat: the last message was more than 30 days ago or the time difference between sending the latest messages is too big
* **Leaved** chat in which you're the creator
* **Empty** chat or messages count in it too small

This process is quite long due to Telegram rate limits and may take several minutes.

Result example:
```bash
Leaved as creator: Channel(1423755780, title=Test, access_hash=1298224268170040224) (https://t.me/+1wf_5EfnX26mODgy)

Messages count too small: Channel(1224935112, @some, title=Some, access_hash=-479479826121742476)
  Msg(id=1, date=2020-04-22 15:50:43 UTC, action=Some(
    ChannelCreate(
      MessageActionChannelCreate {
        title: "Some",
      },
    ),
  ))
```

After you analyze chats, you can use one of the following commands to join or delete chat:
```bash
$ tg_old_chats_manager join --help
Join chat

Usage: tg_old_chats_manager join [OPTIONS] --id <ID>

Options:
  -i, --id <ID>                    Channel/supergroup ID to join
  -a, --access-hash <ACCESS_HASH>  Access hash of the channel/supergroup. It's required for most cases
  -h, --help                       Print help
```
```bash
$ tg_old_chats_manager delete --help
Delete chat

Usage: tg_old_chats_manager delete [OPTIONS] --id <ID>

Options:
  -i, --id <ID>                    Channel/supergroup ID to delete
  -a, --access-hash <ACCESS_HASH>  Access hash of the channel/supergroup. It's required for most cases
  -h, --help                       Print help
```

_P.S: A chat is a group, supergroup, or channel_
