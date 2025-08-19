# Design Document

## Overview

이 디자인은 현재의 댓글 시스템을 레딧 스타일의 트리 구조 UI로 개선하는 것입니다. 기존의 하트 기반 좋아요 시스템과 댓글 기능은 유지하면서, 시각적으로 계층적 구조를 명확하게 표현하는 트리 라인과 개선된 레이아웃을 추가합니다.

## Architecture

### Current Architecture Analysis

현재 댓글 시스템은 다음과 같은 구조를 가지고 있습니다:

- **CommentList.svelte**: 댓글 목록을 관리하는 최상위 컴포넌트
- **Comment.svelte**: 개별 댓글을 렌더링하는 컴포넌트 (재귀적으로 답글 렌더링)
- **CommentForm.svelte**: 댓글 작성 폼 컴포넌트
- **useComments.svelte.ts**: 댓글 목록 상태 관리 훅
- **useReplies.svelte.ts**: 답글 상태 관리 훅

### Proposed Changes

레딧 스타일 UI를 구현하기 위해 다음과 같은 변경사항을 적용합니다:

1. **Comment.svelte 리팩토링**: 트리 구조 시각화를 위한 새로운 레이아웃
2. **CSS 스타일 개선**: 레딧과 유사한 시각적 스타일
3. **트리 라인 컴포넌트**: 계층 구조를 나타내는 시각적 요소

## Components and Interfaces

### 1. Enhanced Comment Component

**Comment.svelte** 컴포넌트를 다음과 같이 개선합니다:

#### Props Interface

```typescript
interface CommentProps {
	comment: CommentInfo;
	postId: string;
	onReply?: (newComment: CommentInfo) => void;
	depth?: number; // 새로 추가: 댓글 깊이
	isLast?: boolean; // 새로 추가: 마지막 답글인지 여부
}
```

#### Layout Structure

```
┌─ Comment Container
│  ├─ Tree Lines (if depth > 0)
│  │  ├─ Vertical Line (from parent)
│  │  └─ Horizontal Line (to comment)
│  ├─ Comment Content
│  │  ├─ User Info Row
│  │  │  ├─ Profile Image
│  │  │  ├─ Username
│  │  │  ├─ Handle
│  │  │  └─ Timestamp
│  │  ├─ Comment Text
│  │  └─ Action Buttons
│  │     ├─ Like Button (existing)
│  │     ├─ Reply Button
│  │     └─ More Options
│  └─ Replies Container
│     ├─ Collapse/Expand Button
│     └─ Nested Comments (recursive)
```

### 2. Tree Line System

트리 라인 시스템은 다음과 같은 요소들로 구성됩니다:

#### Visual Elements

- **Vertical Line**: 부모 댓글에서 내려오는 세로선
- **Horizontal Line**: 세로선에서 댓글로 연결되는 가로선
- **Corner Line**: 세로선과 가로선을 연결하는 모서리
- **Collapse Button**: 답글을 접고 펼치는 버튼

#### CSS Classes

```css
.comment-tree-line {
	/* 기본 트리 라인 스타일 */
}

.comment-tree-vertical {
	/* 세로선 스타일 */
}

.comment-tree-horizontal {
	/* 가로선 스타일 */
}

.comment-tree-corner {
	/* 모서리 스타일 */
}
```

### 3. Depth Management

댓글 깊이 관리를 위한 시스템:

#### Maximum Depth

- 최대 깊이: 5단계
- 5단계 초과 시 더 이상 들여쓰기 증가하지 않음

#### Depth Calculation

```typescript
const calculateDepth = (comment: CommentInfo, parentDepth: number = 0): number => {
	return Math.min(parentDepth + 1, 5);
};
```

## Data Models

기존 `CommentInfo` 인터페이스는 변경하지 않고 그대로 사용합니다. 추가적인 UI 상태만 컴포넌트 레벨에서 관리합니다.

### UI State Extensions

```typescript
interface CommentUIState {
	depth: number;
	isLast: boolean;
	isCollapsed: boolean;
	showReplyForm: boolean;
}
```

