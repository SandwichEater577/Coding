# TechniHack - Q&A do obrony (trudne pytania)

Ten README jest celowo w formie pytań i odpowiedzi pod obronę projektu.
Zakres: tylko to, co realnie jest w `index.html`, `Assets/style.css`, `Assets/reset.css`, `js/data.js`, `js/ui.js`, `js/commands.js`, `js/main.js`.

## HTML (`index.html`)

### P: Czemu skrypty są wpięte z atrybutem `defer` i rozbite na pliki?

**O:** `defer` gwarantuje niewstrzymywanie parsowania HTML - przeglądarka buduje najpierw DOM. Dodatkowo skrypty wykonują się w kolejności w jakiej zostały dodane do strony (najpierw dane i UI, w końcu main ze zdarzeniami). Dzięki temu unikamy konfliktów oraz zyskujemy mniejsze i bardziej czytelne pliki.

### P: Po co `aria-label="terminal input"` na `<input>`?

**O:** To poprawia dostępność dla czytników ekranu. Input nie ma widocznego `<label>`, więc `aria-label` dostarcza nazwę kontrolki technologii asystującej.

### P: Dlaczego struktura layoutu to `header + main`, a nie wszystko w samych `div`?

**O:** To semantyka HTML5. `header` komunikuje sekcję nagłówkową aplikacji, `main` zawiera główną treść. Pomaga to w dostępności, SEO i czytelności architektury.

### P: Czemu elementy mają jednocześnie klasy i ID?

**O:** ID są wygodne dla JS (`#clockTime`, `#mission1`), klasy do wielokrotnego stylowania (`.mission`, `.shortcut`). To rozdział odpowiedzialności: JS targetuje unikalne punkty, CSS obsługuje wzorce.

### P: Czemu w terminalu output jest `div`, a nie np. `p`?

**O:** Zawartość jest wielowierszowa i składa się z grup statusów systemowych. `div` daje pełną kontrolę blokową i łatwiejsze składanie sekcji pseudo-terminala.

## CSS (`Assets/style.css`)

### P: Czemu layout główny jest na `body` jako grid: `grid-template-rows: auto 1fr`?

**O:** `auto` dopasowuje wysokość nagłówka do treści, `1fr` bierze resztę ekranu dla części roboczej. To prosty i stabilny wzorzec „stała góra + elastyczny dół”.

### P: Czemu jest tyle selectorów łączonych, np. `#header, .header`?

**O:** To warstwa kompatybilności po refaktorach nazw. Chroni UI przed regresją, jeśli w HTML użyta jest wersja klasowa lub ID. Cena: większa złożoność CSS.

### P: Po co `min-width: 0` na `#terminal`?

**O:** W grid/flex dzieci domyślnie nie zawsze chcą się zwężać (`min-width: auto`). `min-width: 0` pozwala terminalowi realnie się kurczyć i zapobiega overflow poziomemu.

### P: Jak zrobione są paski postępu bez dodatkowych elementów HTML?

**O:** Tor paska jest rysowany przez `::after`, a wypełnienie przez `::before`. `#progressMission::before { width: 67%; }` i `#progressTime::before { width: 45%; }` określa aktualny poziom.

### P: Dlaczego są dwa breakpointy: `1024px` i `700px`?

**O:** `1024px` to „tablet/laptop compact”, gdzie header przechodzi na 2 kolumny i stats schodzą niżej. `700px` to telefon, gdzie układ jest jednokolumnowy, mniejsze fonty i skróty (`shortcuts`) zawijają się (`flex-wrap: wrap`).

### P: Czemu input dziedziczy styl (`color: inherit; font: inherit`) i ma transparentne tło?

**O:** Dzięki temu input wygląda jak część terminala, a nie osobny „webowy” komponent. Nadal jest to poprawny semantycznie input formularza.

### P: Co jest „trudne” w animacji `logo-glitch`?

**O:** Keyframes są celowo nieregularne (np. 72.4%, 73.1%, 79.3%, 92.7%), więc glitch wygląda mniej mechanicznie. To bardziej wiarygodny efekt niż idealnie równy rytm.

## Reset (`Assets/reset.css`)

### P: Po co reset z `* { margin: 0; padding: 0; box-sizing: border-box; }`?

**O:** Usuwa różnice domyślnych stylów przeglądarek i upraszcza obliczanie rozmiarów. `border-box` powoduje, że padding i border liczą się do deklarowanej szerokości/wysokości.

### P: Czy `body { background: rgb(100, 155, 100); }` w reset.css nie konfliktuje ze style.css?

**O:** Konflikt jest zamierzony i bezpieczny. W `style.css` później ustawiane jest `background: var(--bg)`, więc finalnie wygrywa styl z pliku ładowanego później. To pokazuje kaskadę CSS w praktyce.

