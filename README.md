# Anime downloader (for a lack of a better name)

This tool makes use of ani-cli to download anime episodes to a designated
directory

# Dependency

Other than the obvious ani-cli, this program requires no runtime dependency.
Install it and you're set

# Motivation

I wanted something akin to the arr stack but uses direct downloads instead
since
- There aren't many public torrent trackers
- Many torrents tracked are dead torrents (have 0 seeders)
- Having to manually ssh into my Jellyfin server gets old after 2 - 3 times

# Configuration

All the configuration options for this program is contained within the
`config` table, as demonstrated by the example config below

```toml
[config]
# Seconds to sleep after checking for possible downloads
sleep_secs = sleep_secs
```
# Usage instructions

- Run the program once for it to generate it's configuration directory or
  create the directory (and file) `~/.config/anime-downloader/watchlist.toml`
  yourself
- Write the config file as follow
```toml
[id]
name = "anime_name"
directory = "target_directory"
select = entry_number
current_episode = episode_number
```
where:
- `id` is a valid TOML table name
- `anime_name` is the name of the anime you're watching
- `target_directory` is the full raw path (/home/user/...) as environment
  variable substitution ($HOME/...) isn't supported yet nor is ~
- `entry_number` is the entry number to choose in case there are multiple
  matching titles
- `episode_number` is the episode number to begin download (the program will
  automatically increment this number)

# Special thanks

- [ani-cli](https://github.com/pystardust/ani-cli) For the download backend
