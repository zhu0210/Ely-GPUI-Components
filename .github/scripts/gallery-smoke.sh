#!/usr/bin/env bash
# Opens gallery pages on the running emulator by their launch links, light and dark, and saves a
# screenshot of each and the logcat to smoke/. Fails if the app dies or panics on a page.
set -uo pipefail

APK=mobile/gallery/android/app/build/outputs/apk/debug/app-debug.apk
APP=com.elygpui.gallery
OUT=smoke
mkdir -p "$OUT"

adb install -r "$APK"
adb logcat -c

# The chapter list, then the pages ElyMark leans on.
pages=("" forms overlays menus navigation lists typography editor layout feedback)
failed=0
for page in "${pages[@]}"; do
  for theme in light dark; do
    link="gpui://gallery/$page"
    [ "$theme" = dark ] && link="$link?dark"
    name="${page:-home}-$theme"
    adb shell am start -S -W -a android.intent.action.VIEW -d "$link" -n "$APP/dev.gpui.mobile.GpuiActivity" >/dev/null
    sleep 8
    adb exec-out screencap -p > "$OUT/$name.png"
    if ! adb shell pidof "$APP" >/dev/null; then
      echo "::error::the gallery died on $name"
      failed=1
    fi
  done
done

adb logcat -d > "$OUT/logcat.txt"
if grep -qE "PANIC|panicked" "$OUT/logcat.txt"; then
  echo "::error::the gallery panicked"
  grep -E "PANIC|panicked" "$OUT/logcat.txt" | head -20
  failed=1
fi
if grep -q "failed to open window" "$OUT/logcat.txt"; then
  echo "::error::the gallery could not open its window"
  failed=1
fi
exit $failed
