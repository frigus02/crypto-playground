$crypto_binary = "$PSScriptRoot/../target/debug/crypto-playground.exe"
$strings = Get-Content "$PSScriptRoot/single_char_xor_strings.txt"

foreach ($line in $strings) {
    $result = & $crypto_binary detect-single-char-xor $line 2>&1
    if ($?) {
        $result
    }
}
