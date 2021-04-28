# rsa

```
cargo run --bin encoder encode %file_name%
cargo run --bin encoder e %file_name%
```


Чтобы дешифровать нужно вводить оригинальное имя файла!!!
Допустим шифруем poem.txt
Получили encoded_poem.txt, но для расшифровки вводим poem.txt
```
cargo run --bin ecoder decode %file_name%
cargo run --bin encoder d %file_name%
```
