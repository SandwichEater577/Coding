# DarkWire — Notatki do obrony projektu

Projekt: interaktywna gra terminalowa w HTML/CSS/JS  
Nauczyciel: Adrian Trąbka · TechniSchools Warszawa

---

## SPIS TREŚCI

1. Architektura projektu
2. JavaScript — każda funkcja po kolei
3. Trudne pytania które Trąbka może zadać
4. CSS — layout, animacje, zmienne
5. Checklista kryteriów

---

## 1. ARCHITEKTURA

```
index.html   — wszystkie screeny misji jako ukryte divy
style.css    — wygląd, animacje, CSS variables, layout
JS/script.js — logika gry, eventy, animacje JS
```

**Jak działa zmiana screenu:**
1. Wszystkie `.terminal-screen` mają klasę `screen-unactive` → `display: none`
2. `goToScreen()` dodaje `screen-unactive` na WSZYSTKICH
3. Potem zdejmuje ją tylko z wybranego → tylko ten jeden widoczny
4. `typewrite()` animuje tekst litera po literze

---

## 2. JAVASCRIPT — KAŻDA FUNKCJA

### Zmienne globalne na górze

```js
const terminalOutput = document.getElementById("terminalOutput");
const missionList    = document.getElementById("missionList");
// itd.
```

**Dlaczego na górze pliku:** pobieramy elementy DOM raz, zamiast za każdym razem szukać ich w funkcjach. Wydajniejsze i czytelniejsze.

**`getElementById` vs `querySelector`:**
- `getElementById("id")` — szybszy, szuka tylko po ID
- `querySelector("#id")` — wolniejszy ale bardziej elastyczny, przyjmuje każdy selektor CSS

---

### `wait(seconds)`

```js
const wait = (seconds) =>
  new Promise((resolve) => setTimeout(resolve, seconds * 1000));
```

**Co robi:** zwraca Promise który "wypełnia się" po X sekundach.  
**Dlaczego Promise:** żeby można było pisać `await wait(0.5)` zamiast zagnieżdżonych callbacków.  
**Arrow function:** `(seconds) => ...` to skrócony zapis funkcji, ekwiwalent `function wait(seconds) { ... }`.

**Trąbka może zapytać:** *"Co to jest Promise?"*  
→ Promise to obiekt reprezentujący przyszły wynik operacji asynchronicznej. Ma trzy stany: `pending` (oczekuje), `fulfilled` (zakończone sukcesem), `rejected` (błąd).

---

### `randomBetween(min, max)`

```js
const randomBetween = (min, max) => Math.random() * (max - min) + min;
```

**Co robi:** losuje liczbę z przedziału `[min, max)`.  
**Dlaczego nie samo `Math.random()`:** `Math.random()` daje zakres `[0, 1)` — mnożymy i przesuwamy żeby dostać dowolny zakres.  
**Użycie:** losowe opóźnienia w typewriterze, losowe efekty lagu ekranu.

---

### `updateClock()`

```js
function updateClock() {
  const now = new Date();
  const pad = (n) => String(n).padStart(2, "0");
  dateSpan.textContent = `${now.getFullYear()}.${pad(now.getMonth() + 1)}.${pad(now.getDate())}`;
  timeSpan.textContent = `${pad(now.getHours())}:${pad(now.getMinutes())}:${pad(now.getSeconds())}`;
}
```

**`new Date()`:** tworzy obiekt z aktualną datą i czasem.  
**`getMonth() + 1`:** miesiące w JS są indeksowane od 0 (styczeń = 0, grudzień = 11) — dodajemy 1.  
**`pad`:** wewnętrzna arrow function — dodaje zero na początku jeśli liczba jednocyfrowa (`9` → `"09"`).  
**`padStart(2, "0")`:** uzupełnia string do 2 znaków od lewej zerem.  
**Gdzie wywoływana:** `setInterval(updateClock, 1000)` — co sekundę, plus raz na starcie.

---

### `fetchIp()` — async/await + fetch

```js
async function fetchIp() {
  try {
    const res  = await fetch("https://api.ipify.org?format=json");
    const data = await res.json();
    ipSpan.textContent = data.ip;
    ipSpan.style.color = "var(--info)";
  } catch {
    const fake = Array.from({ length: 4 }, () =>
      Math.floor(randomBetween(1, 255))
    ).join(".");
    ipSpan.textContent = fake;
  }
}
```

