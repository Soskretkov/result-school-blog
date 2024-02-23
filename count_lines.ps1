# Устанавливаем текущую папку как папку, где лежит скрипт
Set-Location -Path $PSScriptRoot

# Выполняем поиск файлов и подсчёт строк, сохраняем результат
$result = Get-ChildItem -Recurse -Filter *.rs | Get-Content | Measure-Object -Line

# Выводим количество строк
Write-Host "Всего строк кода в файлах .rs: $($result.Lines)"

# Добавляем паузу, чтобы окно не закрывалось сразу
Write-Host "Нажмите любую клавишу, чтобы выйти..."
$null = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
