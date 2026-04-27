<?php
function prepare_sql($login, $password) {
    return "SELECT * FROM users WHERE login = '$login' AND password = '$password';";
}

echo prepare_sql("ala", "123456");
?>