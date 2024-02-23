# ������������� ������� ����� ��� �����, ��� ����� ������
Set-Location -Path $PSScriptRoot

# ��������� ����� ������ � ������� �����, ��������� ���������
$result = Get-ChildItem -Recurse -Filter *.rs | Get-Content | Measure-Object -Line

# ������� ���������� �����
Write-Host "����� ����� ���� � ������ .rs: $($result.Lines)"

# ��������� �����, ����� ���� �� ����������� �����
Write-Host "������� ����� �������, ����� �����..."
$null = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
