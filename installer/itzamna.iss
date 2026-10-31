; Itzamna — Mayan Git Studio
; Consecration & Installation Ceremony (Inno Setup 6)

#define MyAppName "Itzamna"
#define MyAppFullName "Itzamná — Mesoamerican Git Studio"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "indoctrinatedrecluse"
#define MyAppURL "https://github.com/indoctrinatedrecluse/Itzamna"
#define MyAppExeName "itzamna-desktop.exe"

[Setup]
AppId={{C7F32D56-8A31-4F4E-91F2-98436B151F9B}
AppName={#MyAppFullName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputDir=..\dist
OutputBaseFilename=ItzamnaSetup-v{#MyAppVersion}
SetupIconFile=..\itzamna-desktop\assets\icon.ico
WizardImageFile=wizard_large.bmp
WizardSmallImageFile=wizard_small.bmp
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=lowest
DisableDirPage=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Messages]
SetupWindowTitle=🔱 Consecration of {#MyAppFullName} (v{#MyAppVersion})
WelcomeLabel1=🔱 ITZAMNÁ — CONSECRATION OF THE GIT SANCTUARY
WelcomeLabel2=Welcome to the sacred rites of installation.%n%nYou are about to awaken Itzamná, the GPU-accelerated Mesoamerican Git Studio. The celestial loom weaves branches, stelae, and prophetic reflogs into sovereign order.%n%nClick 'Consecrate' to continue, or 'Abandon' to exit the ritual.
SelectDirLabel3=Select the sacred enclave where Itzamná's stone stelae and codices shall be enshrined:
SelectDirBrowseLabel=To consecrate in a different sanctuary directory, click Browse:
SelectTasksLabel2=Select which celestial glyphs and blessings you wish to bestow upon your system:
ReadyLabel2a=Click Awaken to commence the consecration rites on this machine.
InstallingLabel=Carving the Stelae & Awaking the Cosmic Loom...
FinishedHeadingLabel=The Sanctuary is Consecrated!
FinishedLabel=The celestial portals are aligned. Itzamná is ready to govern your Git repositories with GPU-accelerated elegance.%n%nMay your commits remain immutable and your reflogs eternal.
ClickNext=Click Consecrate to proceed.
ButtonNext=&Consecrate >
ButtonInstall=&Awaken

[Tasks]
Name: "desktopicon"; Description: "Inscribe sacred desktop glyph (Create Desktop Shortcut)"; GroupDescription: "Celestial Blessings:"; Flags: checkedonce

[Files]
; Live video player assets extracted to temp for real-time video playback during install
Source: "video_player.ps1"; DestDir: "{tmp}"; Flags: dontcopy
Source: "..\itzamna-desktop\assets\background.mp4"; DestDir: "{tmp}"; Flags: dontcopy

; Installed Application Payload
Source: "..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\itzamna-desktop\assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\ixchel.jar"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist

[Icons]
Name: "{group}\{#MyAppFullName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\assets\icon.ico"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppFullName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\assets\icon.ico"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "☀️ Enter the Sanctuary (Launch Itzamná Git Studio)"; Flags: nowait postinstall skipifsilent

[Code]
function GetCurrentProcessId: DWORD;
external 'GetCurrentProcessId@kernel32.dll stdcall';

procedure InitializeWizard;
var
  ResultCode: Integer;
  TmpDir: String;
  VideoPlayerScript: String;
  VideoAsset: String;
  Params: String;
begin
  // Extract real-time companion video player and background video loop
  ExtractTemporaryFile('video_player.ps1');
  ExtractTemporaryFile('background.mp4');

  TmpDir := ExpandConstant('{tmp}');
  VideoPlayerScript := TmpDir + '\video_player.ps1';
  VideoAsset := TmpDir + '\background.mp4';

  // Launch video companion window tracking our installer PID
  Params := Format('-ExecutionPolicy Bypass -WindowStyle Hidden -File "%s" -VideoPath "%s" -ParentPid %d', [
    VideoPlayerScript,
    VideoAsset,
    GetCurrentProcessId()
  ]);

  Exec('powershell.exe', Params, '', SW_HIDE, ewNoWait, ResultCode);
end;
