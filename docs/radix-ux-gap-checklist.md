# Radix UI 수준 UX 갭 체크리스트

작성일: 2026-07-02

## 목표

Finui는 `egui` 기반 금융 UI 라이브러리지만, `finui-primitives`의 목표는 Radix UI처럼 조합 가능한 primitive와 높은 상호작용 완성도를 제공하는 것이다. 이 문서는 현재 코드와 공식 Radix Primitives 문서를 대조해, 추가 개발에 필요한 남은 작업을 체크리스트로 정리한다.

## 기준선

Radix Primitives의 핵심 기준은 컴포넌트 외형이 아니라 동작 계약이다.

- 공식 Primitives 소개는 "unstyled, accessible" building block을 기준으로 삼고, 고품질 앱/디자인 시스템의 기본 부품을 제공한다고 설명한다.
- Dialog는 modal/non-modal, 자동 포커스 트랩, Title/Description 기반 스크린리더 announce, Escape 닫기, controlled/uncontrolled API를 포함한다.
- Dropdown Menu는 하위 메뉴, checkable item, collision handling, arrow key navigation, typeahead를 포함한다.
- Popover는 fine-grained focus control, collision handling, origin/collision-aware animation을 포함한다.
- Accessibility 기준은 WAI-ARIA 패턴, 키보드 내비게이션, 포커스 관리, 스크린리더 테스트, RTL 지원까지 포함한다.
- API 기준은 unstyled, composable, fully-typed, component part별 access, 일관된 prop/state/data attribute 모델이다.

참고:

- <https://www.radix-ui.com/primitives>
- <https://www.radix-ui.com/primitives/docs/components/dialog>
- <https://www.radix-ui.com/primitives/docs/components/dropdown-menu>
- <https://www.radix-ui.com/primitives/docs/components/popover>
- <https://www.radix-ui.com/primitives/docs/components/tooltip>

## 현재 구현 기준선

- `README.md`는 Finui를 dense, testable financial UI를 위한 `egui` workspace로 정의하고, `finui-primitives`를 Radix-style primitive controls로 둔다.
- `docs/api-surface.md`는 primitive가 option struct, output struct, pure helper, egui render helper를 노출하는 좁은 API를 지향한다고 설명한다.
- `crates/finui-primitives/src/primitives/mod.rs`에는 Dialog, Alert Dialog, Popover, Tooltip, Dropdown/Context Menu, Menubar, Navigation Menu, Select, Scroll Area, Tabs, Accordion, Form controls, Toast 등 많은 Radix 계열 표면이 있다.
- 2026-07-02 기준 `cargo test -p finui-primitives --lib`는 253개 테스트 통과, `cargo test -p finui-grid --lib`는 34개 테스트 통과.
- 같은 기준에서 `cargo test --workspace`는 120초 제한에서 시간 초과했다. 패키지 단위 테스트는 통과했지만 전체 workspace 검증은 별도 안정화가 필요하다.
- `examples/primitives_lab`는 현재 checkbox와 action button만 보여준다. 구현된 primitive 범위에 비해 실제 상호작용 데모/검증 표면이 매우 얇다.
- `examples/grid_lab`는 grid demo 중심이며 primitive 수준의 Radix parity를 검증하는 표면은 아니다.

## 갭 요약

현재 구현은 Radix 스타일의 타입/파트 이름과 순수 상태 헬퍼를 폭넓게 갖췄다. 부족한 지점은 웹 수준 UX가 요구하는 런타임 상호작용이다. 특히 포커스 트랩/복귀, 실제 키보드 시퀀스, 스크린리더 의미 전달, submenu, 충돌 기반 포지셔닝/애니메이션, RTL별 키 동작, hover/touch delay, visual regression, 문서/예제 품질이 아직 Radix 수준으로 증명되지 않았다.

## P0: 공통 런타임 계약

- [ ] 공통 focus manager를 만든다.
  - Dialog/AlertDialog: 열릴 때 초기 포커스 이동, 닫힐 때 trigger로 복귀, modal 내부 Tab/Shift+Tab 순환.
  - Popover/Menu/Select: 열릴 때 콘텐츠 또는 첫 활성 항목 focus, 닫힐 때 trigger 복귀.
  - Tooltip/HoverCard: focus와 hover 모두에서 open/close 동작 일관화.
  - 완료 기준: 각 primitive별 `open -> focus target -> Escape/outside close -> trigger restore` 테스트가 있다.

