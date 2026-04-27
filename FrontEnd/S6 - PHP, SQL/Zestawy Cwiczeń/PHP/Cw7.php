<?php
function reverse($array) {
    $nowa_tablica = [];
    $dlugosc = count($array);
    
    for ($i = $dlugosc - 1; $i >= 0; $i--) {
        $nowa_tablica[] = $array[$i];
    }
    
    return $nowa_tablica;
}

$slowa = ["kot", "pies", "chomik"];
$odwrocone = reverse($slowa);

print_r($odwrocone);
?>