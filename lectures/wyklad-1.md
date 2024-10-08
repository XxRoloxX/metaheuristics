# Wprowadzenie

- obecność nieobowiązkowa
- kolokwium: 5,6 pytań otwartych (5.12 i 12.12), karta i długopis
- projekt: zaliczenie laboratorium
- konsultacje zdalnie z potwierdzeniem mailem

# Projekt (własny problem)

- 1-2 osoby
- pomysł z firmy/zainteresowań
- zgłoszenie najpóźniej do 17.10
- wymaganie użycie 2 metaheurystyk i różnych danych

## Przykłady

- sale szpitalne
- planowanie zadań w fabrykach

### Problemy

#### TSP (Traveling salesman)

Sekwencja miast

#### GCP (Graph Coloring Problem)

Kolory użyte do kolorowania wierzchołków.
Zastosowanie międzyinnymi w telekomunikacji w rozstawiania masztów nie
nadających na tej samej częstotliwości

#### MS-RCPSP (Multi skill resource constrained project scheduling problem)

- zasób, umiejętność, zadanie

#### UCTP (Plan na studia)

- student, zajecia, sala, prowadzący

#### TTP (Traveling Thief Problem) = TSP + KNP

- sekwencja miast + przedmioty do plecaka
- opłaty za zapełnienie plecaka

#### Rozwiązanie

- wiele akceptowalnych rozwiązań
- wektor/liczba

# Analiza problemu

- ograniczenia
- funkcje oceny
- przestrzeń rozwiązań
- metoda rozwiązania (alogytm konstrukcyjny -> np.
  wybieranie najbliższego miasta do problemu komiwojażera (zachłanne))

# Definicje (Możliwe pytania)

## Algorytm

Zestaw instrukcji o określonej złożoności (wiemy kiedy się skończy) oraz dający najlepsze rozwiązanie

## Heurystyka

Metoda pozwalająca na rozwiązanie problemu o danej złożoności, ale nie gwarantuje rozwiązania optymalnego (ale tu i teraz)

## Metaheurystyka

Heurystyka decydująca o użytych heurystykach (używamy kiedy mamy czas xd)

- Wybór losowy
- Algorytm zachłanny
- Heurystyka
- Przegląd konstrukcujny (optymalizacja z przeszukiwaniem drzewa)
- Przegląd zupełny

## Skuteczność a efektywność

### Skuteczność

- efektywność: lepszy np. czas wykonania na to samo rozwiązanie. C jest mniej efektywna ale bardziej skuteczna
- skuteczność (lepsze rozwiązanie)

## Definicje cd.

- _Optymalizacja_: minimalizacja lub maksymalizacja
- _Sąsiedztwo punktów_: punkty rozdzielone jedną operacją (przejście/mutacja/operator)
- _Funkcje celu, fitness_: funkcja definiująca skuteczność rozwiązania
- _Przeszukiwanie przestrzeni_: w trakcie iteracji rozwiązujemy wiele rozwiązań
- _Eksploracja/eksploatacja_: szukanie obiecujących rozwiązań, weryfikowanie jakość rozwiązań. Duża ekspolaracja możemy dużo pominąć.
  Nadmierna eksploatacja powoduje utykanie w lokalnych optimach

## Niedeterminizm w metaheurysykach (nie losowość)

### Zalety

- zmniejszenie szans na utknięcie w optimach lokalnych

### Wady

- niepowtarzalność wyników

# Sztuczna inteligencja

## Mocna sztuczna inteligencja

- Test turinga / Chiński pokój (eksperyment myślowy, losowe próbowanie wszystkich opcji)

## Słaba sztuczna inteligencja

- Inteligencja obliczeniowa, obliczenia miękkie, rojowa, metaheurystyki

# Metaheurystyki

- wzorowane na naturze (przystosowanie i póla genetyczna)

## Generyczna metaheurystyka

- reprezentacja rozwiązania
- sąsiedztwo (zmiana rozwiązania)
- funkcja oceny (jakości)
- warunek zatrzymania
- liczba punktów przeszukiwania (próbkowania)

# Zastosowanie

- kiedy nie mamy gotowego algorytmu
- jeżeli mamy dużo czasu
- Jeżeli mamy reprezentacje, sąsiedztwo i funkcje
- Przykłady kiedy nie stosować (Totolotek, strategia na giełdzie, łamanie szyfru)

# Przykłady

- PSO (mrówkowy)
- SA - schładzanie stopu (wyrzażanie)
- TS - metoda prób i błędów z pamięcią
- AIS "naturalna" klasyfikacja w organizmie
- EA - naśladowanie ewolucji naturalnej

# Harmonogramowanie

- zasoby, zadania, ograniczenia, terminy
- osobnikiem jest połączenie powyższych, a oceną jest czas i koszt harmonogramu

# Plan zajęć

- osobnikiem jest plan (studenci przypisani do określonych zajęć o określonej godzinie, miejscu)
- oceną jest spełnienie ograniczenia twarde oraz miękkie takie jak okienka, wygodne godziny rozpoczynania i kończenia zajęć

# Literatura

- jak to rozwiązać czyli to nowoczesna heurystyka (David B. Fogiel, Zbigniew Michalewicz)

Pytania pod koniec wykładu mogą się pojawić na egzaminie

Hasło do kursu na e-portalu: Zima#2204

# Architektura:

- Problem z pliku

## Problem

- optymalizacja z wczesnym przeliczenie odległości (w postaci macierzy): eval()

## EA

- config (może CLI, albo config w jsonie), warunek stopu,
  wielkość populacji
- populacja właściwa, populacja+1 :selection(), best(), worst()

## Osobnik z Populacji

- init()
- mutate()
- crossover()

## Logger

- co pokolenie wysyła statystyki populacji