- [ ] modal/inert 정책을 실제 런타임 동작으로 만든다.
  - 현재 modal/default flag와 backdrop은 있으나, 배경 영역의 keyboard/pointer/focus 차단 계약을 명확히 증명해야 한다.
  - modal=false일 때 외부 상호작용 허용, modal=true일 때 외부 focus/interaction 차단을 분리한다.
  - 완료 기준: Dialog, AlertDialog, DropdownMenu modal/non-modal 케이스별 입력 차단 테스트가 있다.

- [x] dismissable layer 이벤트 모델을 확장한다.
  - 현재 `DismissPolicy`는 outside click과 Escape 중심이다.
  - Radix 수준의 `onEscapeKeyDown`, `onPointerDownOutside`, `onInteractOutside`, close prevent/default 방지에 해당하는 Rust API를 설계한다.
  - 완료 기준: outside pointer, secondary click, Escape, trigger 재클릭, nested layer의 닫힘 순서를 테스트한다.

- [ ] 포털/레이어 컨테이너 계약을 실제 배치 계층으로 연결한다.
  - 여러 primitive에 `container`/`force_mount` output은 있으나 실제 container routing은 약하다.
  - 완료 기준: 지정 컨테이너, nested portal, forced mounted but visually hidden 상태를 테스트한다.

- [ ] controlled/uncontrolled API를 컴포넌트별로 통일한다.
  - 현재 `open`, `default_open`, `value`, `default_value` 필드는 많지만, 실제 state owner 규칙은 helper 단위에 머문다.
  - 완료 기준: 각 Root가 controlled state와 uncontrolled local state를 같은 방식으로 다루는 예제와 테스트를 가진다.

- [ ] data-state/data-side/data-align/data-disabled/data-orientation에 해당하는 출력 계약을 모든 primitive에 통일한다.
  - 완료 기준: 공통 trait 또는 naming convention 문서가 있고, primitive별 누락 목록이 없다.

## P1: 접근성 및 의미 전달

- [ ] egui 환경에서 가능한 accessibility bridge를 정의한다.
  - ARIA 속성을 그대로 적용할 수 없으므로, `role`, `label`, `description`, `state`, `value`, `live`를 output/event로 노출하는 Finui 고유 계약이 필요하다.
  - 완료 기준: screen-reader용 snapshot 구조 또는 agent-readable accessibility tree가 있다.

- [ ] Dialog/AlertDialog의 Title/Description announce 계약을 강화한다.
  - 현재 title/description 렌더 함수는 있지만, Dialog Content에 accessible title/description이 필수인지 검증하지 않는다.
  - 완료 기준: title 누락, description 생략, visually hidden title을 각각 표현할 수 있다.

- [ ] Form/Label/Message의 실제 연결성을 강화한다.
  - 현재 validity/data output은 있으나 label-control-message 연결 관계가 UI runtime에서 추적되는지 약하다.
  - 완료 기준: field name, label for/id, described-by, error message mapping에 해당하는 Finui 계약이 있다.

- [ ] Toast live region 동작을 구현/검증한다.
  - 현재 `aria_live`, role, hotkey label output은 있으나 실제 announce queue와 hotkey focus 이동이 부족하다.
  - 완료 기준: foreground/background toast announce 우선순위, viewport hotkey, close/action alt text를 테스트한다.

- [ ] RTL 동작을 표시용 output이 아니라 실제 키보드/배치 동작으로 만든다.
  - 일부 RootOptions에 direction이 있으나 ArrowLeft/ArrowRight, side/align, menu direction의 실제 반전 검증이 부족하다.
  - 완료 기준: LTR/RTL 각각에서 menu, select, tabs, radio, slider 키 동작 테스트가 있다.

## P1: 키보드 및 입력 상호작용

