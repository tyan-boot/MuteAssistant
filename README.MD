# MuteAssistant

A tool to mute your microphone using keyboard hotkey.

# Download
Download pre-compiled binary from [release](https://github.com/tyan-boot/MuteAssistant/releases)

# Config
To use this tool, you must create a config file named `config.json` and place it with the same directory as binary.

__example config with hotkey `ALT+1`.__

```json
{
    "mods": ["0x1"],
    "key": "0x31"
}
```

__example config with hotkey `CTRL+ALT+1`.__

```json
{
    "mods": ["0x1", "0x2"],
    "key": "0x31"
}
```

`mods` is an array of modifiers keys, which values can be:

| value  |     key     |
| ------ | ----------- |
| 0x0001 | ALT key     |
| 0x0002 | CTRL key    |
| 0x0004 | SHIFT key   |
| 0x0008 | WINDOWS key |

`key` is the virtual-key code of the hot key, which can be found in [Virtual Key 
Codes](https://docs.microsoft.com/zh-cn/windows/win32/inputdev/virtual-key-codes)

# Todo
- [ ] Monitor default capture device change.
- [ ] GUI
- [ ] Toast to show mute state.

# Motivation
When i was using ThinkPad, there is a special fn key in keyboard to mute microphone and a led indicates mute state.
It's useful when you want to mute microphone temporarily but don't want to switch to voice chat program, e.g. discord.

And now i switch from ThinkPad to Surface, it lacks such a convenient key to mute microphone, so i wrote `MuteAssistant`.

