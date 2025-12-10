# MORSE Language Examples

## Example 1: Basic IF statement

```morse
.. ..-./ 10 > 5
    .--. .-. .. -. -/ "Hello Morse World!"
.-. -../
```

## Example 2: Multiple conditions (future)

```morse
.. ..-./ 15 > 10
    .--. .-. .. -. -/ "Fifteen is greater"
. .-.. ... ./
    .--. .-. .. -. -/ "Not greater"
.-. -../
```

## Example 3: Variables (future with SET)

```morse
... . -/ x = 42
.--. .-. .. -. -/ x

.. ..-./ x > 40
    .--. .-. .. -. -/ "Big number"
.-. -../
```

## Example 4: While loop (future)

```morse
... . -/ count = 0

.-- .... .. .-.. ./ count < 3
    -.. ---/
        .--. .-. .. -. -/ count
        ... . -/ count = count + 1
    .-. -../
.-. -../
```

## Quick Reference

| What | Morse | Example |
|------|-------|---------|
| IF | `.. ..-./` | `.. ..-./ x > 5` |
| PRINT | `.--. .-. .. -. -/` | `.--. .-. .. -. -/ "text"` |
| END | `.-. -../` | `.-. -../` |
| SET | `... . -/` | `... . -/ x = 10` |
| WHILE | `.-- .... .. .-.. ./` | `.-- .... .. .-.. ./ x < 5` |
