# 6. Zigzag Conversion

Строка `"PAYPALISHIRING"` записана зигзагом в заданное число строк вот так (лучше смотреть моноширинным шрифтом):

```text
P   A   H   N
A P L S I I G
Y   I   R
```

Если читать построчно, получится: `"PAHNAPLSIIGYIR"`.

Напишите функцию, которая принимает строку и число строк и выполняет такое преобразование:

```text
string convert(string s, int numRows);
```

## Пример 1

Вход: s = "PAYPALISHIRING", numRows = 3

Выход: "PAHNAPLSIIGYIR"

## Пример 2

Вход: s = "PAYPALISHIRING", numRows = 4

Выход: "PINALSIGYAHRPI"

Пояснение:

```text
P     I    N
A   L S  I G
Y A   H R
P     I
```

## Пример 3

Вход: s = "A", numRows = 1

Выход: "A"

## Ограничения

- 1 <= s.length <= 1000
- `s` состоит из английских букв (строчных и заглавных), `,` и `.`
- 1 <= numRows <= 1000
