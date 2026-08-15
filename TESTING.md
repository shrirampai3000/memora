# MEMORA regression testing checklist

> **purpose**: prevent regressions. test core features rigorously every time

## critical edge cases (sorted by regression frequency)

### 1. window overlay & fullscreen spaces (macOS)

### 1.1. Live Text Interaction (macOS)

commits: `e9c76934`, `9acdf850`

- [ ] **Native Live Text selection** â€” On macOS, verify that native Live Text selection works within the app's text overlay.
- [ ] **Native Data Detectors** â€” On macOS, verify that native data detectors (e.g., phone numbers, addresses, dates) are active and clickable within the app's text overlay.
- [ ] **Cross-architecture Live Text compilation** â€” On both x86_64 (Intel) and arm64 (Apple Silicon) macOS machines, verify that Live Text functionality is available and works without compilation errors or runtime issues.


- [ ] **window mode CSS restore** â€” In window mode (not fullscreen), verify that CSS styling is correct and as expected (e.g., no unexpected transparent panels).
- [ ] **keyboard input in main window from tray** â€” Open the main window from the tray icon and immediately try typing. Verify that keyboard input works without requiring a click.
- [ ] **WKWebView keyboard focus recovery** â€” Interact with embedded web views (e.g., billing, help sections), then navigate back to other UI elements. Verify keyboard focus is correctly recovered by the WKWebView.



these break CONSTANTLY. any change to `window_api.rs`, `main.rs` shortcuts, activation policy, or NSPanel code must test ALL of these.

commits that broke this area: `0752ea59`, `d89c5f14`, `4a64fd1a`, `fa591d6e`, `8706ae73`, `6d44af13`, `b6ff1bf7`, `09a18070`

- [ ] **overlay shortcut on fullscreen space** â€” press shortcut while a fullscreen app (e.g., Chrome fullscreen) is active. overlay MUST appear on top.
- [ ] **chat shortcut on fullscreen space** â€” press chat shortcut while on a fullscreen space. chat panel MUST appear on top. Fixed: panel pre-created at startup, show uses order_frontâ†’activate order.
- [ ] **chat shortcut on normal desktop** â€” chat appears, receives keyboard focus, can type immediately.
- [ ] **overlay toggle on/off** â€” press shortcut twice. first shows, second hides. no "ghost" window left behind.
- [ ] **chat toggle on/off** â€” press chat shortcut twice. first shows, second closes.
- [ ] **overlay does NOT follow space swipe** â€” show overlay, then three-finger swipe to another space. overlay should NOT follow you (no blink-and-disappear). was broken by `MoveToActiveSpace` staying set.
- [ ] **no blink on show** â€” overlay appears instantly, no flash of white/transparent then reappear. was broken multiple times (`3097872b`, `8706ae73`, `09a18070`).
- [ ] **no blink on hide** â€” overlay disappears instantly. no momentary reappear after hiding.
- [ ] **overlay on second monitor** â€” with 2 monitors, show overlay. it appears on the monitor where the mouse cursor is.
- [ ] **window mode vs fullscreen mode** â€” switch overlay mode in settings. shortcut still works in both modes. no crash.
- [ ] **switch modes while overlay is visible** â€” change from fullscreen to window mode in settings while overlay is showing. should not crash (`b4eb2ab4`).
- [ ] **keyboard focus in overlay** â€” show overlay, start typing. text input works immediately without clicking (`d74d0665`, `5a50aaad`).
- [ ] **keyboard focus in chat** â€” show chat, start typing. text input works immediately.
- [ ] **escape closes overlay** â€” press Escape while overlay is visible. it hides.
- [ ] **no space jump on show** â€” showing the overlay should NOT cause a space transition animation (`6d44af13`, `d74d0665`).
- [ ] **no space jump on hide** â€” hiding the overlay should NOT switch you to a different space.
- [ ] **screen recording visibility setting** â€” toggle "show in screen recording" in settings. overlay should appear/disappear from screen recordings accordingly (`206107ba`).
- [ ] **search panel focus** â€” open search, keyboard focus is in search input immediately (`2315a39c`, `1f2681e3`).
- [ ] **ghost clicks after hide** â€” hide overlay via `order_out`. clicking where overlay was should NOT trigger overlay buttons (`32e1a962`).
- [ ] **pinch-to-zoom works** â€” pinch gesture on trackpad zooms timeline without needing to click first (`d99444a7`, `523a629e`).
- [ ] **shortcut reminder on all Spaces** â€” switch between 3+ Spaces (including fullscreen apps). reminder pill stays visible on every Space simultaneously.
- [ ] **shortcut reminder on fullscreen app** â€” fullscreen Chrome/Safari, reminder shows at top center. not just leftmost Space.
- [ ] **shortcut reminder doesn't steal focus** â€” showing reminder never takes keyboard focus from active app.
- [ ] **chat on non-primary Space** â€” switch to Space 3 (normal desktop), press chat shortcut. chat appears on Space 3, not Space 1. no Space transition animation.
- [ ] **chat re-show on fullscreen Space** â€” show chat on fullscreen Space, hide it, show again. must reappear on same fullscreen Space.
- [ ] **space monitor only hides main overlay** â€” swipe Spaces. main overlay hides. chat window and shortcut reminder are unaffected.
- [ ] **space monitor doesn't race with show** â€” show overlay via shortcut. the `activateIgnoringOtherApps` call must not trigger space monitor's hide callback.
- [ ] **Chat streaming UX** â€” Verify that chat streaming uses a state-aware grid dissolve loader for a smooth user experience.
- [ ] **chat always-on-top toggle** â€” Toggle the "chat always-on-top" setting and verify that the chat window behaves as expected (e.g., stays on top of other applications when enabled). (`b6c363e5`)
- [ ] **overlay hidden in OBS when screen recording toggle is off** â€” Verify that the overlay is NOT visible in OBS (or other screen capture tools) when the "show in screen recording" toggle is off. (`87d107a29`)
- [ ] **resizable shortcut overlay** â€” Change shortcut overlay size (small/medium/large) in settings and verify it updates correctly on all spaces. (`1e1e17171`)
- [ ] **overlay resize support for webview fallback** â€” Verify that the overlay can be resized even when using the webview fallback. (`d095f5994`)
- [ ] **text selection not blocked by URL overlays** â€” On URL-heavy pages, verify that text selection is not blocked by clickable URL overlays. (`eb9e65b4`)
- [ ] **macOS focused-app capture with AX observers** â€” On macOS, verify that focused-app capture works correctly when switching between applications, utilizing AX observers. (`22830119`)
- [ ] **macOS native Live Text interaction** â€” On macOS, verify that native Live Text interaction, including text selection and data detectors, is re-enabled and functions correctly. (`e9c76934`)
- [ ] **Livetext single worker thread** â€” verify no GCD thread exhaustion freeze during heavy livetext analysis. (`a3e29d42a`)
- [ ] **VisionKit semaphore timeouts** â€” verify no deadlocks in vision pipeline if VisionKit hangs (10s timeout). (`397f46133`)
- [ ] **Notification panel order_out** â€” verify no ghost clicks after hiding notification/shortcut panels. (`32fed7c8c`)
- [ ] **Excluded windows from screenshots** â€” Verify that windows specified in the ignore list are correctly excluded from full-monitor screenshots taken via ScreenCaptureKit (SCK). (`61212c429`)
- [ ] **Swift overlay meeting toggle** â€” Verify that the meeting toggle in the Swift-based overlay works correctly and reflects the recording state. (`e5e955aa6`)
- [ ] **shortcut reminder dot anchor** â€” with a meeting live, hover the red dot on the pill. the dot must NOT move (it stays on the bell in both states) and the transcript card drops under it. collapsed pill stays screen-centred; the expanded bar grows leftwards only. re-check at all three overlay sizes.
- [ ] **shortcut reminder hover target** â€” while collapsed, move the cursor across the top of the screen ~150pt to the LEFT of the pill. the bar must NOT pop open â€” only the visible pill is hover-tracked.
- [ ] **shortcut reminder health states stay centred** â€” trigger a recording failure. the "recording needs help" pill is centred on screen (not anchored right), and returns to the centred collapsed pill after recovery.


### 2. dock icon & tray icon (macOS)

commits that broke this area: `0752ea59`, `7562ec62`, `2a2bd9b5`, `f2f7f770`, `5cb100ea`

- [ ] **dock icon visible on launch** â€” app icon appears in dock immediately on startup.
- [ ] **tray icon visible on launch** â€” tray icon appears in menu bar on startup.
- [ ] **dock icon persists after overlay show/hide** â€” show and hide overlay 5 times. dock icon must remain visible every time. was broken by Accessory mode switches.
- [ ] **tray icon persists after overlay show/hide** â€” same test. tray icon must remain visible.
- [ ] **dock right-click menu works** â€” right-click dock icon. "Show MEMORA", "Settings", "Check for updates" all work (`d794176a`).
- [ ] **tray menu items don't fire twice** â€” click any tray menu item. action happens once, not twice (`9e151265`).
- [ ] **tray health indicator** â€” tray icon shows green (healthy) or yellow/red (issues) based on recording status.
- [ ] **tray doesn't show false "Error" on transient issues** â€” Under load (running many pipes, high CPU), tray icon should not flip to "Error" if recording is actually working. Only sustained errors (>2min) show red. Tests under transient DB/OCR/audio pressure. (`abc234aae`)
- [ ] **tray on notched MacBook** â€” on 14"/16" MacBook Pro, tray icon is visible (not hidden behind notch). if hidden, user can Cmd+drag to reposition.
- [ ] **activation policy never changes** â€” after ANY user interaction, dock icon should remain visible. no Accessory mode switches. verify with: `ps aux | grep MEMORA`.
- [ ] **no autosave_name crash** â€” removed in `2a2bd9b5`. objc2â†’objc pointer cast was causing `panic_cannot_unwind`.
- [ ] **no recreate_tray** â€” recreating tray pushes icon LEFT (behind notch). must only create once (`f2f7f770`).
- [ ] **tray upgrade button opens in-app checkout** â€” Verify that clicking the tray's upgrade button correctly opens the in-app checkout experience. (`078fcfb2`)
- [ ] **modernized tray menu** â€” Verify the tray menu's updated layout and functionality match the modernized design. (`b6c363e5`)
- [ ] **Recording toggle in tray** â€” Verify that the tray menu has a single toggle to start/stop recording (replacing separate items). (`cdc1d0fd9`)
- [ ] **Headless webview teardown** â€” Enable Headless, close Home, and verify every app webview process exits while recording, scheduled pipes, and the existing tray icon remain active. Global shortcuts must do nothing while dormant; opening MEMORA from the tray must recreate Home and restore shortcuts.
- [ ] **Headless record-only mode** â€” Enable Headless â†’ Record only, close Home, and verify recording and the tray remain active while scheduled pipe occurrences are consumed as no-ops. Open MEMORA from the tray and verify pipes wait for their next future schedule without a catch-up burst.
- [ ] **Enterprise hidden-UI policy reversal** â€” With enterprise hidden UI active, turn the server policy off while the app is running. Home should reopen on the next policy refresh, the dock/full tray and global shortcuts should return, and `~/.MEMORA/enterprise.json` should persist `hide_app: false`. If the user separately enabled Headless, the app should remain dormant.

### 3. monitor plug/unplug

commits: `28e5c247`

- [ ] **unplug external monitor while recording** â€” recording continues on remaining monitor(s). no crash. log shows "Monitor X disconnected".
- [ ] **plug in external monitor while recording** â€” new monitor is detected within 5 seconds. recording starts on it. log shows "Monitor X reconnected".
- [ ] **unplug and replug same monitor** â€” recording resumes. same monitor ID reused. no duplicate recording tasks.
- [ ] **unplug all external monitors (laptop only)** â€” built-in display continues recording. no crash.
- [ ] **plug monitor with different resolution** â€” recording starts at correct resolution. OCR works on new monitor.
- [ ] **"use all monitors" setting** â€” with this ON, all monitors auto-detected. no manual configuration needed.
- [ ] **specific monitor IDs setting** â€” with specific IDs configured, only those monitors are recorded. unplugging a non-configured monitor has no effect.
- [ ] **resolution change (e.g., clamshell mode)** â€” closing MacBook lid with external monitor. recording continues on external.
- [ ] **queue stats after unplug** â€” check logs. no queue stats for disconnected monitor after disconnect.
- [ ] **--use-all-monitors flag override** â€” Verify that the `--use-all-monitors` CLI flag correctly overrides tier-based defaults (e.g., if a tier defaults to a single monitor, the flag should still enable all monitors). (`bd5b94328`)

### 4. audio device handling

