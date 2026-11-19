# DuoMac

**Made by Sangseok Youn** · Henge(Leviidev) 기반

iPhone(특히 iPhone Duo) 안에서 진짜 macOS를 돌리는 앱이에요. 원격 접속이 아니라 iPhone 안에서 macOS가 직접 돌아가요.
[Henge](https://github.com/Leviidev/Henge)(GPL-2.0) 에뮬레이터를 바탕으로, 설치를 Mac에서 명령 한 줄로 끝낼 수 있게 묶었어요.

- 펼친 iPhone Duo 화면을 꽉 채워요 (iOS 27.2 SDK 빌드).
- USB-C/HDMI로 모니터를 연결하면 모니터에 Mac 화면이, iPhone은 트랙패드·키보드가 돼요.
- 블루투스 키보드·마우스, 파일·클립보드 공유가 돼요.

## 준비물

| | 조건 |
|---|---|
| iPhone | A14 칩 이상, **iOS 27** 이상 (iPhone Duo 권장), 빈 공간 약 **43GB** |
| Mac | macOS, 인터넷 연결 |
| 그 외 | USB 케이블(데이터 지원), 본인 Apple ID (무료 계정 가능) |

## 설치 (명령 한 줄)

1. iPhone을 **USB로 Mac에 연결**하고 잠금을 풀어두세요.
2. iPhone에서 **설정 → 개인정보 보호 및 보안 → 개발자 모드**를 켜고 재시동하세요.
   (메뉴가 안 보이면 아래 명령을 한 번 실행하면 나타나요.)
3. Mac의 **터미널**에 붙여넣고 Return:

   ```bash
   curl -fsSL https://raw.githubusercontent.com/amamov/DuoMac/main/duomac/install.sh | zsh
   ```

   - Apple ID와 비밀번호, 이중 인증 코드를 물어요. **본인 계정으로 앱을 서명**하는 데만 쓰여요.
   - 앱 설치와 JIT 페어링까지 자동으로 해요. iPhone에 허용 창이 뜨면 **허용**을 눌러주세요.

4. iPhone에서 마무리:
   1. App Store에서 [LocalDevVPN](https://apps.apple.com/app/id6755608044)을 설치하고, 열어서 **연결**을 한 번 눌러요.
   2. **설정 → 일반 → VPN 및 기기 관리**에서 내 Apple ID 개발자 앱을 **신뢰**해요. (처음 한 번)
   3. **DuoMac**을 열고 **Turn On JIT** → **Install** 탭에서 **macOS Sequoia 15.6.1**을 설치해요.
      설치하는 동안 충전기를 꽂고 DuoMac을 열어두세요. (다운로드 16GB, 시간이 꽤 걸려요)
   4. 끝나면 DuoMac을 완전히 닫았다가 다시 열고 **Start**.

## 7일마다 갱신 (무료 Apple ID)

무료 Apple ID로 서명한 앱은 7일 뒤에 열리지 않아요. iPhone을 USB로 꽂고 **같은 명령을 다시 실행**하면 돼요. 설치한 macOS는 그대로 남아요.
(유료 개발자 계정이면 1년 동안 유지돼요.)

## 사용법

- 화면이 Mac의 트랙패드예요. 탭 = 클릭, 두 손가락 탭 = 우클릭, 두 손가락 스크롤, 세 손가락 스와이프 = Spaces·Mission Control.
- 볼륨 버튼을 클릭으로 쓸 수도 있어요. (설정 → Trackpad)
- 네 손가락 탭 = 세션 컨트롤 표시/숨기기.

## 문제 해결

| 증상 | 해결 |
|---|---|
| "USB로 연결된 iPhone이 안 보여요" | 케이블을 다시 꽂고 잠금 해제, '신뢰' 누르기. 그래도 안 되면 설치 도구가 Mac의 usbmuxd 재시작을 제안해요 (Mac 암호 필요). 충전 전용 케이블이 아닌지도 확인하세요. |
| "개발자 모드가 꺼져 있어요" | 설정 → 개인정보 보호 및 보안 → 개발자 모드 켜기 → 재시동 → 켜기. |
| Turn On JIT 실패 "couldn't reach this device" | LocalDevVPN이 연결돼 있는지 확인하세요. Wi-Fi에서 하세요 (셀룰러에서는 잘 안 돼요). |
| JIT가 계속 실패 | iPhone을 USB로 꽂고 설치 명령을 다시 실행하면 페어링을 새로 만들어요. iPhone 암호를 바꾸면 페어링이 무효가 돼요. |
| "No space" / 설치 실패 | 빈 공간 43GB 이상 확보. 공간이 부족하면 Install 탭에서 Sonoma(약 40GB)나 Ventura(약 35GB)를 고르세요. |
| 앱이 안 열려요 (7일 후) | 위의 갱신을 해주세요. |

## 알려진 한계 (Henge 기준)

- macOS 안에서 Apple 계정 로그인이 안 돼요 → App Store 앱 설치 불가. Chrome, VS Code, Discord, Photoshop 등 웹에서 받는 앱은 돼요.
- 에뮬레이션이라 실제 Mac보다 느려요. 로그인 직후 몇 분은 특히 느려요.
- 첫 부팅 때 iPhone이 많이 뜨거워질 수 있어요.
- TestFlight·App Store 배포는 불가능해요: 배포용 서명에는 JIT에 필요한 디버거 권한(get-task-allow)이 빠져요.

## 이 저장소에서 바꾼 것

Henge `main`(7b55730) 위에:

- 앱 이름 DuoMac, 기본 번들 ID `io.duomac.app`
- [Henge #221](https://github.com/Leviidev/Henge/issues/221) 수정: iOS 27.3 + 큰 RAM iPhone에서 macOS 설치가 "저장 공간 부족"으로 실패하던 문제
- [Henge #130](https://github.com/Leviidev/Henge/pull/130): 같은 Wi-Fi에 다른 iPhone이 있으면 JIT가 엉뚱한 기기에 붙던 문제
- 유료 팀용 entitlements (`ios/Henge/Henge.team.entitlements`)
- Autopilot 명령 `jit`, `install` (개발·테스트용 원격 조작)
- `duomac/`: 설치 도구. `duomac-pair`는 [idevice](https://github.com/jkcoxson/idevice)로 USB를 통해 iOS 27 원격 페어링 파일을 만들어 앱에 넣어요. 서명·설치는 [plumesign](https://github.com/claration/impactor)(MIT)을 받아 써요.

## 직접 빌드하기

Henge와 같아요 (`ios/README.md`, `CHANGELOG.md`의 Building). 추가로:

```bash
# Xcode 스크립트 단계가 macOS 샘플을 iOS SDK로 빌드하다 실패하지 않도록 먼저
env -u SDKROOT make build/henge-agent.gz build/tl-samples.dmg
# 무료 계정용 IPA
HENGE_ENTITLEMENTS=Henge/Henge.personal.entitlements HENGE_IPA_OUT=$PWD/duomac/dist/DuoMac.ipa zsh ios/scripts/package_ipa.sh
# USB 도우미
cd duomac/pair && cargo build --release
```

## 만든 사람

Sangseok Youn ([@amamov](https://github.com/amamov)). macOS 에뮬레이터 엔진은 [Leviidev/Henge](https://github.com/Leviidev/Henge)와 그 기여자들의 작업이에요.

## 라이선스

GPL-2.0-or-later (Henge와 같음). 함께 들어간 StikJIT은 MPL-2.0, idevice는 MIT. 자세한 내용은 [`docs/licensing.md`](../docs/licensing.md).
