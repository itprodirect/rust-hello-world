# 09 - Release Process

Use this guide to cut repeatable tagged releases from `main`.

## Preconditions

1. Working tree clean (`git status`).
2. `main` is up to date.
3. `CHANGELOG.md` contains a section for the new version.

## One-Command Release (PowerShell)

```powershell
$Version="0.2.1"; $Tag="v$Version"; New-Item -ItemType Directory .tmp -Force | Out-Null; $Notes=".tmp/release-$Tag-notes.md"; $Body=Get-Content CHANGELOG.md -Raw; $Pattern="(?s)## \[$Version\].*?(?=^## \[|\z)"; $Match=[regex]::Match($Body,$Pattern,[System.Text.RegularExpressions.RegexOptions]::Multiline); if (-not $Match.Success) { throw "Could not find CHANGELOG section for $Version" }; $Match.Value | Set-Content $Notes; cargo test --workspace; git tag -a $Tag -m "$Tag"; git push origin main; git push origin $Tag; gh release create $Tag --title $Tag --notes-file $Notes
```

## One-Command Release (Bash)

```bash
VERSION="0.2.1"; TAG="v$VERSION"; mkdir -p .tmp; NOTES=".tmp/release-$TAG-notes.md"; awk -v v="$VERSION" 'BEGIN{p=0} $0 ~ "^## \\["v"\\]"{p=1} $0 ~ "^## \\[" && $0 !~ "^## \\["v"\\]" && p{exit} p{print}' CHANGELOG.md > "$NOTES"; [ -s "$NOTES" ] || { echo "Could not find CHANGELOG section for $VERSION"; exit 1; }; cargo test --workspace && git tag -a "$TAG" -m "$TAG" && git push origin main && git push origin "$TAG" && gh release create "$TAG" --title "$TAG" --notes-file "$NOTES"
```

## Verify After Release

1. `gh release view <tag>`
2. `gh run list --branch main --limit 5`
3. `gh api repos/<owner>/<repo>/commits/<tag-sha>/check-runs`

Last updated: 2026-03-05
