# JavaScript Events - Notatki

## Typy zdarzeń

### Mouse Events

- **click** - kliknięcie myszą
- **mouseover** - najechanie myszą na element
- **mouseout** - opuszczenie elementu myszą
- **mousedown** - przytrzymanie przycisku myszy
- **mouseup** - zwolnienie przycisku myszy
- **mousemove** - poruszanie myszą

### Keyboard Events

- **keydown** - wciśnięcie klawisza
- **keyup** - zwolnienie klawisza
- **keypress** - wciśnięcie klawisza (przestarzałe)

### Form Events

- **submit** - wysłanie formularza
- **change** - zmiana wartości
- **focus** - fokus na element
- **blur** - utrata fokusu

### Window Events

- **load** - załadowanie strony
- **scroll** - przewijanie
- **resize** - zmiana rozmiaru okna

## Sposoby przypisania zdarzeń

### 1. Atrybut HTML

```html
<button onclick="alert('Klik!')">Kliknij</button>
```

### 2. Właściwość DOM

```javascript
element.onclick = function () {
  console.log("Kliknięto!");
};
```

### 3. addEventListener (najlepszy sposób)

```javascript
element.addEventListener("click", function () {
  console.log("Kliknięto!");
});
```

## Obiekt Event

```javascript
element.addEventListener("click", (event) => {
  console.log(event.target); // element, na którym nastąpiło zdarzenie
  console.log(event.type); // typ zdarzenia
  console.log(event.preventDefault()); // zapobieganie domyślnemu zachowaniu
});
```
