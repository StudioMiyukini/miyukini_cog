; ============================================================================
; Miyukini Central — Inno Setup Script
; ============================================================================
; Prerequis : Inno Setup 6+ (https://jrsoftware.org/isinfo.php)
;
; Generer l'installateur :
;   1. cargo build --release -p miyukini-central-native -p kindmother-service
;   2. "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer\miyukini-central.iss
;
; Sortie : installer\output\MiyukiniCentral-0.2.0-Setup.exe
; ============================================================================

#define MyAppName      "Miyukini Central"
#define MyAppVersion   "0.2.0"
#define MyAppPublisher "Studio Miyukini"
#define MyAppURL       "https://miyukini.com"
#define MyAppExeName   "miyukini-central.exe"

; Racine du depot (un niveau au-dessus de installer/)
#define RepoRoot  AddBackslash(ExtractFilePath(RemoveBackslashUnlessRoot(ExtractFilePath(SourcePath))))
#define ReleaseDir RepoRoot + "target\release"

[Setup]
AppId={{E8F3A1B7-4C2D-4F8E-9A6B-1D3E5F7A9C2B}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
DefaultDirName={localappdata}\Miyukini-COG
DefaultGroupName={#MyAppName}
; Decommenter quand une icone .ico sera disponible :
; SetupIconFile={#RepoRoot}assets\miyukini.ico
OutputDir={#RepoRoot}installer\output
OutputBaseFilename=MiyukiniCentral-{#MyAppVersion}-Setup
Compression=lzma2/ultra64
SolidCompression=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=lowest
WizardStyle=modern
DisableProgramGroupPage=yes
LicenseFile={#RepoRoot}LICENSE
; Decommenter quand une icone .ico sera disponible :
; UninstallDisplayIcon={app}\{#MyAppExeName}

[Languages]
Name: "french";  MessagesFile: "compiler:Languages\French.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Creer un raccourci sur le Bureau"; GroupDescription: "Raccourcis :"; Flags: unchecked

[Files]
; --- Binaires ---
Source: "{#ReleaseDir}\miyukini-central.exe";  DestDir: "{app}"; Flags: ignoreversion
Source: "{#ReleaseDir}\kindmother-server.exe";  DestDir: "{app}"; Flags: ignoreversion

; --- Voix Miou ---
Source: "{#RepoRoot}voices\fr\*"; DestDir: "{app}\voices\fr"; Flags: ignoreversion recursesubdirs createallsubdirs

; --- Documentation (optionnel) ---
Source: "{#RepoRoot}docs\distribution\MODE_EMPLOI.md"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist
Source: "{#RepoRoot}LICENSE"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist

; --- Documents juridiques ---
Source: "{#RepoRoot}docs\legal\*"; DestDir: "{app}\docs\legal"; Flags: ignoreversion recursesubdirs createallsubdirs skipifsourcedoesntexist

[Icons]
; Menu Demarrer
Name: "{userprograms}\{#MyAppName}";                    Filename: "{app}\{#MyAppExeName}"; WorkingDir: "{app}"
Name: "{userprograms}\Desinstaller {#MyAppName}";       Filename: "{uninstallexe}"
; Bureau (si coche par l'utilisateur)
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; WorkingDir: "{app}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Lancer {#MyAppName}"; WorkingDir: "{app}"; Flags: nowait postinstall skipifsilent

[Code]
// Verifier si une instance tourne avant desinstallation.
// L'app doit creer un mutex "MiyukiniCentralMutex" pour que cela fonctionne.
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
