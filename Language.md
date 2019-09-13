
A file editing language inspired by vim

```
move '3j'
yank '3w'
delete 'd]'

insert "# some comment"

group move '1j' {
  insert '  '
}

search /def test/ {
  append_line "'''\ntestcase\n'''"
  insert '  '
}

group search /def test/ move '1j' {
  insert_line "'''\ntestcase\n'''"
  insert '  '
}

# comment
```




