#!/usr/bin/env bash
# Opens gallery pages on a booted iOS simulator by their launch links, light and dark, and saves
# a screenshot of each and the app's log to smoke/. Fails if the app is not running on a page.
set -uo pipefail

APP_PATH=mobile/gallery/ios/build/Build/Products/Debug-iphonesimulator/ElyGallery.app
APP=com.elygpui.gallery
OUT=smoke
mkdir -p "$OUT"

sim=$(xcrun simctl list devices available | grep -m1 "iPhone" | sed -E 's/.*\(([A-F0-9-]+)\).*/\1/')
xcrun simctl boot "$sim"
xcrun simctl bootstatus "$sim" -b
xcrun simctl install "$sim" "$APP_PATH"

# The chapter list, then the pages ElyMark leans on.
pages=("" forms overlays menus navigation lists typography editor layout feedback)
failed=0
for page in "${pages[@]}"; do
  for theme in light dark; do
    link="gpui://gallery/$page"
    [ "$theme" = dark ] && link="$link?dark"
    name="${page:-home}-$theme"
    xcrun simctl terminate "$sim" "$APP" 2>/dev/null || true
    xcrun simctl openurl "$sim" "$link"
    sleep 10
    xcrun simctl io "$sim" screenshot "$OUT/$name.png"
    # Capture first: `grep -q` exits early, and with pipefail the SIGPIPE it leaves
    # `launchctl` fails the check while the app is running.
    services=$(xcrun simctl spawn "$sim" launchctl list)
    if ! grep -q "$APP" <<<"$services"; then
      echo "::error::the gallery is not running on $name"
      failed=1
    fi
  done
done

xcrun simctl spawn "$sim" log show --last 15m --style compact \
  --predicate 'process == "ElyGallery"' > "$OUT/log.txt" || true
exit $failed