- [ ] DropdownMenu/ContextMenu/MenuBar의 실제 roving focus를 구현한다.
  - 현재 typeahead helper와 일부 menubar navigation helper는 있으나, 메뉴 콘텐츠에서 highlighted item/focus item이 input loop에 연결된 증거가 부족하다.
  - 완료 기준: ArrowUp/Down, Home/End, Enter/Space, Escape, typeahead buffer가 실제 menu open 상태에서 동작한다.

- [ ] DropdownMenu/ContextMenu에 submenu를 추가한다.
  - Radix 기준에서 submenu는 핵심 기능이다.
  - 완료 기준: SubTrigger/SubContent, open delay, pointer grace area, ArrowRight/ArrowLeft 진입/복귀, nested outside dismissal 테스트가 있다.

- [ ] Select를 DropdownMenu alias 수준에서 독립 Select UX로 끌어올린다.
  - 현재 Select content는 DropdownMenu 기반이며 item-aligned positioning, selected item focus, typeahead buffer, value text announcement가 약하다.
  - 완료 기준: open 시 selected item focus, Arrow navigation, Enter commit, Escape cancel, disabled item skip, placeholder/value 렌더 테스트가 있다.

- [ ] Tabs의 방향성과 activation mode를 완성한다.
  - 현재 horizontal 중심 roving focus가 있고 selected를 즉시 변경한다.
  - 완료 기준: horizontal/vertical, automatic/manual activation, Home/End, disabled tab skip, focus와 selected 분리 테스트가 있다.

- [ ] Accordion/Collapsible의 키보드 계약을 추가한다.
  - 현재 click 토글과 open state helper는 있으나 trigger keyboard interaction이 약하다.
  - 완료 기준: Enter/Space toggle, Arrow/Home/End header navigation, single/multiple/collapsible 규칙 테스트가 있다.

- [ ] Slider의 Radix 수준 입력을 완성한다.
  - 현재 pointer drag와 snap/clamp helper는 있으나 keyboard/touch/multiple thumbs/min steps between thumbs가 실제 조작으로 완성됐는지 부족하다.
  - 완료 기준: Arrow/Page/Home/End, vertical/RTL/inverted, multiple thumbs, min step separation, touch/pointer drag 테스트가 있다.

- [ ] Checkbox/Switch/Radio의 keyboard activation을 명확히 구현한다.
  - 현재 click 중심 렌더/상태 변경이 강하다.
  - 완료 기준: Space/Enter, disabled/required/name/value, focus ring, radio arrow navigation, loop focus 테스트가 있다.

- [ ] Tooltip/HoverCard의 delay state machine을 실제 시간/hover/focus 이벤트로 연결한다.
  - 현재 delay duration 필드와 data-state output은 있으나 runtime timer behavior가 제한적이다.
  - 완료 기준: delay, skipDelay, disableHoverableContent, pointer leave grace, focus open/blur close 테스트가 있다.

- [ ] Toast swipe를 실제 gesture로 구현한다.
  - 현재 swipe state/direction output은 있으나 drag gesture와 threshold dismissal이 약하다.
  - 완료 기준: swipe start/move/cancel/end, threshold, pause on hover/focus, action/close focus 테스트가 있다.

## P1: 포지셔닝, 충돌, 애니메이션

- [x] Layer placement를 collision-aware contract로 확장한다.
  - 현재 screen clamp와 일부 flip은 있다.
  - Radix 수준은 side, align, sideOffset, alignOffset, collisionPadding, sticky, avoidCollisions, arrow padding 같은 제어가 필요하다.
  - 완료 기준: viewport edge, scroll container, nested layer, arrow alignment 테스트가 있다.

- [x] side/align output이 실제 충돌 후 결과를 반영하도록 만든다.
  - 현재 side/align은 요청 placement에서 계산되는 경우가 많고, 충돌 후 flip 결과와 항상 일치한다고 보기 어렵다.
  - 완료 기준: 아래에서 위로 flip된 경우 `data-side=top`에 해당하는 output이 나온다.

- [ ] origin-aware/collision-aware animation hook을 추가한다.
  - CSS 변수는 없지만 egui에서도 animation origin, open/close progress, collision side를 output으로 줄 수 있다.
  - 완료 기준: Popover/Dropdown/Tooltip/HoverCard가 open/close progress와 transform origin equivalent를 노출한다.

