<?php
function znajdz_max($array) {
    if (empty($array)) return null;

    $max = $array[0];
    foreach ($array as $liczba) {
        if ($liczba > $max) {
            $max = $liczba;
        }
    }
    return $max;
}

$liczby = [1, 5, 23, 12, 7];
echo "Największa liczba to: " . znajdz_max($liczby);
?>