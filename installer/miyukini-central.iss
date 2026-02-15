; ============================================================================
; Miyukini Central — Inno Setup Script
; ============================================================================
; Prerequis : Inno Setup 6+ (https://jrsoftware.org/isinfo.php)
;
; Generer l'installateur :
;   1. cargo build --release -p miyukini-central-native -p kindmother-service
;   2. "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer\miyukini-central.iss
;
; Sortie : installer\output\MiyukiniCentral-0.1.0-Setup.exe
; ============================================================================

#define MyAppName      "Miyukini COG"
#define MyAppVersion   "0.1.0"
#define MyAppPublisher "Studio Miyukini"
#define MyAppURL       "https://miyukini.com"
#define MyAppExeName   "miyukini-central.exe"
#define PfShortcuts    "{commonpf32}\Miyukini-COG"

; Racine du depot (un niveau au-dessus de installer/)
#define ScriptDir      ExtractFilePath(SourcePath)
#define RepoRoot       AddBackslash(ExtractFilePath(RemoveBackslashUnlessRoot(ScriptDir)))
#define ReleaseDir     RepoRoot + "target\release"

[Setup]
AppId={{E8F3A1B7-4C2D-4F8E-9A6B-1D3E5F7A9C2B}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
; Tout l'app dans AppData (donnees inscriptibles)
DefaultDirName={localappdata}\Miyukini-COG
DefaultGroupName={#MyAppName}
; Decommenter la ligne suivante quand une icone sera disponible :
; SetupIconFile={#RepoRoot}assets\miyukini.ico
OutputDir={#RepoRoot}installer\output
OutputBaseFilename=MiyukiniCentral-{#MyAppVersion}-Setup
Compression=lzma2/ultra64
SolidCompression=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
; Admin requis pour creer le dossier et raccourcis dans Program Files
PrivilegesRequired=admin
WizardStyle=modern
DisableProgramGroupPage=yes
LicenseFile={#RepoRoot}LICENSE
; Decommenter si icone dispo :
; UninstallDisplayIcon={app}\{#MyAppExeName}

[Languages]
Name: "french"; MessagesFile: "compiler:Languages\French.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Creer un raccourci sur le Bureau"; GroupDescription: "Raccourcis :"; Flags: unchecked

[Files]
; --- Binaires principaux ---
Source: "{#ReleaseDir}\miyukini-central.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#ReleaseDir}\kindmother-server.exe"; DestDir: "{app}"; Flags: ignoreversion

; --- Voix Miou (optionnel — incluses si le dossier existe) ---
Source: "{#RepoRoot}voices\fr\*"; DestDir: "{app}\voices\fr"; Flags: ignoreversion recursesubdirs createallsubdirs skipifsourcedoesntexist

; --- Documentation (optionnel) ---
Source: "{#RepoRoot}docs\distribution\MODE_EMPLOI.md"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist
Source: "{#RepoRoot}LICENSE"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist
; --- Documents juridiques (licence, politique, licence pro service-tier) ---
Source: "{#RepoRoot}docs\legal\*"; DestDir: "{app}\docs\legal"; Flags: ignoreversion recursesubdirs createallsubdirs skipifsourcedoesntexist

[Dirs]
; Dossier dans Program Files pour raccourci + desinstalleur
Name: "{#PfShortcuts}"

[Icons]
; Program Files : raccourci lancement + desinstalleur (seuls elements dans PF)
Name: "{#PfShortcuts}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; WorkingDir: "{app}"
Name: "{#PfShortcuts}\Desinstaller {#MyAppName}"; Filename: "{uninstallexe}"
; Bureau (si coche par l'utilisateur)
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; WorkingDir: "{app}"; Tasks: desktopicon

[Run]
; Lancer l'application apres l'installation
Filename: "{app}\{#MyAppExeName}"; Description: "Lancer {#MyAppName}"; WorkingDir: "{app}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
; Supprimer le dossier Program Files (raccourcis)
Type: filesandordirs; Name: "{#PfShortcuts}"

[Code]
// Verifier si une instance est en cours avant la desinstallation.
// NOTE: L'app doit creer un mutex "MiyukiniCentralMutex" pour que cette verification soit effective.
function InitializeUninstall(): Boolean;
begin
  Result := True;
  if CheckForMutexes('MiyukiniCentralMutex') then
  begin
    if MsgBox('Miyukini Central est en cours d''execution.' + #13#10 +
              'Fermez l''application avant de desinstaller.' + #13#10#13#10 +
              'Reessayer ?', mbError, MB_RETRYCANCEL) = IDRETRY then
    begin
      Result := InitializeUninstall();
    end
    else
      Result := False;
  end;
end;