### P: Czemu resetuje się też `button` i `input`?

**O:** Domyślne style kontrolek są mocno zależne od przeglądarki/OS. Reset daje neutralny punkt startowy, a docelowy wygląd jest dopiero w `style.css`.

## JavaScript (Pliki w folderze `js/`)

### P: Czemu projekt został podzielony na kilka plików zamiast jednego monolitu `script.js`?

**O:** Pozwala to na lepszą organizację kodu wynikowego. `data.js` przechowuje dane konfiguracyjne i referencje DOM, `ui.js` animacje i wizualizacje, `commands.js` to silnik komend w terminalu (włącznie z easter-egg "ADI HACKER") a `main.js` podpina nasłuchiwacze i bootstrapuje grę. Taka modularyzacja ułatwia zrozumienie kodu.

### P: Czemu logowanie Easter Egg'a generuje styl CSS bezpośrednio w JS zamiast w zewnętrznym pliku?

**O:** "ADI HACKER" to easter egg - jego efekty są wstrzykiwane bezpośrednio przez headera gdy użyje się ukrytej komendy co sprawia wrażenie "włamania" w strukturę DOM.

### P: Czemu wszędzie są guardy typu `if (clockTimeElement)`?

**O:** To defensywne programowanie. Jeśli ktoś zmieni HTML albo usunie element, aplikacja nie wywali się `Cannot read properties of null`.

### P: Jak działa zegar i czemu aktualizuje się co sekundę?

**O:** `updateClock()` buduje nowy `Date`, formatuje datę (`YYYY.MM.DD`) i czas (`HH:MM:SS`), potem wpisuje do DOM. `setInterval(updateClock, 1000)` odświeża co sekundę.

### P: Dlaczego `updateClock()` jest wywołane przed `setInterval`?

**O:** Żeby nie pokazywać placeholdera (`YYYY.MM.DD` i `HH:MM:SS`) przez pierwszą sekundę po załadowaniu strony.

### P: Skąd publiczne IP i co jeśli API nie odpowie?

**O:** IP pobierane jest przez `fetch("https://api.ipify.org?format=json")`. W trakcie ładowania jest `LOADING...`, po sukcesie kolor statusu przechodzi na `--info`, a przy błędzie ustawiane jest `N/A` i kolor błędu.

### P: Czemu użyto `innerHTML` do renderu terminala, zamiast tworzyć każdy element przez `createElement`?

**O:** Tu to świadomy trade-off: szybkie renderowanie dużego, stałego bloku „ekranu terminala”. Minusem jest mniejsza granularność i potencjalne ryzyko XSS przy danych z zewnątrz. W tym kodzie treść jest statyczna, więc ryzyko jest ograniczone.

### P: Jak pokazać od razu, że JS działa?

**O:** Status `TRACE` startuje jako `UNACTIVE` w HTML i po uruchomieniu skryptu zmienia się na `ACTIVE` z kolorem `--info`. To natychmiastowy, widoczny dowód wykonania JS.

### P: Jaki jest aktualny „trudny” bug architektoniczny i jak go wyjaśnić?

**O:** `setActiveMission()` przełącza klasę `active`, ale CSS styluje `.mission--active`. To znaczy, że przy klikaniu misji logika i styl nie są w pełni zsynchronizowane. Poprawka: ujednolicić nazwę klasy po obu stronach.

## Pytania przekrojowe (najtrudniejsze)

### P: Dlaczego ten projekt nie używa frameworka?

**O:** Cel projektu to pokazanie fundamentów frontendu: semantyka HTML, kaskada i RWD w CSS, oraz czysty DOM/API w JS. Framework ukryłby część mechaniki, którą tu trzeba świadomie obronić.

### P: Co byś poprawił w pierwszej kolejności technicznie?

**O:**

1. Ujednolicić klasę aktywnej misji (`active` vs `mission--active`).
2. Wynieść inline style z `renderMission1Output()` do klas CSS.
3. Dodać obsługę klawiszy `Enter`, `ArrowUp`, `ArrowDown` dla terminala.
4. Zmienić statyczne szerokości progress barów na wartości sterowane przez JS.

## 30-sekundowa odpowiedz na "co tu jest najwazniejsze"

Projekt to terminalowy interfejs webowy oparty o semantyczny HTML, responsywny CSS Grid/Flex i JS bez frameworka. Najwazniejsze elementy techniczne to: defensywna praca na DOM, zegar czasu rzeczywistego, pobieranie publicznego IP z fallbackiem, oraz swiadome zarzadzanie kaskada i breakpointami. Dzieki temu moge wyjasnic nie tylko "jak", ale tez "dlaczego" dane decyzje zostaly podjete.