**`async`:** oznacza że funkcja jest asynchroniczna — zwraca Promise automatycznie.  
**`await fetch()`:** czeka na odpowiedź z serwera zanim przejdzie dalej.  
**`await res.json()`:** czeka na sparsowanie JSON z odpowiedzi.  
**`try/catch`:** jeśli fetch się nie uda (brak internetu) — generuje losowe fake IP zamiast crashować.  
**`Array.from({ length: 4 }, () => ...)`:** tworzy tablicę 4 losowych liczb.  
**`.join(".")`:** łączy tablicę w string: `[83, 175, 12, 1]` → `"83.175.12.1"`.

---

### `updateTrace(level)`

```js
function updateTrace(level) {
  traceStatus.textContent = level === 0 ? "INACTIVE" : `ACTIVE ${level}%`;
  if      (level === 0) traceStatus.style.color = "var(--text-soft)";
  else if (level < 40)  traceStatus.style.color = "var(--text)";
  else if (level < 70)  traceStatus.style.color = "var(--warning)";
  else                  traceStatus.style.color = "var(--error)";
}
```

**Operator trójkowy:** `condition ? valueIfTrue : valueIfFalse` — skrót dla prostego if/else.  
**`element.style.color`:** bezpośrednia zmiana stylu przez JS (inline style).  
**`var(--warning)`:** odwołanie do zmiennej CSS zdefiniowanej w `:root`.

---

### `startScreenLag(lagLevel)`

```js
function startScreenLag(lagLevel) {
  clearInterval(lagInterval);
  if (lagLevel <= 0) return;

  const baseInterval = Math.max(800, 40000 * (1 - lagLevel / 100));

  lagInterval = setInterval(async () => {
    const type = Math.random();

    if (type < 0.4) {
      terminal.classList.add("glitch-burst");
      await wait(0.05 + randomBetween(0, 0.08));
      terminal.classList.remove("glitch-burst");
    } else if (type < 0.7) {
      const shift = Math.floor(randomBetween(1, lagLevel / 8 + 2));
      terminal.style.transform = `translateX(${shift}px)`;
      await wait(0.04);
      terminal.style.transform = `translateX(-${shift}px)`;
      await wait(0.03);
      terminal.style.transform = "";
    } else {
      terminal.style.filter = `brightness(${1.3 + lagLevel / 100}) contrast(1.2)`;
      await wait(0.06);
      terminal.style.filter = "";
    }
  }, baseInterval * (0.7 + randomBetween(0, 0.6)));
}
```

**`clearInterval` na początku:** zatrzymuje poprzedni lag zanim uruchomi nowy.  
**`Math.max(800, ...)`:** gwarantuje że interval nie będzie mniejszy niż 800ms.  
**`async` callback w `setInterval`:** możliwe — callback może być async, wtedy można `await` w środku.  
**`classList.add/remove`:** dodaje/usuwa klasę CSS — tu odpala animację `glitch-burst`.  
**`element.style.transform`:** przesuwa element przez CSS transform — efekt "trzęsienia".  
**`element.style.filter`:** zmienia efekty wizualne — `brightness` i `contrast`.

---

### `setActiveMission(screenId)`

```js
function setActiveMission(screenId) {
  document.querySelectorAll(".mission").forEach((b) => {
    b.classList.remove("mission--active");
    b.classList.add("mission--locked");
  });
  const entry = Object.entries(MISSION_MAP).find(
    ([_, v]) => v.screen === screenId
  );
  if (!entry) return;
  const btn = document.getElementById(entry[0]);
  if (!btn) return;
  btn.classList.remove("mission--locked");
  btn.classList.add("mission--active");
}
```

**`querySelectorAll(".mission")`:** zwraca NodeList wszystkich elementów z klasą `.mission`.  
**`.forEach()`:** iteruje po każdym elemencie NodeList.  
**`Object.entries(MISSION_MAP)`:** zwraca tablicę par `[klucz, wartość]` z obiektu.  
**`.find()`:** zwraca pierwszy element spełniający warunek.  
**Destrukturyzacja `[_, v]`:** `_` to klucz (ignorowany, konwencja), `v` to wartość.

---

### `showOverlayHint(keyMap)` — coraz szybszy blink

