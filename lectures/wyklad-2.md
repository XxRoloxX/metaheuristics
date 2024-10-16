# Algorytm evolucyjny

### Inicjalizacja

- wybieramy punkty startowe

### Ocena

### Kryterium zatrzymania

- liczba pokoleń
- czas wykonania
- jakość rozwiązania
- moment ujednolicenia wszystkich osobników

### Selekcja

-- Operatory modyfikujące --

### Krzyżowanie

- zbyt dużo krzyżowań może spowodować dojście do optimum lokalnego

### Mutacja

- dużo mutacji generuje "chaotyczne" rozwiązania

### Reprezentacja rozwiązania

- binarna
- binarna kodem Greya
- rzeczywisto-liczbowa
- kombinatorycznaa: występują powtórzenia
- mieszana:połączenie powyższych

### Różnica w zmienie rozwiązania

- Problem kolorowania
- zmiana 535 na 353 nie robi nam różnicy (dla klasycznego problemu komiwojażera)

## Problem grafu

- _NP-trudny_

- wprowadzenie funkcji kary aby uniknąć oceny w postaci Prawda/Fałsz
- zmiana problemu decyzyjnego na optymalizacyjny
- funkcja kary obniża jakość rozwiązania za każde złamanie ograniczenia
- przydatne gdy poszukujemy "równomierności"

## Funkcja oceny

- musi być deterministyczna (pomijająć dynamiczną funkcje oceny)

## MS-RCPSP (harmonogramowani)

- genotyp: ciężko ocenić
- fenotyp: na podstawie genotypu dopiero możemy ocenić

Dwa różne genotypy mogą mieć ten sam fenotyp.

## Mierzenie różnorodności rozwiązań

- odległość Hamminga (duża złożoność) (n^2 / 2)

## Badanie populacji (generacji)

- najlepszy osobnik
- najgorszy osobnik
- średni osobnik

Na podstawie różnorodności na podstawie róznic mędzy
najlepszy a najgorszym osobnikiem w kolejnych populacjach

## Operatory

### Mutacja

- wprowadzenie zmiany, która nie spowoduje błędu w osobniku
- w problemie TSP zmiana miejscami
- jeżeli osobnik zostanie "zepsuty" to musimy go "naprawić"
  (lub wprowadzamy funkcje kary)

### Krzyżowanie

- jednopunktowe: bierzemy część genotypu od jednego punktu w lewo i z drugiego w prawo
- wielopunktowe: analogicznie co jednopunktowy, ale z wieloma punktami
- jednorodne: wybieramy losowe geny z obu rodziców

Do problemy TSP lepszy będzie jednopunktowy/wielopunktowy od jednorodnego

### Selekcja

- Elitarna: (bierzemy zawsze %n najlepszych)
- Ruletka: prawdopodobieństwo jest zależne od jakości osobnika, nie sprawdza się dla podobnych prawdopodobieńst
- Rankingowa: sortowanie i wybór n osobników (drogie sortowanie, dlatego nie używajmy go jak nie musimy)
- Turniejowa: <1, rozmiar_populacji> bierzemy najlepszego osobnika

#### Turniej

- ze zwracaniem
- bez zwracania

### Inicjalizacja

- Losowe (EA startuje z losowych miejsc w przestrzeni)

- Heurystyczna (np. zachłanna)

## Problemy

- za duża/mała mutacja
- krzyżowanie
- populacja
- pokoleń

- osobniki zbyt podobne
- identyczne osobniki
- gubienie dobrych osobników
- nie wychodzimy z lokalnego optimum

## Crowding distance

- jeżeli dużo osobników wchodzi na lokalne optimum
  to możemy je "odstraszyć" karą za "tłok"

## Ograniczenia

- niepełne,redundantne, błednę rozwiązanie

### Naprawa błędów

### Karanie

### Pomijanie błędnych osobników

## Parametry AE

- rozmiar populacji
- liczba pokoleń
- prawdopodobieństwo krzyżowania
- prawdopowodieństwo mutacji
- parametr selekcji
- parametr inicjalizacji (random)

A wartości parametrów wybieramy eksperymentalnie (na razie lepiej unikać dodatkowych parametrów)

Wyjściowe parametry (100 generacji x 100 osobników)

# Optymalizacja wielokryterialna

- wiele funkcji oceniających rozwiązanie
