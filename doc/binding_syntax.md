Syntax for key bindings
=======================

This document defines the syntax of key bindings. This can be parsed by `KeySeq::parse`, `KeyInput::parse`,
and `KeybindDispatcher::bind`.

## Key binding examples

Here are some examples of key bindings with US keyboard.

| Notation          | Corresponding key input                                                                       |
|-------------------|-----------------------------------------------------------------------------------------------|
| `"a"`             | <kbd>A</kbd>                                                                                  |
| `"X"`             | <kbd>Shift</kbd> + <kbd>X</kbd>                                                               |
| `"?"`             | <kbd>Shift</kbd> + <kbd>/</kbd>                                                               |
| `"Ctrl+t"`        | <kbd>Ctrl</kbd> + <kbd>T</kbd>                                                                |
| `"Alt+M"`         | <kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>M</kbd>                                              |
| `"Enter"`         | <kbd>Enter</kbd>                                                                              |
| `"Ctrl+Enter"`    | <kbd>Ctrl</kbd> + <kbd>Enter</kbd>                                                            |
| `"Shift+Up"`      | <kbd>Shift</kbd> + <kbd>↑</kbd>                                                               |
| `"a b c"`         | <kbd>A</kbd> → <kbd>B</kbd> → <kbd>C</kbd>                                                    |
| `"Ctrl+x Ctrl+s"` | <kbd>Ctrl</kbd> + <kbd>X</kbd> → <kbd>Ctrl</kbd> + <kbd>S</kbd>                               |
| `"Mod+x"`         | <kbd>Command</kbd> + <kbd>X</kbd> on macOS, <kbd>Ctrl</kbd> + <kbd>X</kbd> on other platforms |
| `"Super+x"`       | <kbd>Command</kbd> + <kbd>X</kbd> on macOS, <kbd>Win</kbd> + <kbd>X</kbd> on other platforms  |

## Grammar

This is the grammar of key binding representation in [W3C EBNF notation][ebnf].

```ebnf
key-binding     ::= key-sequence
key-sequence    ::= key-combination ((space)+ key-combination)*
space           ::= ' ' | '\t'
key-combination ::= (modifier '+')* key
modifier        ::= 'Control' | 'Ctrl' | 'Command' | 'Cmd' | 'Mod' | 'Alt' | 'Super' | 'Option' | 'Shift' |
                    'control' | 'ctrl' | 'command' | 'cmd' | 'mod' | 'alt' | 'super' | 'option' | 'shift' |
                    'CONTROL' | 'CTRL' | 'COMMAND' | 'CMD' | 'MOD' | 'ALT' | 'SUPER' | 'OPTION' | 'SHIFT'
key             ::= character-key | named-key | function-key
character-key   ::= /* Any unicode character except for spaces */
named-key       ::= 'Space' | 'Plus' | 'Up' | 'Right' | 'Down' | 'Left' | 'Enter' | 'Backspace' | 'Delete' | 'Home' | 'End' | 'PageUp' | 'PageDown' | 'Esc' | 'Tab' | 'Backtab' | 'Insert' | 'Copy' | 'Cut' | 'Paste' | 'Clear' | 'Undo' | 'Redo' | 'ZoomIn' | 'ZoomOut' | 'ScrollLock' | 'NumLock' | 'FnLock' | 'PrintScreen' | 'Menu' | 'Play' | 'Pause' | 'PlayPause' | 'Stop' | 'Rewind' | 'NextTrack' | 'PrevTrack' | 'VolumeUp' | 'VolumeDown' | 'Mute' |
                    'space' | 'plus' | 'up' | 'right' | 'down' | 'left' | 'enter' | 'backspace' | 'delete' | 'home' | 'end' | 'pageup' | 'pagedown' | 'esc' | 'tab' | 'backtab' | 'insert' | 'copy' | 'cut' | 'paste' | 'clear' | 'undo' | 'redo' | 'zoomin' | 'zoomout' | 'scrolllock' | 'numlock' | 'fnlock' | 'printscreen' | 'menu' | 'play' | 'pause' | 'playpause' | 'stop' | 'rewind' | 'nexttrack' | 'prevtrack' | 'volumeup' | 'volumedown' | 'mute' |
                    'SPACE' | 'PLUS' | 'UP' | 'RIGHT' | 'DOWN' | 'LEFT' | 'ENTER' | 'BACKSPACE' | 'DELETE' | 'HOME' | 'END' | 'PAGEUP' | 'PAGEDOWN' | 'ESC' | 'TAB' | 'BACKTAB' | 'INSERT' | 'COPY' | 'CUT' | 'PASTE' | 'CLEAR' | 'UNDO' | 'REDO' | 'ZOOMIN' | 'ZOOMOUT' | 'SCROLLLOCK' | 'NUMLOCK' | 'FNLOCK' | 'PRINTSCREEN' | 'MENU' | 'PLAY' | 'PAUSE' | 'PLAYPAUSE' | 'STOP' | 'REWIND' | 'NEXTTRACK' | 'PREVTRACK' | 'VOLUMEUP' | 'VOLUMEDOWN' | 'MUTE'
function-key    ::= 'F' [1-9] ([0-9])*
```