## Error Handling

### Tree Rendering Errors

- 깊이 계산 오류 시 기본값(0) 사용
- 트리 라인 렌더링 실패 시 기본 들여쓰기로 폴백

### State Management Errors

- 답글 로딩 실패 시 사용자에게 재시도 옵션 제공
- 접기/펼치기 상태 오류 시 기본 펼침 상태로 복원

## Testing Strategy

### Unit Tests

1. **Comment Component Tests**
   - 다양한 깊이에서의 렌더링 테스트
   - 트리 라인 표시 로직 테스트
   - 접기/펼치기 기능 테스트

2. **Tree Line Tests**
   - 올바른 위치에 라인 표시 테스트
   - 테마 변경 시 색상 적용 테스트
   - 반응형 레이아웃 테스트

### Integration Tests

1. **Comment Thread Tests**
   - 중첩된 댓글 구조 렌더링 테스트
   - 답글 추가 시 트리 구조 업데이트 테스트
   - 댓글 삭제 시 트리 구조 유지 테스트

### Visual Regression Tests

1. **Layout Tests**
   - 다양한 화면 크기에서의 레이아웃 테스트
   - 라이트/다크 테마에서의 시각적 일관성 테스트
   - 긴 댓글 텍스트 처리 테스트

## Implementation Approach

### Phase 1: Core Tree Structure

1. Comment.svelte 컴포넌트 리팩토링
2. 기본 트리 라인 시스템 구현
3. 깊이 관리 로직 추가

### Phase 2: Visual Enhancements

1. 레딧 스타일 CSS 적용
2. 호버 효과 및 인터랙션 개선
3. 반응형 레이아웃 최적화

### Phase 3: Advanced Features

1. 접기/펼치기 애니메이션
2. 키보드 네비게이션 지원
3. 접근성 개선

## Design Decisions and Rationales

### 1. Recursive Component Structure

**Decision**: Comment.svelte를 재귀적으로 사용하여 중첩 구조 구현
**Rationale**:

- 기존 구조와의 호환성 유지
- 코드 중복 최소화
- 무한 깊이 지원 가능

### 2. CSS-based Tree Lines

**Decision**: SVG 대신 CSS border를 사용한 트리 라인 구현
**Rationale**:

- 더 나은 성능
- 테마 변경 시 쉬운 색상 적용
- 반응형 레이아웃에 더 적합

### 3. Maximum Depth Limitation

**Decision**: 최대 깊이를 5단계로 제한
**Rationale**:

- 모바일 화면에서의 가독성 확보
- 과도한 중첩으로 인한 UI 깨짐 방지
- 레딧과 유사한 UX 제공

### 4. Preserve Existing Functionality

**Decision**: 기존 좋아요 시스템과 API 구조 유지
**Rationale**:

- 백엔드 변경 최소화
- 기존 사용자 경험 유지
- 점진적 개선 가능

## Responsive Design Considerations

### Mobile Layout

- 트리 라인 두께 조정 (모바일에서 더 얇게)
- 터치 친화적인 버튼 크기
- 가로 스크롤 방지를 위한 최대 너비 제한

### Desktop Layout

- 더 넓은 들여쓰기 간격
- 호버 효과 강화
- 키보드 단축키 지원

## Accessibility Features

### Screen Reader Support

- 적절한 ARIA 레이블 추가
- 댓글 깊이 정보 제공
- 접기/펼치기 상태 안내

### Keyboard Navigation

- Tab 키를 통한 댓글 간 이동
- Enter/Space 키를 통한 접기/펼치기
- 화살표 키를 통한 트리 탐색

## Performance Considerations

### Rendering Optimization

- 깊은 중첩 구조에서의 렌더링 성능 최적화
- 가상화를 통한 대량 댓글 처리 (향후 고려사항)
- 트리 라인 CSS 최적화

### Memory Management

- 접힌 댓글의 DOM 요소 최소화
- 이벤트 리스너 정리
- 컴포넌트 언마운트 시 상태 정리
