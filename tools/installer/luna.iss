; Luna Download Manager Windows installer (Inno Setup 6). Built by tools/installer/build_luna.ps1
;
; Install for the current user (no administrator rights, %LOCALAPPDATA%\Programs) or for
; everyone (Program Files); the wizard asks. Tasks: browser integration (native messaging host
; for Chrome, Edge, Brave and Firefox), .torrent files and magnet links, start with Windows, a
; desktop shortcut.

#ifndef AppVersion
  #define AppVersion "6.0.0"
#endif
#define AppName "Luna Download Manager"
#define AppExe "lunadownloadmanager.exe"
#define HostExe "lunadownloadmanager-nmhost.exe"
#define Root "..\.."
#define Bin Root + "\target\release"

[Setup]
AppId={{7F8C4B12-9A3E-4B76-B819-C826BF39884E}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=Luna Download Manager Authors
AppPublisherURL=https://github.com/asmodeus-oos/LunaDownloadManager
AppSupportURL=https://github.com/asmodeus-oos/LunaDownloadManager/issues
AppUpdatesURL=https://github.com/asmodeus-oos/LunaDownloadManager/releases
AppCopyright=Copyright (C) 2026 Luna Download Manager Authors
DefaultDirName={autopf}\{#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
UsedUserAreasWarning=no
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
WizardStyle=modern
SetupIconFile={#Root}\crates\app\assets\icons\app\mdm.ico
UninstallDisplayIcon={app}\{#AppExe}
UninstallDisplayName={#AppName}
OutputDir={#Root}\target\dist
OutputBaseFilename=LunaDownloadManager-{#AppVersion}-setup
Compression=lzma2/ultra64
SolidCompression=yes
ChangesAssociations=yes
CloseApplications=yes
VersionInfoVersion={#AppVersion}
VersionInfoCompany=Luna Download Manager
VersionInfoDescription={#AppName} setup
VersionInfoProductName={#AppName}
#ifdef Sign
SignTool=mdmsign
SignedUninstaller=yes
  #define SignFlag "signonce"
#else
  #define SignFlag ""
#endif

#include "languages.iss"

[CustomMessages]
lang_en.TaskBrowser=Set up &browser integration (Chrome, Edge, Brave, Firefox)
lang_en.TaskTorrents=Open .&torrent files and magnet links with {#AppName}
lang_en.TaskStartup=Start {#AppName} when &Windows starts
lang_en.TaskGroupOther=Other:
lang_en.Launch=Start {#AppName}
lang_en.KeepData=Also delete your download list and settings?%n%nYour downloaded files are not touched either way.

[Tasks]
Name: "browser"; Description: "{cm:TaskBrowser}"
Name: "torrents"; Description: "{cm:TaskTorrents}"
Name: "startup"; Description: "{cm:TaskStartup}"
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:TaskGroupOther}"; Flags: unchecked

[Files]
Source: "{#Bin}\{#AppExe}"; DestDir: "{app}"; Flags: ignoreversion {#SignFlag}
Source: "{#Bin}\{#HostExe}"; DestDir: "{app}"; Flags: ignoreversion {#SignFlag}
Source: "{#Bin}\lunadownloadmanager-cli.exe"; DestDir: "{app}"; Flags: ignoreversion {#SignFlag}
Source: "{#Bin}\lunadownloadmanager-mcp.exe"; DestDir: "{app}"; Flags: ignoreversion {#SignFlag}
Source: "{#Root}\target\extension\chromium\*"; DestDir: "{app}\extension\chromium"; Flags: ignoreversion recursesubdirs
Source: "{#Root}\target\extension\luna-firefox.xpi"; DestDir: "{app}\extension"; Flags: ignoreversion
Source: "{#Root}\LICENSE"; DestDir: "{app}"; DestName: "LICENSE.txt"; Flags: ignoreversion
Source: "{#Root}\ATTRIBUTION.md"; DestDir: "{app}"; DestName: "ATTRIBUTION.txt"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExe}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Registry]
Root: HKCU; Subkey: "Software\LunaDownloadManager"; ValueType: string; ValueName: "InstallLanguage"; ValueData: "{language}"; Flags: uninsdeletevalue uninsdeletekeyifempty
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "LunaDownloadManager"; ValueData: """{app}\{#AppExe}"" /startup"; Flags: uninsdeletevalue; Tasks: startup
Root: HKA; Subkey: "Software\Classes\LunaDownloadManager.torrent"; ValueType: string; ValueName: ""; ValueData: "Torrent file"; Flags: uninsdeletekey; Tasks: torrents
Root: HKA; Subkey: "Software\Classes\LunaDownloadManager.torrent\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExe},0"; Tasks: torrents
Root: HKA; Subkey: "Software\Classes\LunaDownloadManager.torrent\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExe}"" ""%1"""; Tasks: torrents
Root: HKA; Subkey: "Software\Classes\.torrent"; ValueType: string; ValueName: ""; ValueData: "LunaDownloadManager.torrent"; Flags: uninsdeletekeyifempty; Tasks: torrents
Root: HKA; Subkey: "Software\Classes\.torrent\OpenWithProgids"; ValueType: string; ValueName: "LunaDownloadManager.torrent"; ValueData: ""; Flags: uninsdeletevalue uninsdeletekeyifempty; Tasks: torrents
Root: HKA; Subkey: "Software\Classes\magnet"; ValueType: string; ValueName: ""; ValueData: "URL:Magnet link"; Tasks: torrents
Root: HKA; Subkey: "Software\Classes\magnet"; ValueType: string; ValueName: "URL Protocol"; ValueData: ""; Tasks: torrents
Root: HKA; Subkey: "Software\Classes\magnet\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExe},0"; Tasks: torrents
Root: HKA; Subkey: "Software\Classes\magnet\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExe}"" ""%1"""; Tasks: torrents

[Run]
Filename: "{app}\{#HostExe}"; Parameters: "--register"; Flags: runhidden; Tasks: browser; Check: not IsAdminInstallMode
Filename: "{app}\{#HostExe}"; Parameters: "--register --all-users"; Flags: runhidden; Tasks: browser; Check: IsAdminInstallMode
Filename: "{app}\{#AppExe}"; Description: "{cm:Launch}"; Flags: nowait postinstall skipifsilent
Filename: "{app}\{#AppExe}"; Parameters: "/updated"; Flags: nowait runasoriginaluser; Check: Relaunch

[UninstallRun]
Filename: "{app}\{#HostExe}"; Parameters: "--unregister"; Flags: runhidden; RunOnceId: "UnregisterUser"; Check: not IsAdminInstallMode
Filename: "{app}\{#HostExe}"; Parameters: "--unregister --all-users"; Flags: runhidden; RunOnceId: "UnregisterAll"; Check: IsAdminInstallMode

[UninstallDelete]
Type: files; Name: "{app}\com.lunadownloadmanager.host*.json"

[Code]
const
  RunningMutex = 'LunaDownloadManager.Running';

function Relaunch: Boolean;
begin
  Result := ExpandConstant('{param:relaunch|0}') = '1';
end;

procedure ExitRunningApp(Exe: String);
var
  Code, Waited: Integer;
begin
  if CheckForMutexes(RunningMutex) and FileExists(Exe) then
  begin
    Exec(Exe, '/exit', '', SW_HIDE, ewWaitUntilTerminated, Code);
    Waited := 0;
    while CheckForMutexes(RunningMutex) and (Waited < 15000) do
    begin
      Sleep(250);
      Waited := Waited + 250;
    end;
  end;
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
begin
  ExitRunningApp(ExpandConstant('{app}\{#AppExe}'));
  Result := '';
end;

function InitializeUninstall(): Boolean;
begin
  ExitRunningApp(ExpandConstant('{app}\{#AppExe}'));
  Result := True;
end;

procedure ForgetIfOurs(Key, Ours: String);
var
  Value: String;
begin
  if RegQueryStringValue(HKA, Key + '\shell\open\command', '', Value) and (Pos(Lowercase(Ours), Lowercase(Value)) > 0) then
    RegDeleteKeyIncludingSubkeys(HKA, Key);
end;

var
  RemoveData: Boolean;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Value, Data: String;
  Code: Integer;
begin
  Data := ExpandConstant('{userappdata}\lunadownloadmanager');
  if CurUninstallStep = usUninstall then
  begin
    ForgetIfOurs('Software\Classes\magnet', ExpandConstant('{app}\{#AppExe}'));
    if RegQueryStringValue(HKA, 'Software\Classes\.torrent', '', Value) and (Value = 'LunaDownloadManager.torrent') then
      RegDeleteValue(HKA, 'Software\Classes\.torrent', '');
    RemoveData := False;
    if DirExists(Data) and not UninstallSilent() then
      RemoveData := MsgBox(CustomMessage('KeepData'), mbConfirmation, MB_YESNO or MB_DEFBUTTON2) = IDYES;
    if RemoveData then
      Exec(ExpandConstant('{app}\{#AppExe}'), '--forget-sign-ins', '', SW_HIDE, ewWaitUntilTerminated, Code);
  end;
  if (CurUninstallStep = usPostUninstall) and RemoveData then
    DelTree(Data, True, True, True);
end;
