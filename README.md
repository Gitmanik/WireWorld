# WireWorld

Implementacja automatu komórkowego [WireWorld](https://en.wikipedia.org/wiki/Wireworld) w języku Rust

### Obsługa
Program podczas startu próbuje wczytać plik __grid.txt__ zawierający 
stan gry. W przypadku braku lub błędnego pliku generowana jest pusta macierz 50x50.

Po najechaniu myszką w okno gry zostanie pokazana komórka, w której po kliknięciu __lewym przyciskiem myszy__ 
zostanie ustalony stan według aktualnego wyboru.

Można również przeciągnąć plik w okno gry - zostanie załadowana jego zawartość.

### Klawisze
* __Z__ - malowanie Przewodnika
* __X__ - malowanie Głowy
* __C__ - malowanie Ogona
* __V__ - malowanie Pustki
* __P__ - drukowanie aktualnego stanu gry do konsoli
* __S__ - zapisanie aktualnego stanu gry do pliku __current.txt__
* __Spacja__ - Pauzowanie gry


#### Struktura pliku grid.txt
Plik składa się z WxH znaków określających stan komórki w danej pozycji macierzy
* __.__ - pusta przestzeń
* __\>__ - Głowa
* __\<__ - Ogon
* __-__ - Przewodnik

### Wykorzystane biblioteki
* Nannou 0.19.0 [GitHub](https://github.com/nannou-org/nannou)