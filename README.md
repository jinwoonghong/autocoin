# autocoin

Upbit API를 활용한 수동 시작형 자동매매 서비스입니다.

## 핵심 동작

- 앱 실행 후 대시보드 접속 (`/`)
- 대시보드에서 Start/Stop/Reset으로 엔진 제어
- 엔진 상태, 최근 주문, 로그를 실시간 모니터링
- 기본은 Paper mode(모의 실행)로 동작

## 빠른 시작

```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
cp .env.example .env
export $(grep -v '^#' .env | xargs)
uvicorn app.main:app --host 0.0.0.0 --port 8000
```

브라우저에서 `http://localhost:8000` 접속.

## 문서

- 아키텍처: `docs/ARCHITECTURE.md`
- 기술 설계: `docs/TECHNICAL_DESIGN.md`