```js
function showOverlayHint(keyMap) {
  const speeds = [600, 600, 400, 250, 150, 80, 40];
  let second   = 0;
  let blinkId  = null;

  function runBlink(interval) {
    clearInterval(blinkId);
    blinkId = setInterval(() => {
      overlay.style.opacity = overlay.style.opacity === "0" ? "1" : "0";
    }, interval);
  }

  runBlink(speeds[0]);

  const speedUp = setInterval(() => {
    second++;
    if (second < speeds.length) runBlink(speeds[second]);
  }, 1000);
  ...
}
```

**Dwa `setInterval` naraz:** jeden na blink, drugi na przyspieszanie co sekundę.  
**Closure:** `runBlink` ma dostęp do `blinkId` z zewnętrznego scope — to jest closure.  
**`clearInterval` przed nowym:** żeby poprzedni interval nie działał równolegle.  
**`overlay.style.opacity`:** przełącza między `"0"` i `"1"` — efekt migania.

---

### `typewriterScreen(screenId)` — główna animacja

```js
async function typewriterScreen(screenId) {
  const screen    = document.getElementById(screenId);
  const lines     = Array.from(screen.querySelectorAll("p"));
  const originals = lines.map((p) => p.innerHTML);
  lines.forEach((p) => (p.innerHTML = ""));

  for (let i = 0; i < lines.length; i++) {
    const p    = lines[i];
    const full = originals[i];

    if (full.trim() === "") { await wait(0.15); continue; }

    const delay = Math.max(12, Math.min(35, 800 / full.length));

    for (let c = 0; c < full.length; c++) {
      p.innerHTML = full.slice(0, c + 1);
      await wait(delay / 1000);
    }

    await wait(randomBetween(0.02, 0.08));
    rootDivTerminal.scrollTop = rootDivTerminal.scrollHeight;
  }
}
```

**`Array.from()`:** konwertuje NodeList na prawdziwą tablicę (NodeList nie ma `.map()`).  
**`.map(p => p.innerHTML)`:** tworzy tablicę tekstów — zapisujemy zanim wyczyścimy.  
**`Math.max(12, Math.min(35, 800 / full.length))`:** clamp — im dłuższa linia, tym szybsze pisanie (maks 35ms, min 12ms na znak).  
**`full.slice(0, c + 1)`:** zwraca podstring od 0 do znaku c — efekt dopisywania litery.  
**`scrollTop = scrollHeight`:** auto-scroll na dół po każdej linii.  
**`await wait()`:** zatrzymuje pętlę — bez tego wszystko pojawiłoby się natychmiast.

---

### `showScreen(screenId, traceLevel)`

```js
async function showScreen(screenId, traceLevel = 0) {
  if (isAnimating) return;
  isAnimating = true;

  triggerGlitch();
  await wait(0.15);

  document.querySelectorAll(".terminal-screen").forEach((s) => {
    s.classList.add("screen-unactive");
  });

  document.getElementById(screenId).classList.remove("screen-unactive");
  ...
  await typewriterScreen(screenId);
  isAnimating = false;

  const keys = SCREEN_KEYS[screenId];
  if (keys) setScreenKeys(keys);
}
```

**`traceLevel = 0`:** domyślna wartość parametru — jeśli nie podasz, używa 0.  
**`if (isAnimating) return`:** guard clause — blokuje podwójne wywołanie podczas animacji.  
**Schemat ukrywania/pokazywania:** dodaj `screen-unactive` wszędzie → zdejmij z wybranego.  
**`SCREEN_KEYS[screenId]`:** szuka w obiekcie czy dla tego screenu są zdefiniowane klawisze.

---

### Event listeners

```js
// event delegation — jeden listener zamiast 7
missionList.addEventListener("click", (e) => {
  const btn = e.target.closest(".mission");
  if (!btn || isAnimating) return;
  const config = MISSION_MAP[btn.id];
  if (!config) return;
  showScreen(config.screen, config.trace);
});
```

**Event delegation:** jeden listener na rodzicu zamiast osobnych na każdym buttonie. Efektywniejsze.  
**`e.target`:** element który faktycznie został kliknięty — może być `<span>` wewnątrz buttona.  
**`.closest(".mission")`:** idzie w górę drzewa DOM szukając przodka z klasą `.mission`.

