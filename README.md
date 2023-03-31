[![codecov](https://codecov.io/gh/lonepeon/tableformat/branch/main/graph/badge.svg?token=HC7WSVDRO2)](https://codecov.io/gh/lonepeon/tableformat)

# Table format

Small tool accepting a Markdown table as input and generating a new one properly aligned.

```
# Before

| a| b| c| d|
|--|:--|--:|:-:|
| value a1 | value b1 | value c1 | value d1 |
| longer value a2 | longer value b2 | longer value c2 | longvalue d2 |

# After

| a               | b               |               c |      d       |
|-----------------|:----------------|----------------:|:------------:|
| value a1        | value b1        |        value c1 |   value d1   |
| longer value a1 | longer value b2 | longer value c2 | longvalue d2 |
```

## Development

- Setup your local environment using `make setup`
