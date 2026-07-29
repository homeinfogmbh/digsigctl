# digsigctl
Stand: 2026-07-29, geprüft gegen Commit af87a7e

## Zweck
Steuer-Agent für Digital-Signage-Systeme („DDB OS"). Läuft auf dem einzelnen
Signage-Gerät und stellt einen kleinen HTTP-RPC-Server bereit, über den die
zentrale Homeinfo-Infrastruktur das Gerät konfigurieren (Anzeige-URL setzen),
Fernkommandos ausführen (Beep, Reboot, Browser-Neustart, Betriebsmodus) und
Systeminformationen abfragen kann. Nachfolger von `digsigclt` (Python), auf die
neue TYPO3-CMS-Infrastruktur ausgerichtet.

## Stack & Einstiegspunkte
Rust (Edition 2021), Web-Framework Rocket 0.5, async über Tokio, HTTP-Client
`reqwest`. Zwei Binaries in `src/bin/`:
- `digsigctl.rs` — der RPC-Server (Haupt-Binary, Rocket-`#[launch]`).
- `fix-chromium-preferences.rs` — Hilfsprogramm für die Chromium-Preferences.

Kernlogik als Library in `src/lib.rs`; wichtige Module: `portal` (Portal-API +
Startup-Verifikation), `rpc` (Kommandos), `config` + `config/chromium_preferences`
(Chromium-Preferences schreiben), `sysinfo/*` (Systemdaten sammeln), `systemctl`
(systemd-Steuerung), `net` (VPN-IP-Discovery), `pacman` (Paketversionen),
`screenshot`, `sudo`.

## Schnittstellen
### Konsumiert
- **HTTP-API Homeinfo:** `GET https://termgr.homeinfo.de/administer/get-url/?hostname=<host>`
  liefert die anzuzeigende Portal-URL (JSON `{ "url": ... }`). ⚠️ ANNAHME: Der
  Modul-Docstring nennt zusätzlich `https://portal.homeinfo.de/test.html`; im
  Code wird tatsächlich nur `termgr.homeinfo.de` aufgerufen — Docstring wirkt veraltet.
- **systemd** über `systemctl` (via `sudo`, NOPASSWD): steuert `chromium.service`,
  `installation-instructions.service`, `unconfigured-warning.service`,
  `screenshot.service`; prüft `hidslcfg.service` (Konfigurationsmodus).
- **Hostname:** Linux `/etc/hostname` (Windows: `hostname`-Command).
- **Chromium-Preferences-Datei:** `~/.config/chromium/Default/Preferences`
  (für `digsig`-User: `/home/digsig/.config/chromium/Default/Preferences`);
  Fallback-Vorlage `/usr/share/digsigctl/Preferences`.
- **System-/Hardware-Quellen:** `/proc` (cpuinfo, meminfo, cmdline, uptime),
  Disks via `sysinfo`-Crate, `smartctl` (SMART), Sensoren, EFI-Status, `pacman`
  für Paketversionen.
- **Netzwerk:** ermittelt eigene VPN-IP im Netz `fd56:1dda:8794:cb90::/64`
  (Default), um NUR dort zu lauschen — nicht im gesamten Internet.

### Bietet an
HTTP-Endpoints (Default-Port 5000, gebunden an die VPN-IP):
- `POST /configure` — JSON `{ "url": ... }`, setzt Anzeige-URL in Chromium.
- `GET /screenshot` — Screenshot des Systems.
- `GET /sysinfo` — Systeminformationen (JSON).
- `POST /rpc` — JSON-Kommando, siehe unten.
- `GET /verify-portal` — prüft, ob Portal-URL == Chromium-Startseite.
- `GET /portal-url` — liefert die für den Hostname hinterlegte Portal-URL.

RPC-Kommandos (`POST /rpc`): `beep`, `reboot` (optional Delay in s), `identify`,
`configFile`, `restartWebBrowser`, `operationMode` (get, oder set auf
`chromium` / `installationInstructions` / `unconfiguredWarning` / `blackScreen`).

## Deployment / Laufzeit
Läuft als systemd-Service (`assets/digsigctl.service`) unter User/Group `digsig`,
`ExecStart=/usr/bin/digsigctl`, `Restart=on-failure`. `ExecStartPre` wartet per
`curl` auf Erreichbarkeit von `https://portal.homeinfo.de`, bevor gestartet wird.
Beim Start (in eigenem Thread) wird — sofern nicht im Konfigurationsmodus
`hidslcfg.service` — die Startseite gegen die Portal-URL verifiziert und bei
Abweichung neu konfiguriert. Rechte über `assets/digsigctl.sudoers` (NOPASSWD
für definierte `systemctl`- und `smartctl`-Aufrufe). Weitere Assets: udev-/polkit-
Rules für Reboot und PC-Speaker-Beep. ⚠️ ANNAHME: Zielplattform ist Arch-basiert
(HIDSL/DDB OS), da `pacman` genutzt wird; Windows-Pfade existieren im Code, sind
aber vermutlich nicht der produktive Einsatz.

## Ersetzbarkeit
Kopplungsgrad: **mittel**. Nach außen klar über die HTTP-Endpoints und das
Portal-URL-Schema an die Homeinfo-Infrastruktur (`termgr.homeinfo.de`) gekoppelt —
ein Ersatz müsste dieselbe REST-Schnittstelle und das RPC-Kommandoformat
bedienen. Nach innen eng an eine konkrete Geräteumgebung gebunden: feste
systemd-Service-Namen, Chromium-Preferences-Layout, `digsig`-User, sudoers- und
udev-Regeln sowie das VPN-Netz. Der Vorgänger `digsigclt` zeigt, dass die
Fachfunktion prinzipiell portierbar ist; Aufwand steckt in der Nachbildung der
Systemintegration.

## Weitere Doku
- `README.md` (Kurzbeschreibung, Verweis auf Vorgänger `digsigclt`).
- Vorgänger-Repo: https://github.com/homeinfogmbh/digsigclt
- ⚠️ ANNAHME: Zentrales Repo `homeinfo-architektur` (Ordner `komponenten/`) noch
  nicht geprüft — dort ggf. übergeordnete Doku ergänzen/verlinken.