## Key combination

Key combination is a combination of key strokes like `a`, `Enter`, `Ctrl+Alt+a`. Modifiers are concatenated
with `+` and precedes a normal key. No space is allowed between characters because a space represent a sequence.

Normal keys are a single character (e.g. `a`, `X`, `あ`) or a named key (e.g. `Up`, `Enter`, `Tab`). Note that
the characters are case-sensitive. `A` means typing <kbd>A</kbd> and <kbd>Shift</kbd> keys on US keyboard.

These keys are **logical** keys which are inputs as the result of key typing. In comparison, physical keys are
actual keys on your keyboard. For example, typing the physical keys <kbd>Shift</kbd> and <kbd>9</kbd> produces
the logical key `(` with US keyboard, and it also produces the logical key `)` with JP keyboard.

## Key sequence

Key sequence is a sequence of key combinations. Key combinations are concatenated with one or more spaces like
`a b` or `Ctrl+x Ctrl+s`. Spaces prefixed or suffixed to a sequence are ignored.

## Modifiers

The following modifier keys are available:

- `Ctrl`: <kbd>Ctrl</kbd> key
- `Cmd`: <kbd>Command</kbd> key
- `Mod`: <kbd>Command</kbd> key on macOS, <kbd>Ctrl</kbd> key on other platforms
- `Super`: <kbd>Windows</kbd> key on platforms other than macOS, Command key on macOS
- `Alt`: <kbd>Alt</kbd> key
- `Option`: An alias to <kbd>Alt</kbd> key
- `Shift`: <kbd>Shift</kbd> key

> [!Caution]
>
> `Shift` modifier key is only available with named keys like `Shift+Up`. For example, when you want to
> define key binding for <kbd>Shift</kbd> + <kbd>A</kbd>, you need to use `A` instead of `Shift+a`. This
> restriction is for avoiding some confusing edge cases and may be relaxed in the future.

## Named keys

The following modifier keys are available.

- `Space`
- `Plus`
- `Up`
- `Right`
- `Down`
- `Left`
- `Enter`
- `Backspace`
- `Delete`
- `Home`
- `End`
- `PageUp`
- `PageDown`
- `Esc`
- `Tab`
- `Backtab`
- `Insert`
- `Copy`
- `Cut`
- `Paste`
- `Clear`
- `Undo`
- `Redo`
- `ZoomIn`
- `ZoomOut`
- `ScrollLock`
- `NumLock`
- `FnLock`
- `PrintScreen`
- `Menu`
- `Play`
- `Pause`
- `PlayPause`
- `Stop`
- `Rewind`
- `NextTrack`
- `PrevTrack`
- `VolumeUp`
- `VolumeDown`
- `Mute`
- `F1`, `F2`, `F3`, ...

[ebnf]: https://www.w3.org/TR/2008/REC-xml-20081126/#sec-notation
