# Podziały problemów na podproblemy

Przykład kiedy taki podział nie działa:

- TSP+KNP, drastycznie powiększa nam się przestrzeń rozwiązań

- Mając prioritety w osobniku możemy wykonać mutację Gaussa

## Differential Evolution -> algorytm ewolucyjny na przestrzeni ciągłej

# GIGO (Garbage in, garbage out)

# Algorytm zachłanny?

Patrzymy zamiast jednego, dwa miasta do przodu.
Przeglądanie pełne ->n, algorytm zachłanny ->1
Peckish -> pomiędzy tymi powyżej

# 2-opt

Działa na osobniku i odpętlna złe krawędzie. Działa do dwóch krawędzi do przodu.
Dzięki temu rozwiązuje supły

# Lokalne przeszukiwanie (Hill climbing)

- znajdz najlepszego sąsiada
- nie dba o optima globalne
- próbkujemy wiele rozwiązań początkowych aby znaleźć jak najwięcej optimów (i jak najlepsze xd)
- im więcej sąsiadów próbkujemy tym droższe jest uruchomienie algorytmu
- problem z przestrzeniami _plateu_ oraz _lisimi dołami_

# Taboo search

- zapamiętywanie drogi + przeszukiwanie lokalne

## Istotne parametry

- rozmiar list taboo, musi być na tyle dużo żeby się nie powtarzać.
  Nie może też być zbyt dużo bo każda iteracja będzie dużo droższa.
  Taboo czasami lubi utknąć w lokalnym optimum.

- aspiracja: zignorowanie listy taboo

- jak pamiętać listę taboo? (rozwiązanie, albo operacja)

# Symulowane wyżarzanie

- mało parametrów
- najlepsze?
- przeszukiwanie lokalne
- temperatura: podatność na błędne rozwiązania
-
