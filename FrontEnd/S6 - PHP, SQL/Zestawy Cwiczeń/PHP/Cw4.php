<?php
function powtorzTekst($liczba, $tekst) {
    for ($i = 0; $i < $liczba; $i++) {
        echo $tekst . "<br>";
    }
}

powtorzTekst(5, "Przykładowy tekst");
?>