- [ ] Force-mounted closed content의 시각/상호작용 정책을 통일한다.
  - 일부 text color muted 처리만 있고 hit-test/focus exclusion 계약은 약하다.
  - 완료 기준: force-mounted closed layer는 보이지 않고 focus/pointer 대상이 되지 않는다.

## P1: 컴포넌트별 API parity

- [ ] Dialog API에 `Trigger`, `Portal`, `Overlay`, `Content`, `Title`, `Description`, `Close`의 역할/필수성 문서를 붙인다.
- [ ] AlertDialog는 cancel/action의 focus priority와 destructive action semantics를 분리한다.
- [ ] DropdownMenu는 Item/CheckboxItem/RadioItem/Label/Separator/Group/Sub/Arrow/Shortcut 계약을 완성한다.
- [ ] ContextMenu는 pointer origin, long press/touch open, keyboard context key open을 추가한다.
- [ ] Menubar는 top-level navigation과 opened menu content navigation을 하나의 state machine으로 통합한다.
- [ ] NavigationMenu는 viewport/indicator/motion이 실제 hover/focus/open state와 연결되도록 한다.
- [ ] Popover는 Anchor, Trigger, Close, Arrow, modal mode, custom focus hook을 완성한다.
- [ ] Tooltip은 Provider shared delay, accessible trigger labeling, hoverable content 정책을 완성한다.
- [ ] Select는 Value/Icon/Viewport/ScrollButton/ItemIndicator/Group contract를 확장한다.
- [ ] ScrollArea는 native scrolling 보존, thumb drag, scrollbar visibility delay, horizontal/vertical corner behavior를 검증한다.
- [ ] Form Preview 계열은 validation matching, server invalid, async validation demo를 갖춘다.
- [ ] OTP/PasswordToggle은 preview primitive가 아닌 안정 API로 둘지, experimental로 분리할지 결정한다.

## P2: 데모와 시각 검증

- [ ] `examples/primitives_lab`를 primitive catalogue로 확장한다.
  - Dialog, Popover, Tooltip, DropdownMenu, ContextMenu, Select, Tabs, Accordion, Slider, Toast, Toolbar를 한 화면 또는 탭별로 조작할 수 있어야 한다.
  - 완료 기준: 각 primitive가 open/close, disabled, RTL, dark/light, long text, edge placement 상태를 가진다.

- [ ] visual regression baseline을 만든다.
  - 현재 grid screenshot은 있으나 primitive screenshot baseline은 없다.
  - 완료 기준: primitive lab의 주요 상태별 PNG snapshot과 diff threshold가 있다.

- [ ] keyboard scenario test harness를 만든다.
  - 순수 helper 테스트 외에 실제 egui input sequence 테스트가 필요하다.
  - 완료 기준: 메뉴 열기, 방향키 이동, 타입어헤드, Escape 닫기, Dialog Tab trap 같은 end-to-end 시퀀스가 자동화된다.

- [ ] accessibility snapshot test를 만든다.
  - 완료 기준: primitive별 role/name/state/value/description output을 JSON snapshot으로 검증한다.

- [ ] `grid_lab`와 primitive layer의 통합 시나리오를 만든다.
  - grid header/row/cell context menu, hover card, scroll area, selection/focus가 primitive 계약을 실제로 쓰는지 확인한다.
  - 완료 기준: grid context menu keyboard navigation과 dismiss behavior가 primitive harness와 같은 규칙을 따른다.

## P2: 문서 및 개발자 경험

- [ ] 각 primitive별 "Anatomy" 문서를 추가한다.
  - Root/Trigger/Portal/Content/Item/Indicator/Arrow 같은 part 목록과 역할을 정리한다.

- [ ] 각 primitive별 API reference를 작성한다.
  - 옵션 필드, output 필드, state/data mapping, 기본값, egui 제한사항을 표로 정리한다.

- [ ] 각 primitive별 Accessibility 문서를 추가한다.
  - 웹 ARIA와 Finui egui accessibility bridge 사이의 대응표를 만든다.

- [ ] 각 primitive별 Keyboard Interactions 문서를 추가한다.
  - Radix 문서처럼 key별 동작을 명확히 적고 테스트 이름을 연결한다.

