# TechniHack — Notatki do obrony projektu

Ten plik jest przygotowany pod obronę ustną projektu: zawiera możliwe trudniejsze pytania i krótkie odpowiedzi oparte na aktualnym kodzie.

## 1) Architektura całości

### P: Dlaczego na `body` jest `display: grid` z `grid-template-rows: auto 1fr`?

**O:** Taki układ tworzy stałą górną sekcję (`header`) i elastyczną część główną (`main`), która wypełnia resztę wysokości okna. `auto` dopasowuje wysokość nagłówka do treści, a `1fr` zajmuje pozostałe miejsce.

### P: Dlaczego w selektorach są jednocześnie ID i klasy (np. `#header, .header`)?

**O:** HTML był refaktoryzowany. Obsługa obu sposobów nazewnictwa utrzymuje stabilne style po zmianach ID/klas i zapobiega „rozsypaniu” widoku podczas iteracji.

### P: Po co zmienne CSS w `:root` (`--bg`, `--line` itd.)?

**O:** Centralizują wartości motywu. Zmiana jednej zmiennej aktualizuje wszystkie miejsca użycia, więc łatwiej utrzymać spójność i później rozwijać projekt.

## 2) Responsywność (RWD)

### P: Dlaczego są dwa breakpointy (`1024px` i `700px`)?

**O:** `1024px` obsługuje tablet/mniejsze laptopy, a `700px` telefony. Dzięki temu nie ma jednego „zbyt ogólnego” trybu mobile i elementy pozostają czytelne w obu zakresach.

### P: Dlaczego na `body` jest `overflow: auto`, a nie `hidden`?

**O:** Na małych ekranach `hidden` może ucinać treść i blokować dostęp do części interfejsu. `auto` pozwala przewijać, gdy to konieczne.

### P: Dlaczego skróty na małych ekranach przechodzą z jednego wiersza do wielu?

**O:** `flex-wrap: wrap` przy `max-width: 700px` zapobiega wychodzeniu „chipów” poza ekran i poprawia dostępność bez wymuszania poziomego scrolla.

## 3) Decyzje CSS na poziomie komponentów

### P: Co daje `min-width: 0` na `#terminal`?

**O:** W grid/flex dzieci potrafią się przelewać, gdy domyślnie mają `min-width: auto`. `min-width: 0` pozwala im poprawnie się zwężać w kolumnie.

### P: Dlaczego `.mission--active` ma jednocześnie tło i lewą obwódkę?

**O:** To podwójny sygnał stanu aktywnego: wyraźny blok + marker krawędzi, dzięki czemu szybciej widać, która misja jest aktualna.

### P: Jak działają paski postępu bez dodatkowych elementów HTML?

**O:** `::after` rysuje tor paska, a `::before` rysuje wypełnienie. Szerokości (`67%`, `45%`) reprezentują wartość postępu.

### P: Dlaczego input ma `background: transparent; color: inherit; font: inherit; border: none`?

**O:** Dzięki temu pole `<input>` wizualnie wtapia się w terminalowy motyw, ale nadal pozostaje semantycznym i poprawnym elementem formularza.

## 4) Logika zegara w JavaScript

### P: Dlaczego elementy są pobierane na górze pliku (`clockDateElement`, `clockTimeElement`)?

**O:** Unikamy ponownych zapytań do DOM co sekundę. Kod jest czytelniejszy i minimalnie wydajniejszy.

### P: Dlaczego w `formatClockDate` i `formatClockTime` jest ręczne dopełnianie zer?

**O:** Żeby wymusić stały format 2-cyfrowy (`01`, `09` itd.) dla miesiąca/dnia/godziny/minuty/sekundy — to daje stabilny wygląd i przewidywalny format.

### P: Dlaczego `updateClock()` jest wywoływane raz przed `setInterval`?

**O:** Bez pierwszego wywołania placeholdery byłyby widoczne przez ~1 sekundę. Pierwsze wywołanie natychmiast pokazuje aktualny czas.

### P: Po co warunki `if (clockDateElement)` i `if (clockTimeElement)`?

**O:** To kod defensywny: jeśli kiedyś zmienią się ID albo elementów nie będzie, skrypt nie wywali błędu.

## 5) „Dogrywka” — pytania pogłębiające od nauczyciela

### P: Co poprawiłbyś jako następny krok?

**O:**

1. Zastąpić ręczne dopełnianie zer metodą `String(...).padStart(2, "0")` dla czytelności.
2. Po zakończeniu refaktoru zostawić jedną konwencję nazewnictwa (usunąć duplikaty selektorów).
3. Dodać interakcje klawiaturowe w input terminala (historia po `↑`/`↓`).
4. Przenieść szerokości pasków postępu do wartości dynamicznych sterowanych przez JS.

### P: Dlaczego bez frameworka?

**O:** Zakres projektu jest UI + prosta logika. Czyste HTML/CSS/JS utrzymuje lekkość projektu i dobrze pokazuje fundamenty webowe.

---

## Szybka checklista przed prezentacją

- Umieć wyjaśnić, czemu użyto `grid-template-rows: auto 1fr`.
- Umieć opisać, jak pseudo-elementy tworzą pasek postępu.
- Umieć wyjaśnić sens dwóch breakpointów i co każdy zmienia.
- Umieć wyjaśnić, czemu `updateClock()` jest przed `setInterval`.
- Mieć przygotowaną jedną konkretną propozycję rozwoju projektu.

## 6) Ostatnie aktualizacje (stan na teraz)

- Dodano mocniejszą responsywność (`1024px` i `700px`) z lepszym układaniem topbara, sidebara i skrótów.
- Powiększono logo (również w breakpointach), żeby branding był czytelniejszy.
- Dodano animację `logo-glitch` i później ją uspokojono: jest rzadsza oraz bardziej nieregularna.
- W JS status `TRACE` po starcie skryptu zmienia się na `ACTIVE` jako szybki wizualny dowód działania JavaScript.

### P: Jak pokazać nauczycielowi, że JS na pewno działa?

**O:** Najprostszy dowód to zmiana tekstu statusu `TRACE` z `UNACTIVE` na `ACTIVE` po załadowaniu skryptu. To jest widoczne od razu bez klikania.

### P: Co oznacza „bardziej unikalny glitch” w praktyce?

**O:** Zmieniono keyframes tak, żeby efekt nie pojawiał się regularnie co stały krok, tylko w nieregularnych „burstach”. Dzięki temu animacja wygląda naturalniej i mniej mechanicznie.