```js
document.addEventListener("keydown", (e) => {
  if (isAnimating) return;
  if (activeKeys[e.key]) {
    e.preventDefault();
    activeKeys[e.key]();
  }
});
```

**`e.key`:** string z nazwą wciśniętego klawisza (`"Enter"`, `"s"`, `"1"` itd.).  
**`activeKeys[e.key]`:** sprawdza czy obiekt ma właściwość z tym kluczem — dynamiczne komendy.  
**`e.preventDefault()`:** blokuje domyślne zachowanie przeglądarki.  
**`activeKeys[e.key]()`:** wywołuje funkcję przypisaną do klawisza.

```js
rootDivTerminal.addEventListener("wheel", (e) => {
  e.preventDefault();
  const dir = e.deltaY > 0 ? 1 : -1;
  rootDivTerminal.scrollBy({ top: 23 * dir, behavior: "instant" });
});
```

**`e.deltaY`:** wartość scrolla — dodatnia = w dół, ujemna = w górę.  
**`scrollBy`:** przewija o podaną liczbę pikseli.  
**`behavior: "instant"`:** bez animacji — efekt krokowego scrollu jak w starym terminalu.

```js
document.addEventListener("keydown", (e) => {
  ...
}, { once: true });
```

**`{ once: true }`:** listener odpala się tylko raz, potem jest automatycznie usuwany.

---

## 3. TRUDNE PYTANIA

### P: Co to jest closure?

Closure to funkcja która "zapamiętuje" zmienne z zewnętrznego scope'u nawet po jego zakończeniu.

W projekcie — `showOverlayHint()`:
```js
function showOverlayHint(keyMap) {
  let blinkId = null;  // zmienna w zewnętrznym scope

  function runBlink(ms) {
    clearInterval(blinkId);  // runBlink widzi blinkId — to jest closure
    blinkId = setInterval(...);
  }

  runBlink(speeds[0]);  // wywołujemy runBlink
  // blinkId nadal istnieje i jest dostępny przez runBlink
}
```

---

### P: Czym różni się `let` od `const` od `var`?

| | `var` | `let` | `const` |
|---|---|---|---|
| Zasięg | cała funkcja | blok `{}` | blok `{}` |
| Można przypisać ponownie | ✅ | ✅ | ❌ |
| Hoisting | ✅ (undefined) | ❌ (TDZ error) | ❌ (TDZ error) |

W projekcie: `const` dla stałych referencji do DOM, `let` dla zmiennego stanu (`isAnimating`, `activeKeys`).

---

### P: Co to jest `async/await`?

`async` — funkcja zwraca Promise.  
`await` — zatrzymuje wykonanie do rozwiązania Promise, ale tylko wewnątrz `async`.

```js
// bez async/await (callback hell)
fetch(url).then(res => res.json()).then(data => console.log(data));

// z async/await (czytelne)
async function getData() {
  const res  = await fetch(url);
  const data = await res.json();
  console.log(data);
}
```

---

### P: Czym jest event delegation?

Zamiast listenera na każdy button, jeden listener na rodzicu + `e.target`:

```js
// BEZ — 7 listenerów
mission1.addEventListener("click", fn);
mission2.addEventListener("click", fn);
// x7...

// Z event delegation — 1 listener
missionList.addEventListener("click", (e) => {
  const btn = e.target.closest(".mission");
});
```

Zalety: mniej pamięci, działa dla dynamicznie dodanych elementów.

---

### P: Czym różni się `querySelector` od `querySelectorAll`?

```js
document.querySelector(".mission")     // pierwszy pasujący element (lub null)
document.querySelectorAll(".mission")  // NodeList WSZYSTKICH pasujących elementów
```

NodeList nie jest tablicą. Żeby użyć `.map()`:
```js
Array.from(document.querySelectorAll("p")).map(p => p.innerHTML)
```

---

### P: Jak działa `classList`?

```js
el.classList.add("active")       // dodaje klasę
el.classList.remove("active")    // usuwa klasę
el.classList.toggle("active")    // dodaje/usuwa naprzemiennie
el.classList.contains("active")  // true/false — czy ma klasę
```

---

### P: Kiedy `element.style`, a kiedy klasa CSS?