- [ ] controlled/uncontrolled 예제를 추가한다.
  - caller-owned state, local helper state, agent-controlled state를 각각 보여준다.

- [ ] "Radix parity matrix" 문서를 유지한다.
  - component, supported parts, missing parts, keyboard, focus, accessibility, visual tests, examples, status를 표로 관리한다.

- [ ] crate-level feature stability 정책을 문서화한다.
  - stable, experimental, preview, internal helper를 구분한다.

## P2: 테마와 스타일 시스템

- [ ] "unstyled primitive"와 "Finui default theme" 경계를 분리한다.
  - 현재 primitive가 Radix color와 Finui theme를 직접 칠하는 경우가 많다.
  - 완료 기준: logic-only part output과 themed renderer를 분리할 수 있다.

- [ ] state token을 통일한다.
  - hover, active, selected, highlighted, checked, disabled, invalid, open/closed 색상/두께/radius 기준을 문서화한다.

- [ ] dark/light contrast 검증을 확대한다.
  - 현재 theme 테스트는 기본 대비 수준이다.
  - 완료 기준: 주요 state text/background/border 대비가 snapshot 또는 numeric check로 검증된다.

- [ ] long text, dense financial labels, Korean labels, mixed numeric labels의 clipping/ellipsis 정책을 통일한다.

- [ ] icon contract를 정리한다.
  - Radix icon asset 사용 범위, fallback, tinting, accessible icon/decorative icon 구분을 문서화한다.

## P3: 테스트/릴리스 운영

- [ ] 전체 workspace 테스트가 시간 제한 안에 안정적으로 끝나도록 분리한다.
  - 현재 패키지별 lib 테스트는 빠르게 통과하지만 `cargo test --workspace`는 120초 제한에서 시간 초과했다.
  - 완료 기준: CI와 로컬 quick/full test command가 문서화되고, quick path는 2분 안에 안정적으로 끝난다.

- [x] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets`, `cargo check --workspace --all-targets`, no-default-features check를 CI gate로 명확히 묶는다.

- [ ] primitive별 acceptance test 이름을 문서 checklist에 연결한다.
  - 체크박스를 완료 처리할 때 테스트 이름과 명령을 함께 남긴다.

- [ ] agent-testable UX receipts를 만든다.
  - `finui-grid`의 agent-testable 방향을 `finui-primitives`에도 적용한다.
  - 완료 기준: agent가 primitive state, focus, selected item, open layer stack을 snapshot으로 읽을 수 있다.

- [ ] public API breaking change policy를 정한다.
  - pre-release이므로 API 축소/정리 여지는 있지만, downstream `imai`가 쓰는 표면은 별도 보호해야 한다.

## 우선 개발 순서

1. 공통 focus/layer/dismiss/accessibility bridge를 먼저 만든다.
2. Dialog, DropdownMenu, Select, Tooltip/Popover를 Radix 수준의 대표 primitive로 끌어올린다.
3. `primitives_lab`를 상호작용 검증 허브로 확장한다.
4. keyboard/accessibility/visual snapshot harness를 추가한다.
5. 나머지 primitive에 같은 계약을 반복 적용한다.

## 완료 판정 기준

다음 조건이 모두 만족되어야 Radix UI 수준에 근접했다고 볼 수 있다.

- [ ] 주요 primitive가 Root/Trigger/Content/Portal/Item/Indicator 등 part anatomy를 문서와 코드에서 일관되게 제공한다.
- [ ] pointer, keyboard, focus, dismiss, controlled/uncontrolled state가 실제 runtime test로 검증된다.
- [ ] 스크린리더 의미에 해당하는 Finui accessibility snapshot이 primitive별로 존재한다.
- [x] Popover/Menu/Tooltip 계열의 포지셔닝이 충돌 후 side/align/output과 일치한다.
- [ ] 대표 primitive 상태가 visual snapshot으로 검증된다.
- [ ] `primitives_lab`에서 개발자가 모든 primitive의 주요 상태를 직접 조작할 수 있다.
- [ ] quick CI command와 full CI command가 분리되어 있고, quick path는 일상 개발에 충분히 빠르다.
- [ ] downstream 앱이 쓸 stable API와 preview/internal API가 문서상 구분된다.
