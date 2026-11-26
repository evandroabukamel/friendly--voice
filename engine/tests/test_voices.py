"""Voice deletion: the store and the DELETE endpoint (no model weights needed)."""

from pathlib import Path

import pytest

from friendly_voice_engine.voices import CONDITIONALS, VoiceMeta, VoiceStore


class FakeConditionals:
    def save(self, path: Path) -> None:
        Path(path).write_bytes(b"conds")


def saved_voice(store: VoiceStore, voice_id: str = "a1b2c3d4e5") -> str:
    meta = VoiceMeta(voice_id=voice_id, name="Mãe", language="pt-BR", reference_count=1, reference_seconds=12.0)
    store.save(meta, FakeConditionals(), b"RIFF")
    return voice_id


def test_delete_removes_the_voice_folder(tmp_path):
    store = VoiceStore(tmp_path)
    voice_id = saved_voice(store)
    assert (tmp_path / voice_id / CONDITIONALS).exists()
    assert store.delete(voice_id) is True
    assert not (tmp_path / voice_id).exists()
    assert store.delete(voice_id) is False, "idempotent: already gone"


def test_delete_refuses_ids_outside_the_store(tmp_path):
    store = VoiceStore(tmp_path / "voices")
    (tmp_path / "keep").mkdir()
    for bad in ["../keep", "..", "local:spk-1", ""]:
        assert store.delete(bad) is False
    assert (tmp_path / "keep").exists()


class FakeEngine:
    def __init__(self, store: VoiceStore):
        self.voices = store

    def forget(self, voice_id: str) -> bool:
        return self.voices.delete(voice_id)


def test_delete_endpoint(tmp_path):
    pytest.importorskip("httpx")
    from fastapi.testclient import TestClient

    from friendly_voice_engine.server import create_app

    store = VoiceStore(tmp_path)
    voice_id = saved_voice(store)
    client = TestClient(create_app(engine=FakeEngine(store)))
    assert client.delete(f"/v1/voices/{voice_id}").status_code == 204
    gone = client.delete(f"/v1/voices/{voice_id}")
    assert gone.status_code == 404
    assert "unknown voice_id" in gone.json()["detail"]
