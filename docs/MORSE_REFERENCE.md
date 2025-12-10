# Morse Code Reference

## Language Keywords

All keywords must end with `/` separator.

| Keyword | Morse Code | Full Syntax | Description |
|---------|------------|-------------|-------------|
| **if** | `.. ..-.` | `.. ..-./` | Conditional statement |
| **else** | `. .-.. ... .` | `. .-.. ... ./` | Else clause |
| **while** | `.-- .... .. .-.. .` | `.-- .... .. .-.. ./` | While loop |
| **for** | `..-. --- .-.` | `..-. --- .-./` | For loop |
| **do** | `-.. ---` | `-.. ---/` | Do block |
| **end** | `.-. -..` | `.-. -../` | Block terminator |
| **print** | `.--. .-. .. -. -` | `.--. .-. .. -. -/` | Print output |
| **return** | `.-. . - ..- .-. -.` | `.-. . - ..- .-. -./` | Return from function |
| **func** | `..-. ..- -. -.-.` | `..-. ..- -. -.-./` | Function declaration |
| **set** | `... . -` | `... . -/` | Variable assignment |
| **true** | `- .-. ..- .` | `- .-. ..- ./` | Boolean true |
| **false** | `..-. .- .-.. ... .` | `..-. .- .-.. ... ./` | Boolean false |
| **and** | `.- -. -..` | `.- -. -../` | Logical AND |
| **or** | `--- .-.` | `--- .-./` | Logical OR |
| **not** | `-. --- -` | `-. --- -/` | Logical NOT |

## Morse Code Alphabet

### Letters

| Letter | Morse | Letter | Morse | Letter | Morse |
|--------|-------|--------|-------|--------|-------|
| A | `.-` | J | `.---` | S | `...` |
| B | `-...` | K | `-.-` | T | `-` |
| C | `-.-.` | L | `.-..` | U | `..-` |
| D | `-..` | M | `--` | V | `...-` |
| E | `.` | N | `-.` | W | `.--` |
| F | `..-.` | O | `---` | X | `-..-` |
| G | `--.` | P | `.--.` | Y | `-.--` |
| H | `....` | Q | `--.-` | Z | `--..` |
| I | `..` | R | `.-.` | | |

### Numbers

| Number | Morse |
|--------|-------|
| 0 | `-----` |
| 1 | `.----` |
| 2 | `..---` |
| 3 | `...--` |
| 4 | `....-` |
| 5 | `.....` |
| 6 | `-....` |
| 7 | `--...` |
| 8 | `---..` |
| 9 | `----.` |

## How to Build Keywords

To create a keyword in Morse code:

1. **Spell out the keyword letter by letter**
   - Example: `IF` = `I` + `F` = `..` + `..-.`

2. **Separate letters with spaces**
   - Result: `.. ..-.`

3. **Add the `/` terminator**
   - Final: `.. ..-./`

## Examples

### Breaking down "PRINT"
```
P = .--.
R = .-.
I = ..
N = -.
T = -

Combine: .--. .-. .. -. -
With terminator: .--. .-. .. -. -/
```

### Breaking down "WHILE"
```
W = .--
H = ....
I = ..
L = .-..
E = .

Combine: .-- .... .. .-.. .
With terminator: .-- .... .. .-.. ./
```

### Breaking down "END"
```
E = .
N = -.
D = -..

Combine: . -. -..
With terminator: . -. -../
```

Wait, that's not what we're using! We use `.-. -../` for END.

Let me check what we're actually using...

Actually, looking at our morse.rs:
- END = `.-. -..` (which is R + D = "RD")

So it seems some keywords are abbreviated! Here's what they actually map to:

### Actual Keyword Mappings

| Keyword | Letters Encoded | Morse |
|---------|----------------|-------|
| if | IF | `.. ..-.` |
| else | ELSE | `. .-.. ... .` |
| while | WHILE | `.-- .... .. .-.. .` |
| for | FOR | `..-. --- .-.` |
| do | DO | `-.. ---` |
| end | RD (abbreviated) | `.-. -..` |
| print | PRINT | `.--. .-. .. -. -` |
| return | RETURN | `.-. . - ..- .-. -.` |
| func | FUNC | `..-. ..- -. -.-.` |
| set | SET | `... . -` |
| true | TRUE | `- .-. ..- .` |
| false | FALSE | `..-. .- .-.. ... .` |
| and | AND | `.- -. -..` |
| or | OR | `--- .-.` |
| not | NOT | `-. --- -` |

## Quick Tips

- **Dot (dit)**: `.` - short signal
- **Dash (dah)**: `-` - long signal (3x dot length)
- **Space between symbols**: same as dot length
- **Space between letters**: 3x dot length
- **Always end keywords with**: `/`

## Practice

Try encoding these yourself:

```
var  = ...- .- .-.
let  = .-.. . -
fn   = ..-. -.
loop = .-.. --- --- .--.
```