**Klasa CSS** → zmiana "stanowa" (aktywny/nieaktywny, widoczny/ukryty):
```js
terminal.classList.add("glitch-burst");
```

**`element.style`** → wartość obliczana dynamicznie w JS:
```js
traceStatus.style.color    = "var(--error)";    // zależy od poziomu
terminal.style.transform   = `translateX(${shift}px)`;  // losowa wartość
overlay.style.opacity      = "0";               // animacja fade
```

---

### P: `setTimeout` vs `setInterval`?

```js
setTimeout(fn, 1000)   // odpala fn JEDEN RAZ po 1 sekundzie
setInterval(fn, 1000)  // odpala fn CO 1 sekundę w nieskończoność

clearTimeout(id)       // anuluje setTimeout
clearInterval(id)      // zatrzymuje setInterval
```

W projekcie:
- `setInterval(updateClock, 1000)` — zegar
- `setTimeout(() => triggerGameOver(), 7000)` — game over timer
- `setInterval` w lag systemie — losowe glitche

---

### P: Co robi `{ once: true }` w addEventListener?

```js
document.addEventListener("keydown", handler, { once: true });
// handler odpala się TYLKO RAZ, potem jest automatycznie usunięty
```

Bez tego musisz ręcznie: `document.removeEventListener("keydown", handler)`.

---

### P: Co to jest `Math.random` i jak go używasz?

```js
Math.random()                    // 0.0 - 0.999...
Math.random() * 100              // 0.0 - 99.9...
Math.floor(Math.random() * 100)  // 0 - 99 (całkowite)

// w projekcie:
const randomBetween = (min, max) => Math.random() * (max - min) + min;
randomBetween(1, 255) // losowa liczba między 1 a 255
```

---

### P: `new Date()` — jakich metod używasz?

```js
const now = new Date();
now.getFullYear()   // 2026
now.getMonth()      // 0-11 (!!styczeń = 0!!)
now.getDate()       // 1-31 (dzień miesiąca)
now.getHours()      // 0-23
now.getMinutes()    // 0-59
now.getSeconds()    // 0-59
```

---

## 4. CSS

### P: Jak działa `display: grid` w projekcie?

Trzy miejsca gdzie grid jest kluczowy:

```css
/* 1. home-screen — header + main */
div#home-screen {
  display: grid;
  grid-template-rows: auto 1fr;
  /* auto = header dopasowuje się do zawartości */
  /* 1fr = main zajmuje całą resztę wysokości */
  height: 100vh;
}

/* 2. main-content — sidebar + terminal */
.main-content {
  display: grid;
  grid-template-columns: 260px 1fr;
  /* sidebar: stałe 260px */
  /* terminal: reszta ekranu */
}

/* 3. terminal — output + input + shortcuts */
#terminal {
  display: grid;
  grid-template-rows: 1fr auto auto;
  /* output: reszta wysokości (scrollowalny) */
  /* input i shortcuts: tyle ile potrzeba */
}
```

---

### P: Grid vs Flex — kiedy co?

| | Flex | Grid |
|---|---|---|
| Wymiary | 1D — wiersz LUB kolumna | 2D — wiersze I kolumny |
| Najlepszy do | elementów w linii | całego layoutu strony |

W projekcie:
- **Grid** — główny layout (header/main, sidebar/terminal, terminal rows)
- **Flex** — elementy w linii: statsy w headerze, shortcuty, misje w sidebarze

---

### P: Co to są CSS custom properties (zmienne)?

```css
:root {
  --bg: #0a0f0a;
  --text: #00ff88;
  --error: #ff4444;
  --warning: #ffcc00;
}

/* użycie: */
color: var(--text);
background: var(--bg);
```

Zalety: zmiana jednej wartości = zmiana wszędzie. Łatwe motywy. Czytelniejszy kod.  
W JS też można: `traceStatus.style.color = "var(--error)"`.

---

### P: Jak działają animacje CSS — `@keyframes`?

```css
/* 1. definiujesz keyframes */
@keyframes logo-glitch {
  0%, 100% { transform: translate(0, 0); text-shadow: none; }
  72.4%    { transform: translate(-1px, 0); text-shadow: 1px 0 #00ffff; }
  73.1%    { transform: translate(1px, 0);  text-shadow: -1px 0 #ff00ff; }
}

/* 2. przypisujesz do elementu */
.logo-text {
  animation: logo-glitch 4.8s infinite linear;
  /*         nazwa       czas  pow.  timing */
}
```

