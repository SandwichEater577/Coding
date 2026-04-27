<?php
$suma = 0;
$i = 1;
$limit = 5;

while ($i <= $limit) {
    $suma += $i;
    $i++;
}

$srednia = $suma / $limit;

echo "Suma liczb od 1 do 5 to: " . $suma . "<br>";
echo "Średnia liczb to: " . $srednia;
?>