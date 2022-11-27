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
- Run `make watch` to continuously run Clippy and the unit tests