Timing functions: `linear` (stała prędkość), `ease` (wolno-szybko-wolno), `steps(2)` (skokowe).

---

### P: `transition` vs `animation`?

```css
/* transition — płynne przejście gdy zmienia się właściwość */
.mission {
  transition: background-color 0.18s ease, color 0.18s ease;
}
/* gdy JS zmieni kolor, przejście trwa 0.18s */

/* animation — działa samodzielnie, nie potrzebuje zmiany stanu */
.logo-text {
  animation: logo-glitch 4.8s infinite;
}
```

`transition` → reaguje na zmianę (hover, classList change)  
`animation` → działa według keyframes, niezależnie

---

### P: Co robi `filter` i `transform`?

```css
/* transform — geometryczne */
transform: translateX(3px)   /* przesuwa poziomo */
transform: translateY(-5px)  /* przesuwa pionowo */
transform: scale(1.1)        /* powiększa */

/* filter — wizualne */
filter: blur(2px)            /* rozmycie */
filter: brightness(1.3)      /* jaśniejszy */
filter: contrast(1.2)        /* wyższy kontrast */
filter: hue-rotate(90deg)    /* zmiana barwy */
```

W projekcie: `transform: translateX()` w efekcie lagu, `filter: brightness/contrast` w glitch.

---

### P: `overflow` — jak działa?

```css
div#home-screen {
  overflow: hidden;    /* nic nie wyjeżdża poza home-screen */
}

#terminalOutput {
  overflow-y: auto;    /* scrollbar pionowy gdy za dużo tekstu */
  overflow-x: hidden;  /* brak poziomego scrolla */
}
```

Bez `overflow: hidden` na `#home-screen` terminal rozciągałby stronę przy długich tekstach.

---

### P: `position: fixed` i `inset`?

```css
#press-enter-overlay {
  position: fixed;  /* wychodzi z normalnego układu, relatywny do okna */
  inset: 0;         /* skrót dla top:0; right:0; bottom:0; left:0 */
}
```

`position: fixed` = element zawsze w tym samym miejscu, niezależnie od scrolla.  
`inset: 0` = rozciąga element na cały ekran.

---

## 5. CHECKLISTA KRYTERIÓW

| Kryterium | Gdzie |
|---|---|
| `setInterval` | `setInterval(updateClock, 1000)`, lag system, overlay blink |
| `setTimeout` | `wait()` używa `setTimeout`, game over timer, overlay hide |
| `querySelector` | stałe na początku pliku: `querySelector("#traceStatus")` itd. |
| `querySelectorAll` | `querySelectorAll(".terminal-screen")` i `".mission"` |
| Listener `click` | `missionList.addEventListener("click", ...)` |
| Listener `keydown` | `document.addEventListener("keydown", ...)` x2 |
| Listener `scroll/resize` | `addEventListener("wheel", ...)`, `addEventListener("resize", ...)` |
| `event.preventDefault()` | w wheel i keydown |
| `filter/transform` CSS | `transform: translateX()` w lagu, `filter: brightness`, `@keyframes logo-glitch` |
| `event.target` | `e.target.closest(".mission")` w click listenerze |
| `Math.random` | `randomBetween()` — typewriter, lag, fake IP |
| `element.style` | `traceStatus.style.color`, `terminal.style.transform`, `overlay.style.opacity` |
| `element.classList` | `add/remove` — glitch, setActiveMission, showScreen |
| `Date` | `new Date()` w `updateClock()` |
| `flex` | header stats, sidebar, input bar, shortcuty |
| Animacje | `logo-glitch`, `terminal-flicker`, `ending-fade-in`, `go-glitch` |
| Efekt wizualny | typewriter, lag glitch, ending overlay, game over |
| Jakość JS | CAPS_SNAKE dla stałych, komentarze sekcji, krótkie funkcje z jednym zadaniem |

**Poza zakresem (+2 pkt):**
- `async/await` z `Promise` i `fetch()`
- event delegation (`e.target.closest()`)
- `{ once: true }` w addEventListener
- `Array.from()` + `.map()`, `.find()`, `.forEach()`
- CSS `backdrop-filter`, `inset`, custom properties, `clamp()`