- [ ] **meetings-only releases configured audio devices outside meetings** â€” Run `bun run test:e2e:meetings-only-audio` on macOS and Windows. The isolated real-audio lane must observe `0` running devices before a manual meeting, `>=1` while it is active, then `0` within one monitor tick after stop. `/health` must report `waiting_for_meeting` rather than a capture fault while idle. (#5611)
- [ ] **CoreAudio Process Tap selection and fallback** â€” desktop migration V3 automatically enables `experimentalCoreaudioSystemAudio` on macOS 14.4+; the Rust serde/headless default remains OFF. With it ON, System Audio uses the CoreAudio Process Tap; initial creation failures fall back to SCK, while unrecoverable runtime rebuild failures disconnect the stream so the device manager reconstructs it through normal backend selection. (`75a52603b`, `5634664da`, #5236, #5880)
- [ ] **CoreAudio tap does not own or perturb the meeting speaker** â€” while MEMORA records continuously with the Process Tap, inspect the private aggregate composition and verify it contains the tap list but no `main_sub_device` or physical output subdevice/UID (CoreAudio may materialize an empty `sub_device_list`). Complete 8 cold Zoom joins and 8 Slack Huddle joins with default audio; each must connect without stopping MEMORA or pressing `Cmd+Shift+4`, and system-audio capture must remain live before and after each join. Leave at least one Zoom join silent for more than 45 seconds: zero-filled callbacks are legitimate pre-call/idle delivery and must not stop or rebuild the tap generation. (#5880)
- [ ] **CoreAudio tap route and signal matrix** â€” run `cargo run -p MEMORA-audio --example coreaudio_tap_signal_probe -- 60`, play a known 1 kHz tone, and switch the default output through at least one 44.1 kHz virtual device, one 48 kHz virtual device, and a 96 kHz physical device. Every route must emit 24,000-sample/500ms windows at 48 kHz, show `a1000` near the source amplitude and well above `a440`, avoid `stall:true`, and keep one tap generation. Then run `cargo test -p MEMORA-audio inclusion_capture_repeated_lifecycle_stress --lib -- --ignored --nocapture` to exercise repeated create/start/stop teardown. (#5880)
- [ ] **meeting piggyback OFF (default)** â€” with `experimentalMeetingPiggyback` off, a meeting starts/ends with zero device-set changes: no "Meeting Tap" device, no suspensions, logs contain no `meeting_piggyback` actions.
- [ ] **meeting piggyback ON, detected meeting** â€” flag on ("Smart recording" in settings), ANY capture mode, macOS 14.4+: join a Zoom call with a NON-default mic selected in Zoom â†’ within ~4s a "Meeting Tap (output)" session stream starts, and within ~6s (two confirmation ticks â€” we never race the app's own device acquisition) the Zoom-selected mic is capturing; the global "System Audio (output)" stream and non-resolved mics are suspended; transcripts attribute to "Meeting Tap"/the resolved mic; on meeting end everything reverts and the resolved mic is NOT left in enabled devices (settings unchanged).
- [ ] **meeting apps always keep the mic (calm near-end)** â€” flag on, AirPods (or any BT mic) as the enabled/default mic: join a Google Meet/Zoom call and stay in it 10+ minutes, then switch mics inside the app a few times. The app must NEVER lose its microphone ("can't connect to microphone", capture dying) and the logs must show ZERO MEMORA open/close cycles of the app's current mic during the call â€” no per-tick `starting recording`/`Stopping device` churn on it, no suspend/resume oscillation of the other mics (displacement is a one-shot latch), and any failed mic open retries at 10s/30s/60s backoff, never every 2s. (Root cause 2026-07-07: the old near-end opened the resolved mic the tick it appeared and re-evaluated displacement per tick â€” each cycle a real BT open/close â†’ SCO renegotiation storm that starved Meet/Zoom; see `MicFollow` in `meeting_piggyback.rs`.)
- [ ] **piggyback takes precedence over capture mode** â€” flag on, capture mode "always" (continuous): joining a meeting engages the piggyback exactly as in meetings-only (Meeting Tap + resolved mic, global output and non-resolved mics suspended); on meeting end continuous capture resumes on the user's configured devices with no leftover session streams or suspensions.
- [ ] **manual meeting piggyback (all mic-holders)** â€” flag on, start a MANUAL meeting while an app holds the mic: within ~6s (two-tick pid adoption) a Meeting Tap session starts over ALL mic-holding processes and their resolved mics are captured; a second app opening a mic mid-meeting rebuilds the tap over the widened set (one `rebuilding tap` log, NO strike, no System Audio resume/suspend churn); ending the manual meeting reverts everything. MEMORA's own processes must never appear in the tapped set.
- [ ] **enabled resolved mic displaces others (D1, latched)** â€” two mics enrolled, meeting app using one of them (no session stream needed): once the resolved mic delivers, the OTHER enrolled mic is suspended ONCE (single transcription stream, no re-suspend churn); a transient stall of the resolved mic (BT renegotiation) does NOT resume the others â€” they come back only when the app releases its mics, the resolved capture stays dead for 60s+, or the meeting ends; after such a lift, the meeting app recovering onto an enrolled mic re-latches displacement once.
> **OWNERSHIP REQUIRES DELIVERY.** Smart recording is the sole capture owner for a
> confirmed meeting (#6072), but only while it can actually deliver:
> `piggyback_may_own_capture(tap_available, meeting_pid_count, tap_ever_started)`.
> Once a tap has started for the meeting, ownership is kept through any later gap â€”
> resuming there would re-create the device churn #6072 removed. Ownership is
> released only when the piggyback has never delivered AND cannot contend: no
> meeting pids (nothing is holding the devices) or no tap support on this OS.
>
> #6072 originally took ownership unconditionally, so a meeting whose tap never
> built recorded nothing: on 2.6.1 an 86s manual meeting produced 3,584 samples
> (~0.22s) and an empty transcript, while identical audio captured normally with no
> meeting active. Exhausted strikes deliberately still hold ownership â€” a meeting
> app really is holding devices there, so resuming needs device-level validation.

- [ ] **tap build failure keeps recording** â€” flag on, force tap failure (revoke System Audio Recording permission) BEFORE any tap starts: the user's configured capture keeps running for the whole meeting and audio is recorded. Confirm â‰¤3 retries per meeting with 60s cooldown, and that `piggyback_meeting_summary` does NOT carry `captured_no_audio: true`.
- [ ] **tap build failure is backend-neutral** â€” repeat with `experimentalCoreaudioSystemAudio` ON: normal capture keeps running either way, and the System Audio stream comes up on whichever backend the user's settings select.
- [ ] **app quits mid-meeting** â€” kill the meeting app: tap stream tears down within ~2s; no crash, no rebuild storm. A tap HAS started for this meeting, so suspensions are deliberately kept until the watcher ends it.
- [ ] **no pid (ui_scan/reattach) keeps recording** â€” a DETECTED meeting whose sensor can't identify the process (ui_scan, post-restart reattach) leaves normal capture running; nothing is holding the devices, so there is nothing to contend with. Manual meetings are NOT in this bucket â€” they derive their pid set from the live mic-holder enumeration (see the manual-piggyback item above).
- [ ] **a gap AFTER a working tap still suspends** â€” join a real call so the tap starts, then make the pid set go empty: suspensions must be KEPT (no resume churn). This is the anti-SCO-storm behaviour from #6072 and must not regress while fixing the silent-meeting case.
- [ ] **silent meetings are detectable** â€” if a meeting ever DOES capture nothing, `piggyback_meeting_summary` must carry `captured_no_audio: true`. The `outcome` strings (`stable_fallback`, `no_pid`, `unavailable`) are all reported for meetings that captured nothing and all read as benign, so `outcome` alone cannot distinguish a silent meeting from a healthy one.
- [ ] **the gate is visible in logs** â€” with a meeting active and the flag on, released builds (INFO level) must log `smart recording owns capture for this meeting â€” not starting normal audio device: <device>`. This used to be DEBUG, i.e. invisible in shipped builds, which is why a silent meeting left no trace anywhere.
- [ ] **browser pop-out / Little Arc meeting detection** â€” join a Google Meet call in a Chrome pop-out window or an Arc "Little Arc" window, then focus a different app: the meeting is detected within one poll of the mic being held (log shows the `UnresolvedBrowser` -> `Active` promotion); a non-meeting window whose title merely looks like a meeting code (e.g. a doc named "abc-defg-hij") with no mic held must NOT trigger detection. (`ff0337416`)
- [ ] **piggyback on unsupported OS** â€” flag on, macOS <14.4 / Windows <20348: one-time "using the stable capture path" log. The meeting now genuinely rides the stable path: normal capture keeps running and audio is recorded (`outcome: unavailable`, and NOT `captured_no_audio`). The Meeting Tap stream must not silently downgrade to endpoint-wide loopback.
- [ ] **mid-call mic switch in the app** â€” change Zoom's mic mid-call: capture follows within ~6s (two confirmation ticks AFTER Zoom has the new device â€” the app always acquires first; old session mic stops after two missed resolver ticks, new one starts); changing the OS default while Zoom is pinned elsewhere does NOT switch capture. Switching onto a mic the sweep had displaced only clears its suspension flag â€” no immediate restart racing the app.
- [ ] **mic switch must not flap the meeting** â€” switch the meeting app's mic (AirPods â†’ phone â†’ built-in, several times): `meeting_status_changed` stays active=true throughout â€” NO end/re-start pair ~20s after each switch, no Meeting Tap teardown/rebuild, no System Audio resume/suspend churn. (Root cause was device-derived session keys: macOS synthesized the audio-session id from the device set and WASAPI GUIDs are per-endpoint, so a mic switch re-keyed the session and the meeting rode the 20s ending grace into EndMeeting + instant restart.)
- [ ] **piggyback never acts on silence** â€” during a piggybacked call, silence must never make the piggyback rebuild, restart, probe, or notify: mute in the meeting app (or leave a silent tab rendering nothing) for 5+ min â†’ NO notification, NO Meeting Tap rebuild, NO resolved-mic restart, and the logs contain no silence-driven piggyback actions. A silent meeting device is the user's own in-meeting feedback loop â€” they fix it in the app and the mic/output follow tracks the switch. (The GLOBAL System Audio tap's silence watchdog â€” item above, `75a52603b` â€” is unchanged and out of scope here.) Dead-stream handling still applies: hard open failures notify once via `audio_capture_health_mic_capture_failed`, and dead capture streams retry through `MicFollow`'s backoff.
- [ ] **Windows per-process tap supervisor** â€” Windows 20348+: verify target-exit detection (app quits -> Meeting Tap stops and stable capture resumes) and default-endpoint loopback re-anchor for explicit global loopback. There is deliberately NO per-pid silence watchdog (see the item above â€” the piggyback never acts on silence). Confirm tap startup failure falls back through the user's normal/default audio settings, not through an implicit low-level endpoint-loopback downgrade inside the Meeting Tap.
- [ ] **piggyback telemetry** â€” flag on (any capture mode): end a piggybacked meeting â†’ one `piggyback_meeting_summary` appears in PostHog Live Events with the correct outcome; a failed meeting-mic open appears as `audio_capture_health_mic_capture_failed` (there are no mic-silent events â€” the piggyback never acts on silence); analytics toggle OFF â†’ nothing is sent (engine `TELEMETRY_ENABLED` gate).


- [ ] **default audio device** â€” with "follow system default", recording uses whatever macOS says is default.
- [ ] **plug in USB headset** â€” if set to follow defaults and macOS switches to headset, recording follows.
- [ ] **unplug USB headset** â€” recording falls back to built-in mic/speakers. no crash. no 30s timeout errors.
- [ ] **bluetooth device connect/disconnect** â€” AirPods connect mid-recording. audio continues without gap.
- [ ] **no audio device available** â€” unplug all audio. app continues (vision still works). log shows warning, not crash.
- [ ] **audio stream timeout recovery** â€” if audio stream times out (30s no data), it should reconnect automatically.
- [ ] **per-device audio-timeout recovery** â€” Force one microphone timeout, then prove that only later usable microphone audio clears `active_no_data`; healthy system output alone must not clear it. Repeat with system output timed out and microphone live. Verify a one-shot timeout expires before the native 90-tick alert, repeated zero-fill/receive timeouts keep the failure active, a recovered device clears immediately, an unresolved current device still raises `recording needs help`, and removing/deselecting the failed device stops it from degrading current capture. Run this after a packaged macOS display/wake transition.
- [ ] **multiple audio devices simultaneously** â€” input (mic) + output (speakers) both recording. both show in device list.
- [ ] **disable audio setting** â€” toggling "disable audio" stops all audio recording. re-enabling restarts it.
- [ ] **Metal GPU for whisper** â€” transcription uses GPU acceleration on macOS (`f882caef`). verify with Activity Monitor GPU tab.
- [ ] **Qwen3-asr OpenBLAS** â€” On Linux/Windows, verify that qwen3-asr uses OpenBLAS for improved transcription performance. (`e64ee25f4`)
- [ ] **Batch transcription mode** â€” Verify that batch transcription mode works correctly with both cloud and Deepgram engines.
- [ ] **Cloud transcription batch capping** â€” Send large audio chunks (>200s) to cloud transcription. Verify they are correctly capped/split and do not trigger Cloudflare 413 errors. (`792145ac6`)
- [ ] **Lower RMS threshold for batch mode output devices** â€” In batch transcription mode, verify that output devices correctly use a lower RMS threshold.
- [ ] **OpenAI-compatible STT connection test** â€” Configure OpenAI-compatible STT, then use the connection test feature. Verify it accurately reports connection status.
- [ ] **OpenAI-compatible STT editable model input** â€” When using OpenAI-compatible STT, verify that the model input fields are editable.
- [ ] **OpenAI-compatible STT with custom vocabulary** â€” Configure OpenAI-compatible STT with a custom vocabulary. Verify that transcription accuracy improves when this vocabulary is present in the audio. Verify that vocabulary is sent as both prompt and context. (`d3a4b6bcc`)
- [ ] **OpenAI-compatible transcription engine support** â€” Enable and configure the OpenAI-compatible transcription engine. Verify that audio is correctly captured and transcribed using this engine.
- [ ] **"transcribing..." only for recent chunks** â€” Verify that the "transcribing..." caption/indicator only appears for audio chunks that are less than 2 minutes old. (`b70116b`)
- [ ] **no transcribing caption on old silent chunks** â€” Verify that old silent audio chunks do not trigger or display a "transcribing..." caption. (`54a550f4`)
- [ ] **silent chunks deleted, not stored** â€” After periods of silence, verify that no empty transcription rows are stored in the database for silent audio chunks, and they are instead correctly deleted. (`cb2cc205`)
- [ ] **silent chunk zombie loop prevention** â€” Verify that silent audio chunks do not lead to a "zombie loop" resulting in excessive CPU usage or large log files. (`6b3a71eb`)
- [ ] **write-ahead transcription cache performance** â€” Verify that the write-ahead transcription cache improves the performance and responsiveness of audio transcription. (`46350671`)
- [ ] **enhanced audio pipeline diagnostics** â€” Check logs and verify that enhanced audio pipeline diagnostics provide useful and accurate information. (`2e68400c`)
- [ ] **audio start/stop shortcuts toggle capture** â€” Verify that the audio start/stop shortcuts correctly toggle audio capture on and off. (`3701cce2`)
- [ ] **bulk import transcription dictionary** â€” Verify that the bulk import functionality for the transcription dictionary works correctly, including smart delimiter detection. (`73adc9d4`)
- [ ] **Audio start/stop shortcuts** â€” Verify that designated audio start/stop shortcuts reliably toggle audio capture on and off. Check logs for corresponding start/stop events.
- [ ] **Filter music toggle UI** â€” Verify that a "filter music" toggle exists in recording settings and correctly enables/disables music filtering.
- [ ] **Music detection thresholds** â€” With "filter music" enabled, play various types of music. Verify that music is correctly detected and filtered, and that non-music speech is still captured.
- [ ] **Audio reconciliation FK constraint loop** â€” Verify that audio reconciliation does not enter an infinite retry loop on foreign key constraints. (`e9e2dc252`)
- [ ] **Skip reconciliation when transcription disabled** â€” Disable audio transcription in settings. Verify that audio reconciliation is skipped. (`ceb77559d`)
- [ ] **read-pool pressure never costs a transcription** â€” Put the read pool under load (open the timeline, run a broad search, run several pipes) while audio is being transcribed. Verify transcriptions still land: no insert may fail in its pre-read phase. Logs may say "cross-device dedup check unavailable"; that is the intended fail-open â€” a duplicate cross-device transcription is acceptable, losing the recording is not. (`MEMORA-CLI-SN`)
- [ ] **a batch rollback is not reported as a crashed worker** â€” When one write in a batch fails, the others are rolled back with it. Verify those log "was rolled back with its batch â€¦ retrying", not "attempted to communicate with a crashed background worker". Nothing crashed and their write was never attempted. (`MEMORA-CLI-SN`)
- [ ] **dead System Audio auto-reconnect** â€” Simulate a dead system audio stream. Verify it auto-reconnects and resumes capture. (`0f287761d`)
- [ ] **SCK System Audio dead-display recovery (clamshell)** â€” Mid-call, close the MacBook lid with an external display attached â†’ remote-participant (System Audio) audio resumes within ~1 min via auto re-anchor, mic unaffected. AND: leave the machine idle (nothing playing) for 10 min with stable displays â†’ NO System Audio rebuild/churn (guards the reverted output recv-timeout `0f287761d` / `357e4dfcc`). Watchdog: `core::sck_output_watchdog` (#3901).
- [ ] **per-device audio toggle** â€” In the tray menu, verify you can toggle recording for individual audio devices. (`3ee3defcb`)
- [ ] **per-monitor tray toggle** â€” With 2+ displays, open the tray recording section and uncheck one monitor. Verify only that display stops screen capture; re-check resumes it. Pause one display individually, then global pause/resume â€” the individually paused display stays off. (`4685`)
- [ ] **stable audio device order** â€” Verify that audio devices listed in the tray menu maintain a stable order across refreshes. (`4577ac8a6`)
- [ ] **Mic disconnect false-positives on sleep/wake** â€” Put the computer to sleep and wake it up. Verify that no false-positive mic disconnect notifications or logs are generated. (`796baa619`)


#### Audio device recovery (monitor unplug / device switch)

commits: device_monitor.rs atomic swap, tiered backoff, empty device list guard

- [ ] **unplug monitor during active Zoom call** â€” output audio recovers within 15 seconds. Verify: `grep "DEVICE_RECOVERY.*output.*restored" ~/.MEMORA/MEMORA-app.*.log`. Verify: `curl localhost:3030/search?content_type=audio&limit=5` shows output device transcriptions resume.
- [ ] **unplug and replug monitor within 5 seconds** â€” no audio gap. both input and output continue. Verify: no "stopping" log for input device.
- [ ] **unplug monitor, wait 2 minutes, replug** â€” output recovers both times. Verify: two `DEVICE_RECOVERY` log entries.
- [ ] **switch audio output (AirPods â†’ speakers) during call** â€” output audio continues with <5s gap. Old device kept running until new one starts (atomic swap).
- [ ] **health endpoint during output recovery** â€” `curl localhost:3030/health` shows `device_status_details` with output device present within 15 seconds of recovery.
- [ ] **SCK transient failure doesn't cascade** â€” if ScreenCaptureKit returns empty device list, running devices are NOT disconnected. Verify: `grep "device list returned empty" ~/.MEMORA/MEMORA-app.*.log` shows warning but no disconnections.
- [ ] **DB gap query after device switch** â€” run: `sqlite3 ~/.MEMORA/db.sqlite "SELECT t1.timestamp as gap_start, t2.timestamp as gap_end, (julianday(t2.timestamp) - julianday(t1.timestamp)) * 86400 as gap_seconds FROM audio_transcriptions t1 JOIN audio_transcriptions t2 ON t2.id = (SELECT MIN(id) FROM audio_transcriptions WHERE id > t1.id AND is_input_device = 0) WHERE t1.is_input_device = 0 AND (julianday(t2.timestamp) - julianday(t1.timestamp)) * 86400 > 60 ORDER BY t1.timestamp;"` â€” should return no rows if output was continuously captured.
- [ ] **Bluetooth audio device hijack recovery** â€” Join a video call (e.g., Zoom, Proton Meet) on AirPods, end the call, join another call ~1 min later on same AirPods. Verify the second call is fully captured (audio_chunks created throughout). CoreAudio sometimes delivers zero-fill when another app has exclusive claim; the watchdog detects sustained silence (>30s of exact zeros) after initial healthy audio and rebuilds the device. Regression: `a2e89b2ae` (initial detection), fixed by `357e4dfcc` (only trip watchdog after stream is healthy). USB devices that never produce real audio should NOT trigger rebuild storms.
- [ ] **Manual-mode pinned-input fallback** â€” In manual mode (NOT "Follow System Default"), pin only AirPods (input) as your mic. Mid-recording, turn off the AirPods. Verify: within ~20-25s the monitor engages the system default mic as a substitute and capture continues. Re-connect AirPods. Verify the substitute is torn down and capture returns to AirPods. Log markers: `grep "PINNED_FALLBACK" ~/.MEMORA/MEMORA-app.*.log` shows `pinned input '...' missing > 20s, capturing from system default '...' until it returns` then `clearing fallback '...': pinned input returned`. Edge: if you also user-disabled the default mic (privacy mode), expect "system default ... is user-disabled â€” no fallback engaged" log + zero capture rather than auto-fallback.
- [ ] **Pinned-input fallback when the dead device IS the system default (or there is none)** â€” Same as above, but make AirPods BOTH the pinned input AND the macOS default input (the common real case), then turn them off. Verify capture continues from the built-in mic within ~20-25s â€” NOT zero capture. The decider now fails over to any *available* input when the system default is unusable (it IS the dead pinned device â€” Bluetooth lingers as the registered default â€” or CoreAudio reports no default). Reconnect AirPods â†’ substitute torn down. Sub-checks: (a) prefers an on-board mic over virtual/aggregate inputs (e.g. BlackHole/Aggregate present â†’ built-in chosen); (b) still respects privacy â€” if the only other input is user-disabled, expect zero capture, not auto-fallback; (c) transient empty device list (SCK failure) â†’ no fallback that cycle, no disconnect cascade; (d) sleep/wake flap < grace does not engage a spurious fallback. Regression: `ba23c8531` (ruark 6/18 "no recording either side of the call", frames=0/samples=0 â€” AirPods were the only input AND the default).

#### meeting detection & speaker identification

commits: calendar_speaker_id.rs, meetings.rs, meeting_persister.rs

- [ ] **restart during active meeting** â€” start a 1:1 calendar meeting (2 attendees), quit app mid-meeting, relaunch. meeting re-detected via calendar event still in progress. speaker names assigned. verify: `grep "meeting detected via calendar" ~/.MEMORA/MEMORA-app.*.log` shows detection after restart. verify: `sqlite3 ~/.MEMORA/db.sqlite "SELECT id, name FROM speakers WHERE name != ''"` shows both user and attendee names.
- [ ] **calendar-only meeting detection** â€” schedule a 1:1 meeting with 2 attendees, no meeting app (Zoom/Meet) open. meeting detected purely via calendar. verify: `grep "meeting_started" ~/.MEMORA/MEMORA-app.*.log`.
- [ ] **calendar meeting auto-end** â€” calendar meeting detected, wait past the calendar event end time. meeting_ended fires. verify: `grep "meeting ended via calendar" ~/.MEMORA/MEMORA-app.*.log`.
- [ ] **speaker naming in 1:1** â€” during 1:1 call with userName set in settings, input speaker named as user, output speaker named as other attendee. verify: `curl 'localhost:3030/search?content_type=audio&speaker_name=<attendee>&limit=5'` returns results.
- [ ] **auto-name input speaker** â€” with userName set, after ~2 minutes of speaking into mic, dominant input speaker named. verify: `grep "auto speaker identification: named" ~/.MEMORA/MEMORA-app.*.log`.
- [ ] **speaker names survive restart** â€” speaker named pre-restart stays named post-restart. verify: `sqlite3 ~/.MEMORA/db.sqlite "SELECT id, name FROM speakers WHERE name != ''"` shows same speakers before and after restart.
- [ ] **no duplicate speaker naming on restart** â€” restart during meeting, speakers already named aren't overwritten or duplicated. verify: no duplicate names in speakers table.
- [ ] **meeting detection stability** â€” Verify that meeting detection does not drop when alt-tabbing during long calls. (`7684f1d47`)
- [ ] **speaker search deduplication** â€” Search for speakers in the UI. Verify that results are deduplicated and reassignment targets are stable. (`34a62c053`)
- [ ] **meeting detection regardless of transcription mode** â€” Verify that meeting detection works even when transcription is disabled. (`ef39e728d`)
- [ ] **Windows UI Automation meeting detection** â€” On Windows, join a meeting in a supported app (Zoom, Teams, etc.). Verify detection works via UI element scanning rather than just process focus. (`fe905d6af`, `01eb9cf33`)
- [ ] **macOS Zoom menu bar detection** â€” On macOS, join a Zoom meeting. Verify detection works even if Zoom window is not focused, by scanning menu bar items. (`849372fa9`)
- [ ] **Stop auto-detected meeting from overlay** â€” During an auto-detected meeting, verify that the stop button in the overlay correctly terminates the meeting session. (`403d5b732`)
- [ ] **MLX transcription model reuse** â€” Verify that the MLX transcription model is reused across requests to prevent GPU memory spikes or crashes. (`59deeba19`)
- [ ] **Meeting detection app coverage** â€” Verify detection works for 35+ supported apps and various browser URL patterns. (`e6740eb38`)
- [ ] **Meeting detection UI labels** â€” Verify meeting status shows "starts in Xm" and filters all-day events correctly. (`ef470d9e1`)
- [ ] **Meeting detection support for Signal, WhatsApp, Telegram, and Teams 2** â€” Verify that meetings from these apps are correctly detected and recorded. (`8d2f1a542`, `a74e393e1`)
- [ ] **Browser meetings splitting fix** â€” Verify that meetings in the browser are correctly split into separate events. (`d8ba1dad3`)
- [ ] **Meeting with hidden UI controls** â€” Start a Zoom/Teams meeting. Minimize the meeting window or switch apps (Zoom controls move out of accessibility tree). Verify meeting stays active and does NOT auto-terminate after 30 seconds. Audio output detection prevents false "meeting ended" events. (`4e784f620`)
- [ ] **OpenAI-compatible transcription endpoint** â€” Verify that the `/v1/audio/transcriptions` endpoint works as expected, following the OpenAI specification. (`5a14e9a92`)

### 5. frame comparison & OCR pipeline

commits: `6dd5d98e`, `831ad258`

commits: `6dd5d98e`, `831ad258`

- [ ] **static screen = low CPU** â€” leave a static image on screen for 60s. CPU should drop below 5% (release build). hash early exit should kick in.
- [ ] **active screen = OCR runs** â€” actively browse/type. OCR results appear in search within 5 seconds of screen change.
- [ ] **identical frames skipped** â€” check logs for hash match frequency on idle monitors. should be >80% skip rate.
- [ ] **ultrawide monitor (3440x1440+)** â€” OCR works correctly. no distortion in change detection. text at edges is captured.
- [ ] **4K monitor** â€” OCR works. frame comparison doesn't timeout or spike CPU.
- [ ] **high refresh rate (120Hz+)** â€” app respects its own FPS setting (0.5 default), not the display refresh rate.
- [ ] **very fast content changes** â€” scroll quickly through a document. OCR captures content, no crashes from buffer overflows.
- [ ] **corrupt pixel buffer** â€” sck-rs handles corrupt ScreenCaptureKit buffers gracefully (no SIGABRT). fixed in `831ad258`.
- [ ] **window capture only on changed frames** â€” window enumeration (CGWindowList) should NOT run on skipped frames. verify by checking CPU on idle multi-monitor setup.
- [ ] **Meeting app OCR force** â€” Open a meeting app (Zoom, Teams, Meet). Verify OCR is forced for these apps even if accessibility is available. (`b18ae2253`)
- [ ] **Accessibility automation properties** â€” Verify automation properties (labels, roles, automation IDs) are correctly captured in the accessibility tree across Windows, macOS, and Linux. (`1b7d0db5b`)
- [ ] **Apple Vision per-word OCR fast path** â€” Browse an app with lots of text (e.g., Wikipedia article with 500+ words). Verify OCR database inserts are fast (bulk VALUES insert, not 500 individual RETURNING queries). Check logs for no "Slow DB batch insert" warnings. Regression: `6f3f80dd3` (Apple per-word records now use level="0" for bulk-insert fast path, not level="5" which hit expensive per-row insert).
- [ ] **DB write coalesce queue** â€” Under heavy load (e.g. many pipes + high FPS), verify no "database is locked" errors and no vision stalls due to write contention. (`39c016cb3`, `d119d060d`, `231521192`)
- [ ] **static-screen does not trigger vision-stall WARN** â€” Sit on a Zoom call / slide deck / IDE waiting screen for >2 minutes (no UI activity). Verify the engine does NOT log `health_check: no unique vision frame in Ns` warnings, and the Tauri app's `consecutive_vision_stall` counter stays at 0 (no `vision capture recovered after N stale checks` info line). Dedup-skipped captures must tick `last_db_write_ts` so the health check distinguishes "static screen, dedup working" from "pipeline stuck". Pre-fix: 8â€“14 false alarms/day with single stretches up to 28 minutes. (`a08ec9140`, +follow-up commit)
- [ ] **capture loop survives a wedged ScreenCaptureKit call (macOS)** â€” SCK completion handlers (`SCShareableContent`, `updateContentFilter`, `startCapture`) can silently never fire while the machine is idle; every such call must be bounded so one wedged callback can never freeze the capture loop, starve the 30s idle fallback, or flip `/health` `frame_status` to `stale` on a healthy static screen (the false "recording needs help" incident). Run `bun run test:e2e:capture-loop-liveness:macos` and verify the app log contains `e2e: injecting one hung visual-change probe` (the injection actually fired) while `capture_attempts` keeps advancing. Also verify bare `stale` alone never confirms the desktop incident before its user-presence tier (90s with fresh UI activity, 15 min idle). Live-incident evidence: 6 thread samples wedged in `fetch_shareable_content â†’ pthread_join`, 2026-07-26.
- [ ] **focus-cold display does not trigger recording-health alert (macOS)** â€” On a multi-monitor setup, select one display for recording and work on another long enough for the selected display to enter focus-aware Cold state. Capture attempts may stop, but `capture_loop_heartbeats` must keep advancing, `/health frame_status` must remain `ok`, and the recording-health overlay must stay normal. Run the deterministic full-app regression with `bun run test:e2e:recording-health-focus-cold:macos`; it accelerates the pre-fix attempt-clock false stale transition without changing release thresholds.
- [ ] **gone-silent capture recovers end to end (macOS)** â€” Run `bun run test:e2e:capture-stall-recovery:macos` on a host with Screen Recording permission. The lane first pauses every selected display, verifies `/health frame_status=disabled` and a normal recording-health pill, resumes them, and proves capture progress returns. It then parks one SCK frame worker and proves the bounded, unfiltered CoreGraphics escape hatch can reach a real terminal capture outcome; its isolated seed clears window filters because production must fail closed whenever SCK window-id exclusions are active. It freezes `capture_attempts`, reaches `/health frame_status=stale` and `recording needs help`, triggers exactly one `gone-silent stall` VisionManager restart from per-monitor detection, resumes attempts and terminal outcomes in the same app process, shows `recording again`, and returns the pill to normal. A healthy terminal outcome is a write, dedup, or explicit corrupt-frame skip; failed/time-out captures remain failures and cannot satisfy the recovery proof. Unit coverage must also prove one healthy monitor cannot mask a silent sibling. The same lane parks one id-based `SCShareableContent` lookup: it must return inside the bounded timeout and the next lookup must enumerate real display dimensions. The test uses isolated `MEMORA_PORT=3041` and a debug-only `ignore-disk-pressure` seed so host free-space changes cannot replace the intended fault with a genuine safety shutdown; never point it at the developer's production instance.
- [ ] **Windows idle CPU reduction** â€” Verify low CPU usage on Windows when screen is idle, using event-driven hooks and caching. (`d2c9d1fb8`)
- [ ] **reduced CPU spikes in vision/capture pipeline** â€” Actively browse and use applications, verifying that CPU spikes in the vision/capture pipeline are significantly reduced. (`8f7294e6`)
- [ ] **OCR bounding boxes normalized on Windows/Linux** â€” On Windows and Linux, verify that OCR bounding boxes are correctly normalized to the 0-1 range, ensuring consistent text overlay and interaction. (`aba74513`)
- [ ] **Debounced monitor capture errors** â€” Simulate transient monitor capture errors. Verify that these errors are debounced and do not lead to excessive error logging or app crashes.
- [ ] **Focus-aware capture** â€” Enable "Only record focused monitor" in settings. Verify that MEMORA only captures frames and runs OCR for the monitor that currently has the focused window. (`886b5c05d`)
- [ ] **no stranded WGC session after persistent-capture disable (Windows)** â€” Force repeated persistent WGC init failures on one monitor (e.g. flaky driver, monitor sleep during init) while captures overlap (event-driven engine's capture timeout drops the future but the detached blocking closure keeps running, so two closures can race on the same monitor's shared state). After the "persistent capture disabled for monitor N" warning, verify no live WGC session remains: GPU usage for the app drops to per-frame-capture levels and no `CopyResource` work continues for that monitor. The disable path drains any concurrently stored session under the mutex, and the store path re-checks the disable flag under the same mutex â€” a session stored behind `persistent_capture_disabled == true` would otherwise leak GPU textures until refresh/stream release.

### 6. Battery Saver Mode

commits: `d5a9d052`, `0b32cc9a`, `ca29a67b`

- [ ] **Battery Saver mode functionality** â€” Enable Battery Saver mode. Verify that capture adjustments (e.g., reduced FPS, paused capture) occur as expected when the device's power state changes (e.g., unplugging/plugging power, low battery).
- [ ] **Faster power state UI updates** â€” Change the device's power state (e.g., unplug/plug power). Verify that the UI updates quickly and accurately reflects the current power state and capture mode.
- [ ] **Correct default power mode** â€” On a fresh install or after a reset, verify that the default power mode is set to "performance" until Battery Saver mode is explicitly enabled or configured.

### 7. permissions (macOS)

commits: `d9d43d31`, `620c89a5`, `14acf6f0`

- [ ] **fresh install â€” all prompts appear** â€” screen recording, microphone, accessibility prompts all show on first launch.
- [ ] **denied permission â†’ opens System Settings** â€” if user previously denied mic permission, clicking "grant" opens System Settings > Privacy directly (`620c89a5`).
- [ ] **permission revoked while running** â€” go to System Settings, revoke screen recording. app shows red permission banner within 10 seconds.
- [ ] **permission banner is visible** â€” solid red `bg-destructive` banner at top of main window when any permission missing. not subtle (`9c0ba5d1`).
- [ ] **permission recovery page** â€” navigating to /permission-recovery shows clear instructions.
- [ ] **startup permission gate** â€” on first launch, permissions are requested before recording starts (`d9d43d31`).
- [ ] **faster permission polling** â€” permission status checked every 5-10 seconds, not 30 (`d9d43d31`).
- [ ] **No recurring permission modal after close** â€” Grant macOS permissions, quit the app, and relaunch it multiple times. Verify that the macOS permission modal does NOT reappear every time the app is closed.
- [ ] **Screen Recording Later preserves restart consent** â€” Grant Screen Recording, click **Later** in macOS, and verify MEMORA stays open with a clear **restart MEMORA** button and the warning that MEMORA will not work until restarted. Only clicking that in-app button may relaunch the app. Run `bun run test:e2e:screen-recording-restart:macos`.
- [ ] **mic-grant restart survives a slow boot** â€” with mic permission already granted from a prior session, relaunch with a slow boot (large DB migration or first-run model download) so the window gains focus before `ServerCore` finishes constructing. Verify capture actually starts once boot completes instead of silently staying off. Check `grep "start_capture after mic grant" ~/.MEMORA/MEMORA-app.*.log` â€” should show success, not "gave up after N attempts" (previously a fixed ~10s budget that raced boot and gave up permanently â€” found during the Intel-Mac CI smoke-test investigation, MEMORA#4978). If an attempt does time out, verify a **later window focus retries** rather than being silently disabled for the rest of the session (regression: the old `MIC_FOCUS_CAPTURE_RESTART` latch was set once and never reset).

- [ ] **fresh install â€” all prompts appear** â€” screen recording, microphone, accessibility prompts all show on first launch.
- [ ] **denied permission â†’ opens System Settings** â€” if user previously denied mic permission, clicking "grant" opens System Settings > Privacy directly (`620c89a5`).
- [ ] **permission revoked while running** â€” go to System Settings, revoke screen recording. app shows red permission banner within 10 seconds.
- [ ] **permission banner is visible** â€” solid red `bg-destructive` banner at top of main window when any permission missing. not subtle (`9c0ba5d1`).
- [ ] **permission recovery page** â€” navigating to /permission-recovery shows clear instructions.
- [ ] **startup permission gate** â€” on first launch, permissions are requested before recording starts (`d9d43d31`).
- [ ] **faster permission polling** â€” permission status checked every 5-10 seconds, not 30 (`d9d43d31`).
- [ ] **improved permission recovery UX** â€” Verify that the user experience for recovering from denied permissions is clear and intuitive. (`57cca740`)
- [ ] **Settings returns to normal window level after permission flow** â€” From the full Home/Settings window, open any macOS Privacy & Security pane, return to MEMORA, then focus another app. The other app must be able to appear above MEMORA; only intentional overlay surfaces may remain floating. (`#5753`)

### 8. app lifecycle & updates

commits: `94531265`, `d794176a`, `9070639c`, `0378cab1`, `4a3313d3`, `7ffdd4f1`, `1b36f62d`

- [ ] **clean quit via tray** â€” right-click tray â†’ Quit. all processes terminate. no orphaned ffmpeg/bun processes.
- [ ] **clean quit via dock** â€” right-click dock â†’ Quit. same as above.
- [ ] **clean quit via Cmd+Q** â€” same verification.
- [ ] **force quit recovery** â€” force quit app. relaunch. database is intact. recording resumes.
- [ ] **sleep/wake** â€” close laptop lid, wait 10s, open. recording resumes within 5s. no crash (`9070639c`).
- [ ] **restart app** â€” quit and relaunch. all settings preserved. recording starts automatically.
- [ ] **Cross-platform autorelease pool** â€” Verify that Windows and Linux builds compile and run without issues related to macOS-specific autorelease pool calls. (`851b3037c`)
- [ ] **Main thread safety (macOS)** â€” Verify that tray icon operations, space monitoring, and frontmost app restoration are dispatched to the main thread to prevent crashes. (`ac46aa437`, `418826dfa`, `274826dfa`)
- [ ] **ObjC memory management (macOS)** â€” Verify that all ObjC operations are wrapped in scoped autorelease pools and objects are retained in async callbacks to prevent use-after-free or SIGSEGV crashes. (`4cb9850f7`, `c49350df0`, `139500d52`)
- [ ] **auto-update** â€” when update available, UpdateBanner shows in main window. clicking it downloads and installs.
- [ ] **update without tray** â€” user can update via dock menu "Check for updates" or Apple menu "Check for Updates..." (`d794176a`, `94531265`).
- [ ] **update banner in main window** â€” when update available, banner appears at top of main window.
- [ ] **source build update dialog** â€” source builds show "source build detected" dialog with link to pre-built version.
- [ ] **port conflict on restart** â€” if old process is holding port 3030, new process kills it and starts cleanly (`0378cab1`, `4a3313d3`, `8c435a10`).
- [ ] **no orphaned processes** â€” after quit, `ps aux | grep MEMORA` shows nothing. `lsof -i :3030` shows nothing.
- [ ] **rollback** â€” user can rollback to previous version via tray menu (`c7fbc3ea`).
- [ ] **Zombie CPU drain prevention** â€” Verify that `lsof` calls have a 5-second timeout, preventing zombie CPU drain, especially on quit. Check logs for `lsof` timeouts if applicable.
- [ ] **Tokio shutdown stability** â€” Verify that the `tokio` shutdown process is stable and doesn't panic in the tree walker, especially during application exit or process restarts.
- [ ] **No ggml Metal destructor crash on quit** â€” Perform multiple quick quits (Cmd+Q, tray quit) and restarts. Verify that the app exits cleanly without a `ggml Metal destructor crash`.
- [ ] **Properly wait for UI recorder tasks before exit** â€” During a clean quit, verify that all UI recorder tasks complete properly and no orphaned processes or partial recordings remain.
- [ ] **recording watchdog diagnostics** â€” Verify that the recording watchdog correctly diagnoses and handles recording issues, and provides useful diagnostic information. (`af2b4f3d`)
- [ ] **capture stall detection** â€” Simulate or observe a capture stall. Verify that a notification appears with a "Restart" button to recover. (`d3ead88eb`)
- [ ] **DB write stall detection** â€” if DB writes stall, verify a notification appears with a "Restart" button. (`1b4bf7918`)
- [ ] **clean startup after unclean shutdown on Windows** â€” On Windows, verify that the app starts cleanly after an unclean shutdown (e.g., force quit), without port 3030 binding failures. (`a8413fe2`)
- [ ] **sleep/wake detection on Windows and Linux** â€” Verify that recording resumes correctly after sleep/wake on Windows and Linux. (`f519281b5`)

### 9. database & storage

commits: `eea0c865`, `cc09de61`, `e61501da`, `d25191d7`, `60096fb9`

- [ ] **slow DB insert warning** â€” check logs. "Slow DB batch insert" warnings should be <1s in normal operation. >3s indicates contention.
- [ ] **concurrent DB access** â€” UI queries + recording inserts happening simultaneously. no "database is locked" errors.
- [ ] **store race condition** â€” rapidly toggle settings while recording is active. no crash (`eea0c865`).
- [ ] **event listener race condition** â€” Tauri event listener setup during rapid window creation. no crash (`cc09de61`).
- [ ] **UTF-8 boundary panic** â€” search with special characters, non-ASCII text in OCR results. no panic on string slicing (`eea0c865`).
- [ ] **low disk space** â€” with <1GB free, app should warn user. no crash from failed writes.
- [ ] **opt-in low-disk recording guard** â€” the Storage toggle defaults off and persists across restart. With it off, a low-disk event leaves capture running. With it on, crossing the engine threshold stops the real capture session, leaves `/health` and authenticated search available, and persists the critical in-app notification even when ordinary notifications are disabled. Verify the data volume is selected correctly when `MEMORA_DATA_DIR` is a symlink, junction, or nested mount.
- [ ] **large database (>10GB)** â€” search still returns results within 2 seconds. app doesn't freeze on startup.
- [ ] **Snapshot compaction integrity** â€” Verify compaction doesn't result in NULL offset_index or pool exhaustion. (`09245af5f`)
- [ ] **Audio chunk timestamps** â€” `start_time` and `end_time` are correctly set for reconciled and retranscribed audio chunks in the database.
- [ ] **MEMORA_DATA_DIR usage** â€” Set the `MEMORA_DATA_DIR` environment variable. Verify the app uses this directory for all its data storage. (`d5f30db71`)
- [ ] **DB pool starvation prevention** â€” Simulate high database load (e.g., rapid screen activity, many pipes running) and monitor logs. Verify no "database is locked" errors or signs of DB pool starvation.
- [ ] **write stalls name their own cause** â€” Run `MEMORA db compact` (or any long `VACUUM`) while capture is active. Logs must say "another writer held the SQLite write lock", NOT "pool timed out"; the UI-event recorder must log this at `warn`, not `error`, so a routine VACUUM does not raise a Sentry issue. Starving the write pool instead must say "no write-pool connection became available". The two must never share a message. (`MEMORA-CLI-SQ`)
- [ ] **stuck write coordinator recovers** â€” A process-wide SQLite write permit that is never released must not wedge writes forever. Verify a sustained contention run (past `CONTENTION_PERSISTENT_AFTER_WALL`) marks the write path degraded *and* requests an engine restart, while a normal multi-minute maintenance hold does not. (`MEMORA-CLI-SQ`)
- [ ] **DB write coalescing queue** â€” verify high-frequency captures (e.g. 10 FPS) don't lock the UI or cause write errors. (`c23768f41`)
- [ ] **Multi-byte window titles in suggestions** â€” Interact with suggestions for windows that have multi-byte (e.g., Unicode, emoji) characters in their titles. Verify no char boundary panics.
- [ ] **no concurrent reconciliation issues** â€” Verify that concurrent reconciliation processes do not cause issues during heavy load or sync operations. (`1d436bc3`)
- [ ] **pipe_config blobs skipped in sync** â€” Verify that `pipe_config` blobs are correctly skipped during synchronization, preventing unnecessary data transfer and potential issues. (`08d5c53a`)
- [ ] **Pi's native auto-compaction for pipe session history** â€” Verify that Pi's native auto-compaction feature for pipe session history works as expected, preventing indefinite growth of history and maintaining performance. (`8f49e2cf`)
- [ ] **UTF-8 panic with long multi-byte strings** â€” Introduce long strings with multi-byte UTF-8 characters (e.g., in window titles, chat input, search queries). Verify no panics occur when these strings are truncated, stored, or processed.
- [ ] **fsync snapshots before DB commit** â€” verify data integrity by force-quitting during heavy capture; snapshots should match DB entries. (`2e63282b8`)
- [ ] **Data directory setting location** â€” Verify that the data directory setting is now located in the "Storage" tab of the settings menu. (`0d3ffe30a`)
- [ ] **store.bin encryption** â€” Enable "Encrypt store.bin" in settings (Privacy > Security). Verify that `store.bin` is encrypted and correctly decrypted on startup using the OS keychain. (`143875207`, `aee1cd2b5`, `85ecd7935`)
- [ ] **graceful keychain denial** â€” On macOS, deny keychain access for store encryption. Verify the app handles it gracefully and falls back to unencrypted store if necessary or warns the user. (`b9c01b916`)

- [ ] **slow DB insert warning** â€” check logs. "Slow DB batch insert" warnings should be <1s in normal operation. >3s indicates contention.
- [ ] **concurrent DB access** â€” UI queries + recording inserts happening simultaneously. no "database is locked" errors.
- [ ] **store race condition** â€” rapidly toggle settings while recording is active. no crash (`eea0c865`).
- [ ] **event listener race condition** â€” Tauri event listener setup during rapid window creation. no crash (`cc09de61`).
- [ ] **UTF-8 boundary panic** â€” search with special characters, non-ASCII text in OCR results. no panic on string slicing (`eea0c865`).
- [ ] **low disk space** â€” with <1GB free, app should warn user. no crash from failed writes.
- [ ] **large database (>10GB)** â€” search still returns results within 2 seconds. app doesn't freeze on startup.
- [ ] **Audio chunk timestamps** â€” `start_time` and `end_time` are correctly set for reconciled and retranscribed audio chunks in the database.

### 10. AI presets & settings

commits: `8a5f51dd`, `0b0d8090`, `7e58564e`, `2522a7e2`, `f3e55dbc`, `79f2913f`

- [ ] **Ollama not running** â€” creating an Ollama preset shows free-text input fields (not stuck loading). user can type model name manually (`8a5f51dd`).
- [ ] **custom provider preset** â€” user can add a custom API endpoint. model name is free-text input with optional autocomplete.
- [ ] **settings survive restart** â€” change any setting, quit, relaunch. setting is preserved.
- [ ] **overlay mode switch** â€” change from fullscreen to window mode. setting saves. next shortcut press uses new mode.
- [ ] **FPS setting** â€” change capture FPS. recording interval changes accordingly.
- [ ] **language/OCR engine setting** â€” change OCR language. new language used on next capture cycle.
- [ ] **video quality setting** â€” low/balanced/high/max. affects FFmpeg encoding params (`21bddd0f`).
- [ ] **Settings UI sentence case** â€” All settings UI elements (billing, pipes, team) should use consistent sentence case.
- [ ] **Sidebar text visibility in Auto theme** â€” On macOS, switch between Light, Dark, and Auto system theme modes. Verify that sidebar text remains visible and legible in all modes. (`16d38570d`)
- [ ] **Billing page links to website** â€” Verify that the in-app billing page correctly links to the *new* website billing page.
- [ ] **Non-pro subscriber Whisper fallback** â€” As a non-pro subscriber, verify that audio transcription defaults to `whisper-large-v3-turbo-quantized` and functions correctly.
- [ ] **Pi restart on preset switch** â€” Switch between different AI presets. Verify that the Pi agent restarts if required by the new preset.
- [ ] **Web search disabled for non-cloud providers** â€” When using a non-cloud AI provider, verify that web search functionality is correctly disabled.
- [ ] **Credit balance in billing UI and errors** â€” Verify that the billing UI accurately displays the credit balance and clearly differentiates between `credits_exhausted` and other LLM-related errors.
- [ ] **Unknown AI provider type sanitization** â€” Configure a malformed or unknown AI provider type (e.g., by manual config edit). Verify the app doesn't crash on startup or when navigating to settings, and gracefully handles the unknown type.
- [ ] **standalone settings page** â€” Verify that clicking settings in the tray menu opens a standalone `/settings` page instead of a modal overlay. (`ec2a5789e`)
- [ ] **optional API auth** â€” Enable API auth in settings (or via `--api-auth`). Verify that remote access to the API requires the configured token. (`09f18141a`, `cfc1a74e1`)
- [ ] **privacy settings reordering** â€” Verify that the Security section appears first in the Privacy settings tab. (`4718785b6`)
- [ ] **password field filtering** â€” Verify that password fields are skipped in the accessibility tree and not stored as OCR/text. (`8159641f5`, `d39e42e5b`)
- [ ] **browser extension popup filtering** â€” Verify that browser extension popups (like Bitwarden) are filtered and not captured in the accessibility tree or as black frames. (`52d20987a`, `449ae7a68`, `931db40b6`)

commits: `8a5f51dd`, `0b0d8090`

- [ ] **Ollama not running** â€” creating an Ollama preset shows free-text input fields (not stuck loading). user can type model name manually (`8a5f51dd`).
- [ ] **custom provider preset** â€” user can add a custom API endpoint. model name is free-text input with optional autocomplete.
- [ ] **settings survive restart** â€” change any setting, quit, relaunch. setting is preserved.
- [ ] **overlay mode switch** â€” change from fullscreen to window mode. setting saves. next shortcut press uses new mode.
- [ ] **FPS setting** â€” change capture FPS. recording interval changes accordingly.
- [ ] **language/OCR engine setting** â€” change OCR language. new language used on next capture cycle.
- [ ] **video quality setting** â€” low/balanced/high/max. affects FFmpeg encoding params (`21bddd0f`).
- [ ] **Settings UI sentence case** â€” All settings UI elements (billing, pipes, team) should use consistent sentence case.

### 11. onboarding

commits: `87abb00d`, `9464fdc9`, `0f9e43aa`, `7ea15f32`, `bf1f1004`

- [ ] **fresh install flow** â€” onboarding appears, permissions requested, user completes setup.
- [ ] **auto-advance after engine starts** â€” status screen advances automatically after 15-20 seconds once engine is running (`87abb00d`, `9464fdc9`).
- [ ] **skip onboarding** â€” user can skip and get to main app. settings use defaults.
- [ ] **Onboarding completion destination** â€” After completing onboarding, verify that the home window opens instead of the timeline overlay. (`6ddc33a94`, `3cf668c76`)
- [ ] **shortcut gate** â€” onboarding teaches the shortcut. user must press it to proceed (`0f9e43aa`).
- [ ] **onboarding window size** â€” window is correctly sized, no overflow (`7ea15f32`).
- [ ] **onboarding doesn't re-show** â€” after completing onboarding, restart app. main window shows, not onboarding.
- [ ] **First-run 2-hour reminder notification** â€” On a fresh install, verify that a custom notification panel appears after approximately 2 hours as a first-run reminder.

commits: `87abb00d`, `9464fdc9`, `0f9e43aa`, `7ea15f32`

- [ ] **fresh install flow** â€” onboarding appears, permissions requested, user completes setup.
- [ ] **auto-advance after engine starts** â€” status screen advances automatically after 15-20 seconds once engine is running (`87abb00d`, `9464fdc9`).
- [ ] **skip onboarding** â€” user can skip and get to main app. settings use defaults.
- [ ] **shortcut gate** â€” onboarding teaches the shortcut. user must press it to proceed (`0f9e43aa`).
- [ ] **onboarding window size** â€” window is correctly sized, no overflow (`7ea15f32`).
- [ ] **onboarding doesn't re-show** â€” after completing onboarding, restart app. main window shows, not onboarding.

### 12. timeline & search

commits: `f1255eac`, `25cbdc6b`, `2529367d`, `d9821624`, `e61501da`, `039d5fea`, `50ff4f4c`, `91cc4371`, `bcce42796`, `a98fa2991`, `0ff93b167`, `adbbb8f84`

- [ ] **arrow key navigation** â€” left/right arrow keys navigate timeline frames (`f1255eac`).
- [ ] **search results sorted by time** â€” search results appear in chronological order (`25cbdc6b`).
- [ ] **no frame clearing during navigation** â€” navigating timeline doesn't cause frames to disappear and reload (`2529367d`).
- [ ] **URL detection in frames** â€” URLs visible in screenshots are extracted and shown as clickable pills (`50ef52d1`, `aa992146`).
- [ ] **app context popover** â€” clicking app icon in timeline shows context (time, windows, urls, audio) (`be3ecffb`).
- [ ] **Timeline single "current" bar** â€” Verify that the timeline only shows one "current time" bar, even during rapid updates. (`bcce42796`)
- [ ] **Timeline "Calls" filter** â€” Verify the "Calls" filter on the timeline correctly filters for call-related events. (`0ff93b167`)
- [ ] **Collapsible timeline filters** â€” Verify that timeline filters can be collapsed and expanded correctly. (`0ff93b167`)
- [ ] **window-focused refresh** â€” opening app via shortcut/tray refreshes timeline data immediately (`0b057046`).
- [ ] **code block colors in memories** â€” Verify that code block colors in the memories page match the current app theme. (`1c8d785fc`)
- [ ] **memories page pagination** â€” Verify that memories page pagination works correctly and tags are loaded from the API. (`3e00b70b4`)
- [ ] **frame deep link navigation** â€” `MEMORA://frame/N` or `MEMORA://frames/N` opens main window and jumps to frame N. works from cold start; invalid IDs show clear error.
- [ ] **missing frames return 404** â€” Attempt to access a non-existent frame via the API. Verify that it returns a 404 error. (`2e63282b8`)
- [ ] **Search result exact navigation** â€” Click a search result. Verify it navigates exactly to the associated `frame_id`. (`a98fa2991`)
- [ ] **Search navigation persistence** â€” Navigate to a frame from search results. Shift focus away from the app and back. Verify the navigation is not reset. (`71dee4ca3`)
- [ ] **Search navigation race condition** â€” Verify that search navigation works reliably even if the webview is still mounting (retries should handle it). (`2015137a1`)
- [ ] **Consolidated text search** â€” Perform keyword searches. Verify results are correctly pulled from the consolidated `frames.full_text` and `frames_fts`. (`adbbb8f84`)
- [ ] **Keyword search accessibility** â€” Keyword search should find content within accessibility-only frames and utilize `frames_fts` for comprehensive accessibility text searching.
- [ ] **Keyword search logic** â€” Verify that keyword search SQL correctly uses `OR` instead of `UNION` within `IN()`.
- [ ] **Search prompt accuracy** â€” Verify that search prompts are improved to prevent false negatives from over-filtering.
- [ ] **Past-day timeline navigation** â€” Navigate the timeline to past days (e.g., using date picker or arrow keys). Verify that data loads correctly and the timeline behaves as expected.
- [ ] **`content_type=all` search and pagination** â€” Perform search queries with `content_type=all`. Verify that the result count is accurate and pagination works correctly without missing or duplicating results.
- [ ] **Search pagination with offset** â€” Perform paginated searches, particularly beyond the first page. Verify that results are not empty or incorrect due to double-applied offsets.
- [ ] **`search_ocr()` returns results for event-driven capture** â€” Verify that `search_ocr()` correctly returns OCR results for event-driven captures and does not return empty when visible text is present on screen.
- [ ] **timeline displays consistent timestamps** â€” Verify that the timeline displays consistent timestamps, regardless of locale settings, and that there are no timestamp localization issues via websocket. (`2cf0c14e`)
- [ ] **timeline retry backoff mechanism** â€” Verify that the timeline's retry backoff mechanism functions as expected for data loading, ensuring resilience during temporary data unavailability. (`57cca740`)
- [ ] **arrow key navigation between search results in timeline** â€” Verify that left/right arrow keys correctly navigate between search results within the timeline view. (`3e8f37fc`)
- [ ] **URL chips always shown when detected** â€” Verify that URL chips are always displayed in the UI when URLs are detected in the content. (`cba69e56`)
- [ ] **refresh button inline with suggestion chips (icon-only)** â€” Verify that the refresh button for suggestion chips is displayed inline with the chips and is icon-only. (`a80e9ce6`)
- [ ] **bottom suggestion chips hidden on empty chat** â€” Verify that bottom suggestion chips are hidden when the chat is empty to avoid duplication. (`d6c4b821`)
- [ ] **Refresh button for suggestion chips** â€” A refresh button appears on bottom suggestion chips. Clicking it updates suggestions.
- [ ] **Timeline refresh button hover** â€” verify cursor-pointer and hover state on timeline refresh button. (`0cee47b62`)
- [ ] **Smarter idle suggestions** â€” Verify that "idle suggestions" appear and are contextually relevant when the user is inactive.
- [ ] **Hide suggestion chips on empty chat** â€” Verify that suggestion chips are hidden when the chat is empty to prevent duplication.
- [ ] **Text selection not blocked by URL overlays** â€” On URL-heavy pages, verify that text selection is not blocked by clickable URL overlays.
- [ ] **AI suggestion chip refresh and animations** â€” Verify a refresh button exists on AI suggestion chips, and appropriate animations (e.g., loading spinner) are shown when refreshing.
- [ ] **Activity summary time measurement and relative parsing** â€” Verify activity summaries display accurate time measurements and relative time parsing (e.g., "5 minutes ago", "yesterday") works correctly in the UI.
- [ ] **Hybrid OCR for canvas apps** â€” Verify that text from Google Docs and Figma (canvas-rendered) is captured using hybrid OCR. (`4d2b05990`, `f09f1e9aa`)
- [ ] **Search modal scroll** â€” Verify that the search modal is scrollable on Windows/Linux embedded timeline and trackpad/wheel scrolling works. (`f108f1f0d`, `2a2bd9b5`, `5762c60bf`)
- [ ] **Modal scrolling (general)** â€” Verify that all modals (e.g., settings, pipes, search) are scrollable and handle overflow correctly, especially on Windows and Linux. (`19789657d`)
- [ ] **Search modal UX** â€” Verify that click interference from Live Text and wheel handlers is resolved, and app/date filter timezone bugs are fixed. (`0c883819e`, `b7123231`, `f09f1e9aa`)
- [ ] **Timeline filter viewport scoping** â€” verify timeline filters apply to current viewport, not a fixed 800-frame window. (`9277431e4`)
- [ ] **Chat UI code blocks** â€” verify light text on dark bg in chat code blocks. (`c029f7779`)
- [ ] **Chat image viewer** â€” verify images can be viewed in chat. (`2bcdf8d8b`)
- [ ] **Chat preset dropdown** â€” verify AI preset switching within chat. (`2bcdf8d8b`)
- [ ] **Memories Settings UI** â€” verify frame_id relationship and Memories settings work as expected. (`67f4c4304`)

commits: `f1255eac`, `25cbdc6b`, `2529367d`, `d9821624`

- [ ] **arrow key navigation** â€” left/right arrow keys navigate timeline frames (`f1255eac`).
- [ ] **search results sorted by time** â€” search results appear in chronological order (`25cbdc6b`).
- [ ] **no frame clearing during navigation** â€” navigating timeline doesn't cause frames to disappear and reload (`2529367d`).
- [ ] **URL detection in frames** â€” URLs visible in screenshots are extracted and shown as clickable pills (`50ef52d1`, `aa992146`).
- [ ] **app context popover** â€” clicking app icon in timeline shows context (time, windows, urls, audio) (`be3ecffb`).
- [ ] **window-focused refresh** â€” opening app via shortcut/tray refreshes timeline data immediately (`0b057046`).
- [ ] **frame deep link navigation** â€” `MEMORA://frame/N` or `MEMORA://frames/N` opens main window and jumps to frame N. works from cold start; invalid IDs show clear error.
- [ ] **Keyword search accessibility** â€” Keyword search should find content within accessibility-only frames and utilize `frames_fts` for comprehensive accessibility text searching.
- [ ] **Keyword search logic** â€” Verify that keyword search SQL correctly uses `OR` instead of `UNION` within `IN()`.
- [ ] **Search prompt accuracy** â€” Verify that search prompts are improved to prevent false negatives from over-filtering.

### 13. sync & cloud

- [ ] **CLI remote sync** â€” Run `MEMORA sync remote`. Verify it correctly syncs data to a remote SSH/SFTP server. (`f46e85cb1`)


commits: `2f6b2af5`, `ea7f1f61`, `5cb100ea`

- [ ] **auto-remember sync password** â€” user doesn't have to re-enter password each time (`5cb100ea`).
- [ ] **auto-download from other devices** â€” after upload cycle, download new data from paired devices (`2f6b2af5`).
- [ ] **auto-init doesn't loop** â€” sync initialization happens once, doesn't repeat endlessly (`ea7f1f61`).
- [ ] **Cloud archive docs** â€” Verify that the cloud archive documentation page exists and is accessible via a link from settings.
- [ ] **simplified Arc URL extraction** â€” Verify that simplified Arc URL extraction works correctly, capturing URLs from Arc browser content. (`08d5c53a`)
- [ ] **Randomly generated cloud sync password** â€” On new sync setup, verify that a randomly generated cloud sync password is used.
- [ ] **Trialing subscriptions for pipe sync** â€” With a trialing subscription, verify that pipe sync functions as if the subscription is active, and pipes sync correctly.
- [ ] **Encrypted pipe sync (Pro) and locked toggle (non-Pro)** â€” As a Pro user, enable encrypted pipe sync and verify pipes sync encrypted. As a non-Pro user, verify the encrypted pipe sync toggle is locked and inaccessible.
- [ ] **Arc URL extraction and pipe_config blobs** â€” If Arc Browser is supported, verify accurate URL extraction. Verify that `pipe_config` blobs are correctly skipped during sync (requires inspection of sync data or logs).
- [ ] **Per-device record counts in sync** â€” In sync settings, verify that record counts are displayed for each synchronized device and that sync configuration persists across restarts. (`0e7baaedb`)
- [ ] **transcription daily cost cap** â€” Verify that the daily cost cap for transcription is correctly enforced and prevents further transcription once reached. (`2f67a1041`)
- [ ] **local Google Calendar OAuth** â€” Connect Google Calendar. Verify it uses the local OAuth flow instead of a cloud-based one. (`0177fdf2b`)

### 14. Region OCR (Shift+Drag)

commits: `b3628788`, `738178da`

- [ ] **Shift+Drag region OCR functionality** â€” Perform a `Shift+Drag` region OCR selection on the screen. Verify that the RegionOcrOverlay appears correctly and local OCR processes the selected region.
- [ ] **Local OCR without login for Shift+Drag** â€” Verify that the `Shift+Drag` region OCR uses local OCR and functions correctly without requiring the user to be logged in or have a cloud subscription.

### 15. Windows-specific

commits: `eea0c865`, `fe9060db`, `c99c3967`, `aeaa446b`, `5a219688`, `caae1ebc`, `67caf1d1`, `ff4af7b5`

- [ ] **pre-AVX / non-AVX2 CPU launch (compatibility mode)** â€” (#3125) On x86-64 CPUs without AVX (Atom-line Celeron/Pentium such as the N5095 = Jasper Lake/Tremont, pre-2011 Intel, QEMU `-cpu qemu64`), the app and CLI must LAUNCH and RECORD instead of dying with `0xC000001D STATUS_ILLEGAL_INSTRUCTION`. **Test on a NO-AVX CPU, not merely a no-AVX2 one**: measured on real Windows guests, the pre-fix build ran `record` for 420s under an emulated Sandy Bridge (which has AVX) and died in 1.4s under Westmere (no AVX) with `Exception code: 0xc000001d`, faulting module `MEMORA.exe`. `/arch:AVX2` codegen emits VEX-encoded AVX instructions, so a Sandy-Bridge-class probe passes builds that crash in the field. Local whisper/qwen3 STT is disabled at runtime (AVX2-compiled kernels); parakeet + cloud engines keep working; onboarding shows a "compatibility mode" notice. Guards: never set global `CFLAGS/CXXFLAGS=/arch:AVX2` in Windows release workflows (it poisons every cc/cmake dep incl. bundled SQLite â†’ crash in static initializers before main); ONNX Runtime must stay on the MS DirectML DLL (import-table link, runtime CPUID dispatch) â€” pyke's `download-binaries` static libs are AVX2-only. CI: `release-cli.yml`'s `smoke-windows` job (runs-on windows-latest â€” SDE's Pin injector is broken on windows-2022, and every SDE 10.x kit fails injection on current hosted images; 9.58 works) runs the packaged exe AND an ONNX-init probe (`.github/ci/ort-smoke`, real `Session::builder()` against the shipped DirectML DLL) under Intel SDE `-wsm -chip_check_exe_only` on every Windows CLI build, and `release`/`publish-npm` require it; `release-app.yml` probes the app exe natively via the hidden `--cpu-smoke` flag (static init + import table). `-chip_check_exe_only` is what makes pre-AVX emulation usable â€” it scopes the chip check to the main image so the host's own AVX-executing system DLLs can't cause false failures. Reproduce locally with `sde -wsm -chip_check_exe_only -- MEMORA.exe --version` (pre-fix build faults on `vpxor`; fixed build prints its version), or QEMU `-cpu Westmere`/`-cpu qemu64` for a full guest. Pipes/AI chat on such CPUs use bun's official `windows-x64-baseline` build (runtime-downloaded, mandatory SHA-256; verified 2026-07-31 to run under `sde -snb` while stock bun faults).
- [ ] **COM thread conflict** â€” audio and vision threads don't conflict on COM initialization (`eea0c865`).
- [ ] **high-DPI display (150%, 200%)** â€” OCR captures at correct resolution.
- [ ] **multiple monitors** â€” all detected and recorded.
- [ ] **Windows Defender** â€” app not blocked by default security.
- [ ] **Windows default mode** â€” On Windows, the app should default to window mode on first launch.
- [ ] **Windows taskbar icon** â€” The app should display a taskbar icon on Windows.
- [ ] **Enterprise enforced auto-start enrollment** â€” In an installed Windows enterprise build, set `lockedSettings.autoStartEnabled = "true"`, disable MEMORA in Task Manager Startup apps, and verify both the HKCU `Run` command (including `--autostart`) and `StartupApproved\Run` return to enabled within two seconds. Sign out/in and confirm MEMORA starts in the background. Remove the policy, disable startup again, wait at least two reconciliation intervals, and verify it remains disabled.
- [ ] **Windows login window resets stale OAuth navigation** â€” Click login, choose GitHub or Google, leave the provider flow open, then click login in MEMORA again. The existing "sign in to MEMORA" window must return to the MEMORA login page instead of showing the stale provider page or a blank document.
- [ ] **Windows audio transcription accuracy** â€” On Windows, verify improved audio transcription accuracy due to native Silero VAD frame size and lower speech threshold.
- [ ] **Windows multi-line pipe prompts** â€” Multi-line pipe prompts should be preserved on Windows.
- [ ] **Windows ARM64 support** â€” On a Windows ARM64 device, verify the app installs and runs correctly. (`d62360bc4`)
- [ ] **Windows app matching for meetings** â€” On Windows, verify that meeting detection correctly matches active applications. (`ef39e728d`)
- [ ] **Alt+S shortcut activates overlay with keyboard focus** â€” On Windows, press `Alt+S`. Verify that the overlay window appears and immediately receives keyboard focus, allowing immediate typing.
- [ ] **OcrTextBlock deserialization handles Windows OCR format** â€” On Windows, verify that `OcrTextBlock` deserialization correctly handles the specific Windows OCR format. (`c49ccb55`)
- [ ] **populate accessibility tree bounds for text overlay on Windows** â€” On Windows, verify that accessibility tree bounds are correctly populated for text overlay, ensuring accurate positioning and interaction. (`4d20803a`)
- [ ] **capture full accessibility tree for Chromium/Electron apps on Windows** â€” On Windows, verify that the full accessibility tree is captured for Chromium/Electron applications. (`2e50c772`)
- [ ] **Accessibility tree bounds for text overlay** â€” On Windows, verify that text overlays accurately reflect the accessibility tree bounds, making selection and interaction precise.
- [ ] **No console flash during GPU detection** â€” On Windows startup, verify that no temporary console window flashes during the GPU detection process. (`a0aba1643`)
- [ ] **Filter noisy system apps** â€” On Windows, verify that noisy system apps are filtered out from screen capture and do not appear in the timeline or search results.
- [ ] **Settings window instead of overlay** â€” On Windows, verify that the Settings window is used instead of the overlay for settings, and the shortcut toggle works correctly. (`c13e21b55`)
- [ ] **background work is deprioritized on Windows; app process and UI stay at Normal** â€” (#4849) CLI `MEMORA record` drops itself to BELOW_NORMAL_PRIORITY_CLASS (verify `(Get-Process MEMORA).PriorityClass` â†’ `BelowNormal`, CI asserts in `windows-integration-test.yml`; opt-out `MEMORA_KEEP_NORMAL_PRIORITY=1`). The desktop app process must stay `Normal` â€” it gets targeted lowering only: audio-encode ffmpeg children spawn `BelowNormal`, compaction ffmpeg stays `Idle`, pipe/agent bun subtree spawns `BelowNormal` (interactive chat's pi sidecar stays `Normal` â€” separate spawner in src-tauri/pi.rs), whisper inference dips its thread per-call, LL input-hook threads pinned HIGHEST (system-wide input path â€” Windows silently unhooks starved LL hooks). Verify children: `Get-CimInstance Win32_Process -Filter "Name='ffmpeg.exe' OR Name='bun.exe'"` + `(Get-Process -Id â€¦).PriorityClass` while recording with a pipe enabled. If audio capture gaps appear under all-core CPU load in the CLI, see #4916 (cpal fork's broken thread-priority boost â€” capture threads not insulated from the class drop).
- [ ] **foreground apps stay responsive while a11y capture runs (UIA tree walks off by default)** â€” Full-window UIA tree walks are synchronous cross-process calls serviced on the *target app's* UI thread. The old 2s periodic + focus-change walks caused p95 213ms message-pump stalls (~97 freezes >100ms per 2 min) in Edge and were ~100% of severe foreground freezes; their snapshots also had zero consumers (`paired_capture.rs` owns a11y text capture). `capture_tree`/`tree_capture_interval_ms` now default to `false`/`0` (`crates/MEMORA-a11y/src/config.rs`), including the startup walk. Preserved: per-click `ElementFromPoint` enrichment and a debounced single-element focused-element refresh on focus change (feeds click/app_switch/window_focus element context). Verify: (a) `scripts/windows/cpu-investigation/walker-fix-verification/verify-walker-fix.ps1` (self-contained before/after harness with PASS/FAIL gate; measured p95 156ms â†’ 1.6ms, stalls>100ms 108 â†’ 4 per 2min) or the raw probe `scripts/windows/cpu-investigation/experiments/measure-ui-latency.ps1`; (b) `ui_events` rows still carry `element` context for clicks and app switches (`cargo run -p MEMORA-a11y --example windows_probe_harness`); (c) anyone re-enabling `capture_tree` must keep the startup walk gated on it.

commits: `eea0c865`, `fe9060db`, `c99c3967`, `aeaa446b`, `5a219688`, `caae1ebc`, `67caf1d1`

- [ ] **COM thread conflict** â€” audio and vision threads don't conflict on COM initialization (`eea0c865`).
- [ ] **high-DPI display (150%, 200%)** â€” OCR captures at correct resolution.
- [ ] **multiple monitors** â€” all detected and recorded.
- [ ] **Windows Defender** â€” app not blocked by default security.
- [ ] **Windows default mode** â€” On Windows, the app should default to window mode on first launch.
- [ ] **Windows taskbar icon** â€” The app should display a taskbar icon on Windows.
- [ ] **Windows audio transcription accuracy** â€” On Windows, verify improved audio transcription accuracy due to native Silero VAD frame size and lower speech threshold.
- [ ] **Windows multi-line pipe prompts** â€” Multi-line pipe prompts should be preserved on Windows.

#### Windows text extraction matrix (accessibility vs OCR)

The event-driven pipeline (`paired_capture.rs`) decides per-frame whether to use accessibility tree text or OCR. Terminal apps force OCR because their accessibility tree only returns window chrome.

commits: `5a219688` (wire up Windows OCR), `caae1ebc` (prefer OCR for terminals), `67caf1d1` (no chrome fallback)

**App categories and expected behavior:**

| App category | Examples | `app_prefers_ocr` | Text source | Expected text |
|---|---|---|---|---|
| Browser | Chrome, Edge, Firefox | false | Accessibility | Full page content + chrome |
| Code editor | VS Code, Fleet | false | Accessibility | Editor content, tabs, sidebar |
| Terminal (listed) | WezTerm, Windows Terminal, Alacritty | true | Windows OCR | Terminal buffer content via screenshot |
| Terminal (unlisted) | cmd.exe, powershell.exe | false | Accessibility | Whatever UIA exposes (may be limited) |
| System UI | Explorer, taskbar, Settings | false | Accessibility | UI labels, text fields |
| Games / low-a11y apps | Games, Electron w/o a11y | false | Windows OCR (fallback) | OCR from screenshot |
| Lock screen | LockApp.exe | false | Accessibility | Time, date, battery |

**Terminal detection list** (`app_prefers_ocr` matches, case-insensitive):
`wezterm`, `iterm`, `terminal`, `alacritty`, `kitty`, `hyper`, `warp`, `ghostty`

Note: `"terminal"` matches `WindowsTerminal.exe` but NOT `cmd.exe` or `powershell.exe`.

**Test checklist:**

- [ ] **WezTerm OCR capture** â€” open WezTerm, type commands. search for terminal content within 30s. should return OCR text, NOT "System Minimize Restore Close" chrome.
- [ ] **Windows Terminal OCR** â€” same test with Windows Terminal.
- [ ] **Chrome/Edge full accessibility** â€” open Chrome or Edge, browse a page. search returns full page content from accessibility tree, not just limited UI elements.
- [ ] **VS Code full accessibility** â€” open VS Code with a file. search returns full code content and UI elements from accessibility tree.
- [ ] **Game/no-a11y OCR fallback** â€” open an app with poor accessibility. OCR should run and extract text from screenshot.
- [ ] **OCR engine name** â€” query DB: OCR entries should have engine `WindowsNative` (not `AppleNative`).
- [ ] **Failed OCR = no noise** â€” if OCR fails for a terminal, the frame should have NULL text, not chrome like "System Minimize Restore Close".
- [ ] **Non-terminal chrome-only** â€” rare case where a normal app returns only chrome from accessibility. stored as-is (acceptable, no OCR fallback triggered).
- [ ] **Empty accessibility + empty OCR** â€” app with no tree text and OCR failure. frame stored with NULL text. no crash.
- [ ] **OCR text persisted on frames** â€” `SELECT COUNT(*) FROM frames WHERE full_text IS NOT NULL AND full_text != ''` should be non-zero after a few minutes of use on Windows. (The `ocr_text` table was retired; OCR text now lives in `frames.full_text`, per-word boxes in `frames.text_json`.)

#### Windows text extraction â€” untested / unknown apps

These apps are common on Windows but have **never been tested** with the event-driven pipeline. We don't know if their accessibility tree returns useful text or just chrome. Each needs manual verification: open the app, use it for a few minutes, then `curl "http://localhost:3030/search?app_name=<name>&limit=3"` and check if the text is meaningful.

**Status legend:** `?` = untested, `OK` = verified good, `CHROME` = only returns chrome, `EMPTY` = no text, `OCR-NEEDED` = should be added to `app_prefers_ocr`

| App | Status | a11y text quality | Notes |
|---|---|---|---|
| **Browsers** | | | |
| Chrome | OK | good (full page content) | 2778ch avg, rich a11y tree |
| Edge | ? | probably good | same Chromium UIA as Chrome |
| Firefox | ? | unknown | different a11y engine than Chromium |
| Brave / Vivaldi / Arc | ? | probably good | Chromium-based, needs verification |
| **Code editors** | | | |
| VS Code | ? | unknown | Electron, should have good UIA |
| JetBrains (IntelliJ, etc) | ? | unknown | Java Swing/AWT, UIA quality varies |
| Sublime Text | ? | unknown | custom UI, may need OCR fallback |
| Cursor | ? | unknown | Electron fork of VS Code |
| Zed | ? | unknown | custom GPU renderer, a11y unknown |
| **Terminals** | | | |
| WezTerm | CHROME | chrome only ("System Minimize...") | `app_prefers_ocr` = true, OCR works |
| Windows Terminal | ? | unknown | matches `"terminal"` in `app_prefers_ocr` |
| cmd.exe | ? | unknown | NOT matched by `app_prefers_ocr` |
| powershell.exe | ? | unknown | NOT matched by `app_prefers_ocr` |
| Git Bash (mintty) | ? | unknown | NOT matched by `app_prefers_ocr` |
| **Communication** | | | |
| Discord | ? | unknown | Electron, old OCR data exists |
| Slack | ? | unknown | Electron |
| Teams | ? | unknown | Electron/WebView2 |
| Zoom | ? | unknown | custom UI |
| Telegram | ? | unknown | Qt-based |
| WhatsApp | ? | unknown | Electron |
| **Productivity** | | | |
| Notion | ? | unknown | Electron |
| Obsidian | ? | unknown | Electron |
| Word / Excel / PowerPoint | ? | unknown | native Win32, historically good UIA |
| Outlook | ? | unknown | mixed native/web |
| OneNote | ? | unknown | UWP, should have good UIA |
| **Media / Creative** | | | |
| Figma | ? | unknown | Electron + canvas, likely poor a11y on canvas |
| Spotify | ? | unknown | Electron/CEF |
| VLC | ? | unknown | Qt-based |
| Adobe apps (Photoshop, etc) | ? | unknown | custom UI, historically poor a11y |
| **System / Utilities** | | | |
| Explorer | OK | good | file names, paths, status bar |
| Settings | ? | unknown | UWP, should be good |
| Task Manager | ? | unknown | UWP on Win11 |
| Notepad | ? | unknown | should have excellent UIA |
| **Games / GPU-rendered** | | | |
| Any game | ? | likely empty | GPU-rendered, no UIA tree. should fall to OCR |
| Electron w/ disabled a11y | ? | likely empty | some Electron apps disable a11y |

**Priority to test (most common user apps):**
1. VS Code â€” most developers will have this open
2. Discord / Slack â€” always running in background
3. Windows Terminal / cmd.exe / powershell.exe â€” verify terminal detection
4. Edge / Firefox â€” browser is primary use
5. Notion / Obsidian â€” knowledge workers
6. Office apps â€” enterprise users

**How to verify an app:**
```bash
# 1. Open the app, use it for 2 minutes
# 2. Check what was captured:
curl "http://localhost:3030/search?app_name=<exe_name>&limit=3&content_type=all"
# 3. If text is only chrome (System/Minimize/Close), it may need adding to app_prefers_ocr
# 4. If text is empty and screenshots exist, OCR fallback should kick in
# 5. Update this table with findings
```

**Apps that may need adding to `app_prefers_ocr` list:**
- If cmd.exe / powershell.exe return chrome-only text, add `"cmd"` and `"powershell"` to the list
- If mintty (Git Bash) returns chrome-only, add `"mintty"`
- Any app where the accessibility tree consistently returns only window chrome but screenshots contain readable text

### 15. Help and Support

commits: `deac5ea9`

- [ ] **Intercom integration in help section** â€” Navigate to the desktop app's help section. Verify that Crisp is replaced by Intercom and that the Intercom chat widget and knowledge base search function as expected.

### 16. CI / release

commits: `8f334c0a`, `fda40d2c`

- [ ] **macOS 26 runner** â€” release builds on self-hosted macOS 26 runner (`fda40d2c`).
- [ ] **updater artifacts** â€” release includes `.tar.gz` + `.sig` for macOS, `.nsis.zip` + `.sig` for Windows.
- [ ] **prod config used** â€” CI copies `tauri.prod.conf.json` to `tauri.conf.json` before building. identifier is `github.com/shrirampai3000/memora` not `github.com/shrirampai3000/memora.dev`.
- [ ] **draft then publish** â€” `workflow_dispatch` creates draft. manual publish or `release-app-publish` commit publishes.
- [ ] **macOS E2E workflow** â€” Verify the macOS E2E workflow in CI ensures stability across releases. (`18ca34d55`)

### 16. MCP / Claude integration

commits: `8c8c445c`

- [ ] **Claude connect button works** â€” Settings â†’ Connections â†’ "Connect Claude" downloads `.mcpb` file and opens it in Claude Desktop. was broken because GitHub releases API pagination didn't reach `mcp-v*` releases buried behind 30+ app releases (`8c8c445c`).
- [ ] **MCP release discovery with many app releases** â€” `getLatestMcpRelease()` paginates up to 5 pages (250 releases) to find `mcp-v*` tagged releases. verify it works even when >30 app releases exist since last MCP release.
- [ ] **Claude Desktop not installed** â€” clicking connect shows a useful error, not a silent failure.
- [ ] **MCP version display** â€” Settings shows the available MCP version and whether it's already installed.
- [ ] **macOS Claude install flow** â€” downloads `.mcpb`, opens Claude Desktop, waits 1.5s, then opens the `.mcpb` file to trigger Claude's install modal.
- [ ] **Windows Claude install flow** â€” same flow using `cmd /c start` instead of `open -a`.
- [ ] **download error logging** â€” if download fails, console shows actual error message (not `{}`).

### 17. AI Agents / Pipes

commits: `fa887407`, `815f52e6`, `60840155`, `e66c3ff8`, `c905ffbf`, `01147096`, `5908d7f4`, `46422869`, `4f43da70`, `71a1a537`, `6abaaa36`, `f3e55dbc`, `8e426dec`, `1289f51e`, `4bc9ff1a`, `c336f73d`, `2f7416ae`

- [ ] **Pi process stability** â€” After app launch, `ps aux | grep pi` should show a single, stable `pi` process that doesn't restart or get killed.
- [ ] **Pi readiness handshake** â€” First chat interaction with Pi should be fast (<2s for readiness).
- [ ] **Pi auto-recovery** â€” If the `pi` process is manually killed, it should restart automatically within a few seconds and be ready for chat.
- [ ] **Pipe output accuracy** â€” When executing a pipe, the user's prompt should be accurately reflected in the output.
- [ ] **Silent LLM errors** â€” LLM errors during pipe execution should be displayed to the user, not silently suppressed.
- [ ] **Fast first chat with Pi** â€” The first interaction with Pi after app launch should be responsive, with no noticeable delay (aim for <2s).
- [ ] **Activity Summary tool** â€” MCP can access activity summaries via the `activity-summary` tool, and the `activity-summary` endpoint works correctly.
- [ ] **Search Elements tool** â€” MCP can search elements using the `search-elements` tool.
- [ ] **Frame Context tool** â€” MCP can access frame context via the `frame-context` tool.
- [ ] **Progressive disclosure for AI data** â€” AI data querying should progressively disclose information.
- [ ] **MEMORA Analytics skill** â€” The `MEMORA-analytics` skill can be used by the Pi agent to perform raw SQL usage analytics.
- [ ] **MEMORA Retranscribe skill** â€” The `MEMORA-retranscribe` skill can be used by the Pi agent for retranscription.
- [ ] **AI preset save stability** â€” Saving AI presets should not cause crashes, especially when dealing with pipe session conflicts.
- [ ] **Pipe token handling** â€” Ensure that Pi configuration for pipes uses the actual token value, not the environment variable name.
- [ ] **Pipe user_token passthrough** â€” Verify that the `user_token` is correctly passed to Pi pre-configuration so pipes use the MEMORA provider.
- [ ] **Pipe preset override** â€” Install a pipe from the store. Verify its preset can be overridden by user's default. (`bee49f1e7`)
- [ ] **Pipe configurable timeout** â€” Add `timeout` to pipe.md frontmatter. Verify pipe respects this timeout. (`cc0ecef53`)
- [ ] **Pipe store caching** â€” Navigate pipe store and connections pages. Verify fast loading due to client-side caching. (`f501c19fb`)
- [ ] **Primary + fallback AI preset UI** â€” Verify the UI for primary and fallback AI presets for pipes works as expected. (`da206471a`)
- [ ] **Default AI model ID** â€” Verify that the default AI model ID does not contain outdated date suffixes.
- [ ] **Move provider/model flags** â€” `--provider` and `--model` flags should be correctly moved before `-p prompt` in `pi spawn` commands.
- [ ] **Pi restart on preset switch** â€” Switch between different AI presets. Verify that the Pi agent restarts if required by the new preset.
- [ ] **Faster Pipes page loading** â€” Verify that the "Pipes" page loads significantly faster, especially when there are a large number of pipes configured.
- [ ] **Instant pipe enable toggle UI update** â€” Toggle a pipe's enable status. Verify that the UI updates instantly due to optimistic updates, even if the backend operation takes a moment.
- [ ] **Pipe execution shows parsed text** â€” Execute a pipe that outputs JSON. Verify that the output displayed to the user is correctly parsed text, not raw JSON.
- [ ] **Surface LLM errors in chat UI** â€” Interact with the chat UI using an AI provider under conditions that would cause LLM errors (e.g., exhausted credits, rate limits). Verify these errors are clearly surfaced to the user.
- [ ] **Pipe preset bug fixes and credit drain prevention** â€” Thoroughly test creating, editing, and switching pipe presets to ensure no bugs, especially those that might lead to unexpected cloud credit usage or misconfiguration.
- [ ] **pipe UI improvements** â€” Verify the overall improvements to the Pipes UI, ensuring a better user experience. (`2e68400c`)
- [ ] **proper spinner icon for pipe refresh button** â€” Verify that the pipe refresh button displays the correct spinner icon during loading states. (`b709af2f`)
- [ ] **ChatGPT OAuth provider in pipes** â€” Configure ChatGPT OAuth provider. Verify that pipes using ChatGPT work correctly.
- [ ] **Reduced excessive Pi restarts** â€” When changing AI preset values or other settings, verify that excessive Pi restarts are reduced. Monitor logs for unnecessary restart messages.
- [ ] **Invalid UTF-8 in Pi streaming** â€” Execute a pipe that outputs invalid UTF-8 characters to stdout/stderr. Verify that Pi streaming correctly handles these without crashing or displaying garbled output.
- [ ] **Auto-abort stuck Pi agent** â€” Verify that the Pi agent is auto-aborted if stuck before sending a new message. (`602419151`)
- [ ] **Pi crash loop fix (Windows)** â€” Verify that the Pi agent doesn't enter a crash loop on Windows due to lru-cache interop issues. (`de56176e5`)
- [ ] **Token counter** â€” Verify that the chat UI displays a token counter. (`2f75e90bf`)
- [ ] **Optimize button** â€” Verify that the "optimize" button appears in the pipe dropdown menu. (`5dff9d21a`)
- [ ] **Pipes as App Store** â€” Verify the redesigned Pipes tab, which provides a unified app store experience. (`89d2e0129`)
- [ ] **Tool call UI with progress rail** â€” Execute a pipe that uses tool calls. Verify the redesigned UI featuring a progress rail timeline and auto-collapse for friendly interaction. (`6c23e1399`, `d81ea65c1`)
- [ ] **In-app Notification Panel** â€” Use the `/notify` API (e.g., via a pipe). Verify an in-app notification panel appears instead of a system notification. (`34937b2dc`)
- [ ] **Pipe store stability** â€” verify null guards, sharp corners, unpublish functionality, and data unwrap fixes. (`603c84f7b`)
- [ ] **Pi agent & search timeouts** â€” Run a long-running search or Pi agent task. Verify it doesn't timeout prematurely at 60s (should allow up to 120s for search). (`f01213cf5`)
- [ ] **allow schedule + triggers together in pipe config UI** â€” Verify that both schedule and triggers can be configured together in the pipe config UI without conflicts. (`f32bf9230`)
- [ ] **local event triggers for pipes** â€” Verify that pipes can be triggered by local events such as meeting start/end and pipe chaining. (`508b68ff7`, `776587aa7`)
- [ ] **one event-triggered run per meeting-end generation** â€” With meeting-summary enabled, join a call, leave, rejoin the same app within 120s (this reopens the same meeting row), then leave again. Duplicate delivery of either persisted end timestamp must be suppressed, but the final end after resume must create a second execution for the same meeting id so it summarizes the full transcript. Retranscribing saved audio must create one additional generation only when replacement segments were committed. verify: `grep "suppressed duplicate 'meeting_ended'" ~/.MEMORA/MEMORA-app.*.log` and `sqlite3 ~/.MEMORA/db.sqlite "SELECT event_key FROM pipe_event_runs WHERE pipe_name='meeting-summary' AND event_name='meeting_ended' ORDER BY claimed_at"` shows distinct `<meeting-id>@<generation>` keys; `sqlite3 ~/.MEMORA/db.sqlite "SELECT id, trigger_event, trigger_key FROM pipe_executions WHERE pipe_name='meeting-summary' ORDER BY id DESC LIMIT 5"` keeps the user-facing `trigger_key` equal to the meeting id. (#5481)
- [ ] **failed event run retries** â€” Force an event-triggered run to fail (e.g. sign out of the preset's provider mid-run). Its claim must be released so the next delivery of that event runs. verify: `sqlite3 ~/.MEMORA/db.sqlite "SELECT * FROM pipe_event_runs"` no longer holds the failed key. (#5481)
- [ ] **tool call indicators in pipe run output** â€” Verify that tool call indicators are displayed in the pipe run output. (`dda33a6a7`)
- [ ] **align copy and chat icons in pipe run history** â€” Verify that copy and chat icons are properly aligned in the pipe run history. (`f8fe5cff4`)
- [ ] **Live pipe output streaming** â€” Open the "Runs" tab for a running pipe. Verify that the output streams live as the pipe executes. (`4c1fab276`)
- [ ] **Rich activity-summary** â€” Verify that activity summaries include details about windows, URLs, and audio transcriptions. (`f2d8ba1dad3`)
- [ ] **OpenAI-compatible transcription endpoint** â€” Verify that the `/v1/audio/transcriptions` endpoint works correctly with standard OpenAI clients. (`59deeba19`)
- [ ] **Mermaid diagram XSS sanitization** â€” Verify that mermaid diagrams in the UI are correctly sanitized to prevent XSS attacks. (`3405e9793`)
- [ ] **Per-machine pipe favorites (stars)** â€” Toggle the star icon for a pipe. Verify that favorites are persisted per-machine and that the filter chip correctly shows starred pipes first. (`e1a18adb9`, `0a2c1abb7`)
- [ ] **Connected integrations @mentions in chat** â€” Open the filter popover in chat. Verify that connected integrations (like Notion, Slack, Google Docs) appear as @mentions for easy filtering. (`1c0c95b20`)

commits: `fa887407`, `815f52e6`, `60840155`, `e66c3ff8`, `c905ffbf`, `01147096`, `5908d7f4`, `46422869`, `4f43da70`, `71a1a537`, `6abaaa36`

- [ ] **Pi process stability** â€” After app launch, `ps aux | grep pi` should show a single, stable `pi` process that doesn't restart or get killed.
- [ ] **Pi readiness handshake** â€” First chat interaction with Pi should be fast (<2s for readiness).
- [ ] **Pi auto-recovery** â€” If the `pi` process is manually killed, it should restart automatically within a few seconds and be ready for chat.
- [ ] **Pipe output accuracy** â€” When executing a pipe, the user's prompt should be accurately reflected in the output.
- [ ] **Silent LLM errors** â€” LLM errors during pipe execution should be displayed to the user, not silently suppressed.
- [ ] **Fast first chat with Pi** â€” The first interaction with Pi after app launch should be responsive, with no noticeable delay (aim for <2s).
- [ ] **Activity Summary tool** â€” MCP can access activity summaries via the `activity-summary` tool, and the `activity-summary` endpoint works correctly.
- [ ] **Search Elements tool** â€” MCP can search elements using the `search-elements` tool.
- [ ] **Frame Context tool** â€” MCP can access frame context via the `frame-context` tool.
- [ ] **Progressive disclosure for AI data** â€” AI data querying should progressively disclose information.
- [ ] **MEMORA Analytics skill** â€” The `MEMORA-analytics` skill can be used by the Pi agent to perform raw SQL usage analytics.
- [ ] **MEMORA Retranscribe skill** â€” The `MEMORA-retranscribe` skill can be used by the Pi agent for retranscription.
- [ ] **AI preset save stability** â€” Saving AI presets should not cause crashes, especially when dealing with pipe session conflicts.
- [ ] **Pipe token handling** â€” Ensure that Pi configuration for pipes uses the actual token value, not the environment variable name.
- [ ] **Pipe user_token passthrough** â€” Verify that the `user_token` is correctly passed to Pi pre-configuration so pipes use the MEMORA provider.
- [ ] **Default AI model ID** â€” Verify that the default AI model ID does not contain outdated date suffixes.
- [ ] **Move provider/model flags** â€” `--provider` and `--model` flags should be correctly moved before `-p prompt` in `pi spawn` commands.

### 18. Admin / Team features

commits: `58460e02`, `853e0975`

- [ ] **Admin team-shared filters** â€” Admins should be able to remove individual team-shared filters.
- [ ] **Simplified team invite** â€” Verify the simplified team invite flow using a single web URL without requiring a passphrase. (`44a19b73f`, `b53b08b6e`)
- [ ] **Per-request AI cost tracking and admin spend endpoint** â€” Verify that per-request AI costs are tracked correctly and that the admin spend endpoint provides accurate usage data.

commits: `58460e02`

- [ ] **Admin team-shared filters** â€” Admins should be able to remove individual team-shared filters.

### 19. Logging

commits: `fc830b43`, `f54d3e0d`

- [ ] **Reduced log noise** â€” Verify a significant reduction in log noise (~54%).
- [ ] **PII scrubbing** â€” Ensure that PII (Personally Identifiable Information) is scrubbed from logs.
- [ ] **Phone regex PII scrubbing preservation** â€” Verify phone numbers are scrubbed but accessibility bounds (which look like numbers) are NOT mangled. (`08feb4df5`)
- [ ] **Phone regex PII scrubbing** â€” After generating some PII-containing data (e.g., typing phone numbers), review logs to ensure that the phone regex correctly scrubs PII and does not over-match bare digit sequences.

### 20. Vault Lock (Encryption at rest)

commits: `274a968af`, `dc575e48e`, `81aabbf18`, `d5e071854`, `db08f8c06`, `f4225b580`

- [ ] **Vault lock initialization** â€” Verify that the vault can be initialized and a password set.
- [ ] **Encryption of database and data files** â€” Verify that MEMORA data is encrypted at rest when the vault is locked.
- [ ] **Recording stop on lock** â€” Verify that recording stops immediately when the vault is locked.
- [ ] **Recording resume on unlock** â€” Verify that recording restarts automatically when the vault is unlocked.
- [ ] **Fast vault unlock** â€” Verify that the DB is decrypted quickly and data files are decrypted in the background. (`dc575e48e`)
- [ ] **Vault lock shortcut** â€” Verify that the configurable vault lock shortcut works as expected. (`81aabbf18`)
- [ ] **CLI vault commands** â€” Verify that `MEMORA vault` commands work without the server running. (`f4225b580`)
- [ ] **Skip server start on locked vault** â€” Verify that the server does not start if the vault is locked. (`d5e071854`)

### 21. Privacy & Incognito Detection

- [ ] **PII Filter** â€” Toggle the PII filter in chat or search. Verify that sensitive information is filtered using Tinfoil. (`fec0f1023`)


commits: `ad431b513`, `d9722bccc`, `4df21e83d`

- [ ] **Incognito window detection** â€” Verify that private browsing/incognito windows are correctly detected for major browsers (Chrome, Safari, Firefox, etc.). (`ad431b513`)
- [ ] **Ignore incognito toggle** â€” Verify that the "Ignore Incognito Windows" toggle in settings correctly prevents recording of private windows. (`d9722bccc`)
- [ ] **Incognito detection UI feedback** â€” Verify that the UI correctly reflects when an incognito window is being ignored.
- [ ] **DRM pause behavior** â€” Play DRM-protected content (e.g., Netflix in Safari). Verify that MEMORA pauses recording gracefully and resumes automatically once the DRM content is closed, without crashing the server. (`3d9f0e8bb`)
- [ ] **LAN-access toggle** â€” Toggle "Enable LAN access" in API settings. Verify that the API binds to `0.0.0.0` and that `api_auth` is forcibly enabled for security. (`c8d9c83f0`)

commits: `fc830b43`

- [ ] **Reduced log noise** â€” Verify a significant reduction in log noise (~54%).
- [ ] **PII scrubbing** â€” Ensure that PII (Personally Identifiable Information) is scrubbed from logs.

### 23. GPU & Performance Telemetry

- [ ] **GPU error handling & telemetry** â€” Verify that GPU errors are handled gracefully and CPU/GPU telemetry is correctly reported in logs. (`0d42ea221`)
- [ ] **Clipboard thread leak** â€” Verify that long-running sessions do not exhibit gradual input lag or memory growth due to clipboard thread leaks. (`0718c2e03`, `f0adcddd0`)

### 24. Data Management

- [ ] **Delete local data confirmation** â€” Use the "Delete device local data" feature. Verify an `AlertDialog` appears instead of a standard `window.confirm`. (`b5db080d6`)

### 25. Feedback & Support

- [ ] **Compressed feedback screenshots** â€” Send feedback with a screenshot. Verify that the screenshot is compressed to JPEG before sending. (`591710246`)

## how to run

### before every release
1. run sections 1-4 completely (90% of regressions)
2. spot-check sections 5-10

### before merging window/tray/dock changes
run section 1 and 2 completely. these are the most fragile.

### before merging vision/OCR changes
run section 3, 5, and 14 (Windows text extraction matrix) completely.

### before merging audio changes
run section 4 completely.

### before merging AI changes
run section 10.

### before merging updater / tray-update changes
run section 32, and run `bun run test:e2e:packaged-updater:macos` (macOS).

## known limitations (not bugs)

- tray icon on notched MacBooks can end up behind the notch if menu bar is crowded. Cmd+drag to reposition. dock menu is the fallback.
- macOS only shows permission prompts once (NotDetermined â†’ Denied is permanent). must use System Settings to re-grant.
- debug builds use ~3-5x more CPU than release builds for vision pipeline.
- first frame after app launch always triggers OCR (intentional â€” no previous frame to compare against).
- chat panel is pre-created hidden at startup so it exists before user presses the shortcut. Creation no longer activates/shows â€” only the show_existing path does (matching main overlay pattern).
- shortcut reminder should use `CanJoinAllSpaces` (visible on all Spaces simultaneously). chat and main overlay should use `MoveToActiveSpace` (moved to current Space on show, then flag removed to pin).

## log locations

```
macOS:   ~/.MEMORA/MEMORA-app.YYYY-MM-DD.log
Windows: %USERPROFILE%\.MEMORA\MEMORA-app.YYYY-MM-DD.log
Linux:   ~/.MEMORA/MEMORA-app.YYYY-MM-DD.log
```

### what to grep for

```bash
# crashes/errors
grep -E "panic|SIGABRT|ERROR|error" ~/.MEMORA/MEMORA-app.*.log

# monitor events
grep -E "Monitor.*disconnect|Monitor.*reconnect|Starting vision" ~/.MEMORA/MEMORA-app.*.log

# frame skip rate (debug level only)
grep "Hash match" ~/.MEMORA/MEMORA-app.*.log

# queue health
grep "Queue stats" ~/.MEMORA/MEMORA-app.*.log

# DB contention
grep "Slow DB" ~/.MEMORA/MEMORA-app.*.log

# audio issues
grep -E "audio.*timeout|audio.*error|device.*disconnect" ~/.MEMORA/MEMORA-app.*.log

# window/overlay issues
grep -E "show_existing|panel.*level|Accessory|activation_policy" ~/.MEMORA/MEMORA-app.*.log
```

### 12. mainland china / great firewall

- [ ] **full app functionality behind GFW** â€” download, onboarding, AI chat, cloud features, and update checks must all work (or degrade gracefully) on networks subject to the Great Firewall.
- [ ] **HF_ENDPOINT Chinese mirror** â€” verify model downloads work in China via the HF mirror. (`7ea1eb94e`)

### 22. WhatsApp Gateway

commits: `cf2dcd5f8`, `ad1d00d8f`, `6f623b30a`, `aaf031169`

- [ ] **WhatsApp gateway auto-restart** â€” Manually terminate the WhatsApp gateway process. Verify the watchdog restarts it automatically. (`cf2dcd5f8`)
- [ ] **WhatsApp gateway self-termination** â€” Kill the main MEMORA process. Verify the WhatsApp gateway process also terminates. (`ad1d00d8f`)
- [ ] **WhatsApp history & contacts sync** â€” Verify that WhatsApp chat history and contacts are correctly synchronized. (`aaf031169`)
- [ ] **WhatsApp auto-reconnect** â€” Verify the WhatsApp gateway automatically reconnects on server start. (`6f623b30a`)

### 23. Notifications

- [ ] **Restart notifications toggle** â€” Toggle "restart notifications" in settings. Verify notifications only appear when enabled. (`f82b4f350`)
- [ ] **Notification text selection** â€” Verify that text can be selected in notification inbox messages. (`3449197c3`)
- [ ] **macOS notification "Open" click** â€” Click "Open" on a macOS system notification. Verify it correctly brings the MEMORA window to the front. (`3e86cebb0`)

### 26. Onboarding & Fleet UX

commits: `f6c21a022`, `31e67ae1c`, `8d0a5348d`, `b1c30e99b`

- [ ] **Redesigned Onboarding** â€” Complete the redesigned onboarding. Verify live feed appears and opinionated pipe setup works. (`f6c21a022`)
- [ ] **Pipes & Fleet merged UI** â€” Open Pipes tab. Verify fleet devices appear in the dropdown. Verify local machine is filtered/distinct. (`31e67ae1c`, `8d0a5348d`)
- [ ] **Scheduled vs Manual pipes** â€” In My Pipes, verify sub-tabs for scheduled and manual pipes. (`b1c30e99b`)

### 27. Connections (Multi-instance & New Services)

- [ ] **Microsoft 365 / Teams** â€” Verify that Microsoft Graph OAuth works for Microsoft 365 and Teams (excluding personal accounts). (`635c32347`, `f35e999b0`)
- [ ] **New Integrations** â€” Verify Loops, Resend, and Supabase integrations. (`ea454f324`)
- [ ] **Google Docs Read/Write** â€” Verify Google Docs integration supports both read and write scopes. (`8f3ca5283`)


commits: `c8769545b`, `4f522325b`, `54000c295`

- [ ] **Multi-instance connections** â€” Add two different accounts for the same service (e.g., two Slack workspaces). Verify both work independently. (`c8769545b`)
- [ ] **Post-install connection modal** â€” After installing a pipe, verify the connection modal appears if the pipe requires a service connection. (`c8769545b`)
- [ ] **New service connections** â€” Verify Brex, Stripe, Sentry, Vercel, Pipedrive, Intercom, and Limitless connections can be authorized and sync data. (`4f522325b`, `54000c295`)
- [ ] **Multi-instance OAuth for GitHub and Notion** â€” Verify that multi-instance OAuth works for GitHub and Notion, including fetching identity after token exchange. (`5d6ee5da3`)
- [ ] **Glean icon in connections grid** â€” Verify that the Glean icon is displayed in the connections grid. (`ec6374e1d`)
- [ ] **Google Docs connection & Pro gate** â€” Verify that Google Docs connection works and that the "Pro required" gate correctly appears for non-pro users on the connect button. (`9835b09d8`, `dbf451f34`, `dda16447c`, `e3a2be5cb`)
- [ ] **Bitrix24 CRM integration** â€” Verify that Bitrix24 CRM connection can be authorized and syncs data correctly. (`55026df56`)
- [ ] **OAuth auto-refresh** â€” Verify that expired OAuth tokens for generic proxy connections (like Google, Bitrix24) are automatically refreshed. (`d7835eabb`)

### 28. Deployment & Remote Management

commits: `c6a73b17e`, `945b687ec`

- [ ] **Deploy to offline devices** â€” Use chat prompt to deploy MEMORA to an offline device. Verify it handles the "Screen Sharing" permission dialog by opening it on the target machine. (`c6a73b17e`, `945b687ec`)

### 29. Browser Extension

- [ ] **Extension popup** â€” Open the browser extension popup. Verify connection status is displayed correctly. (`be7c9e8b5`)


- [ ] **Browser extension token auth** â€” Open the browser extension options page. Verify that token-based authentication works and that it can successfully connect to the MEMORA API. (`be14de544`)

### 30. CLI

- [ ] **CLI logout** â€” Run `MEMORA logout`. Verify it clears local auth tokens. (`793c3d6e9`)
- [ ] **CLI sync remote** â€” Verify `MEMORA sync remote` command and its configuration. (`f46e85cb1`)

### 31. Chat (Pi)

- [ ] **Parallel chats** â€” Verify that multiple chat sessions can run in parallel and their background streams remain visible when switching. (`c9d64ce23`)
- [ ] **Chat sidebar navigation** â€” Verify that the chat sidebar (pinned, recents, live status) works correctly and replaces the Home view for "New chat". (`ec5e80992`, `28c4b1ac5`)
- [ ] **Persistent background chats** â€” Verify that chats continue to stream in the background even when navigating away from the chat view. (`0060ae9e5`, `ec5e80992`)
- [ ] **Inline history in overlay** â€” Verify that inline history is restored in the overlay window. (`15b419ec7`)
- [ ] **Notification URL actions** â€” Open a URL action from a native macOS notification when the overlay is not mounted. (`7fdcd2054`)

### 32. Updates (tray "Restart to update")

commits: 2026-08-11 tray-update-ux

- [ ] **Tray restart while signed out** â€” On a signed-out / entitlement-gated install (engine never starts, boot phase `idle`), stage an update and click the tray "Restart to update". It MUST proceed and relaunch â€” before this fix the click silently no-oped forever (MacBook Air report).
- [ ] **Tray click feedback** â€” Clicking "Restart to update" immediately changes the menu item to "Installing updateâ€¦"; a deferred/failed restart shows a native notification instead of doing nothing.
- [ ] **Fast install (no blackout)** â€” The exit-path install uses the pre-extracted rename path (log: `installed via pre-extracted fast path`). Oldâ†’new blackout is a few seconds, not 10â€“40s.
- [ ] **Failed-install detection** â€” If an update quits but doesn't apply, the next boot shows "Update didn't apply â€” click to retry" (marker in `~/.MEMORA/update-attempt.json`, consumed once).
- [ ] **No-update click safety** â€” Clicking the menu item with nothing staged runs a check and must NOT restart the app.

Automated: `bun run test:e2e:packaged-updater:macos` drives all of the above against two real signed release-local builds (macOS only). Rust unit coverage: `cargo test --features e2e staged_update:: updates::tests` (pre-extract fast path, rollback, fallback, idle gate, marker classification).
