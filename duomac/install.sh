#!/bin/zsh
# DuoMac installer for macOS. Run it with the iPhone plugged in over USB:
#   curl -fsSL https://raw.githubusercontent.com/amamov/DuoMac/main/duomac/install.sh | zsh
# Run it again every 7 days with a free Apple ID (that is when its signature expires); your Macs stay on the iPhone.
set -euo pipefail

REPO="amamov/DuoMac"
VERSION="v0.1.0" # the release the installer fetches; bumped with each release
PLUMESIGN_URL="https://github.com/claration/impactor/releases/download/v2.7.3/plumesign-macos-universal"
PLUMESIGN_SHA256="24bdc78f334f7db324d0d3ae35b1fc67d69f3f8f4a354d10b9967bc3fc261a08"
WORK="$HOME/.duomac"
mkdir -p "$WORK"
cd "$WORK"

say() { print -P "%F{cyan}==>%f $*"; }
warn() { print -P "%F{yellow}!!%f $*"; }
die() { print -P "%F{red}xx%f $*"; exit 1; }
ask() { local a; read -r "a?$1 [y/N] " </dev/tty; [[ "$a" == [yY]* ]]; }

[[ "$(uname)" == "Darwin" ]] || die "DuoMac의 설치 도구는 Mac에서만 동작해요."

# 1. Tools and the app
say "필요한 파일을 받는 중..."
curl -fsSL -o DuoMac.ipa "https://github.com/$REPO/releases/download/$VERSION/DuoMac.ipa"
curl -fsSL -o duomac-pair "https://github.com/$REPO/releases/download/$VERSION/duomac-pair"
if [[ ! -x plumesign ]] || [[ "$(shasum -a 256 plumesign | cut -d' ' -f1)" != "$PLUMESIGN_SHA256" ]]; then
  curl -fsSL -o plumesign "$PLUMESIGN_URL"
fi
[[ "$(shasum -a 256 plumesign | cut -d' ' -f1)" == "$PLUMESIGN_SHA256" ]] || die "plumesign 파일이 손상됐어요. 다시 실행해주세요."
chmod +x duomac-pair plumesign

# 2. The iPhone, over USB
udid=""
for attempt in 1 2; do
  udid="$(./duomac-pair devices | head -1)"
  [[ -n "$udid" ]] && break
  warn "USB로 연결된 iPhone이 안 보여요. 케이블을 다시 꽂고, iPhone 잠금을 풀고, '이 컴퓨터를 신뢰' 창이 뜨면 신뢰를 눌러주세요."
  if [[ $attempt == 1 ]] && ask "Mac의 iPhone 연결 서비스(usbmuxd)를 재시작해볼까요? (Mac 암호를 물어요)"; then
    sudo killall usbmuxd || true
    sleep 3
  else
    read -r "?준비되면 Return을 눌러주세요..." </dev/tty
  fi
done
[[ -n "$udid" ]] || die "iPhone을 찾지 못했어요. 다른 케이블이나 포트로 다시 해보세요."
say "iPhone: $udid"

if [[ "$(./duomac-pair devmode "$udid")" != "on" ]]; then
  warn "iPhone의 개발자 모드가 꺼져 있어요."
  echo "   설정 → 개인정보 보호 및 보안 → 개발자 모드 를 켜고, 재시동 후 '켜기'를 누른 다음 이 명령을 다시 실행해주세요."
  exit 1
fi

# 3. Sign with the user's own Apple ID and install
if ./plumesign account list 2>&1 | grep -q "No accounts found"; then
  say "Apple ID로 로그인해주세요. (앱 서명에만 쓰여요. 비밀번호는 Apple에만 전달되고 이 Mac에 저장된 토큰으로 7일 갱신 때 다시 써요)"
  ./plumesign account login </dev/tty
fi
say "DuoMac을 서명해서 iPhone에 설치하는 중..."
./plumesign sign --apple-id --register-and-install --udid "$udid" --package DuoMac.ipa </dev/tty

# 4. Pair for JIT, over USB (no codes to type on the iPhone)
say "JIT용 페어링 파일을 만드는 중... (iPhone에 허용 창이 뜨면 허용을 눌러주세요)"
./duomac-pair pair "$udid"

cat <<'EOF'

설치 완료. 이제 iPhone에서:
  1. App Store에서 LocalDevVPN 을 설치하고, 열어서 연결을 한 번 눌러주세요 (VPN 구성 허용).
     https://apps.apple.com/app/id6755608044
  2. 설정 → 일반 → VPN 및 기기 관리 에서 내 Apple ID 개발자 앱을 '신뢰'해주세요. (처음 한 번)
  3. DuoMac 을 열고 Turn On JIT → Install 탭에서 macOS Sequoia 15.6.1 설치.
     약 43GB가 필요하고, 설치 동안 충전기를 꽂고 DuoMac을 열어두세요.

무료 Apple ID는 7일마다 서명이 만료돼요. 그때 iPhone을 USB로 꽂고 같은 명령을 다시 실행하면 돼요. (설치한 macOS는 그대로 남아요)
EOF
