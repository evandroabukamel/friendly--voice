# ADR 0005 — Android reutilizando o núcleo Rust via JNI

**Status:** aceito · Sprint 4

## Contexto
Queremos o app no Android sem duplicar regras de negócio (sotaque, tom,
agenda) em Kotlin, e com build automatizado no CI/CD.

## Decisão
* `crates/bootstrap` concentra a composition root, usada pela CLI e pelo app.
* `crates/mobile` (cdylib) expõe os casos de uso ao Kotlin via JNI. A API é
  "stringly typed" de propósito: entradas simples e saída JSON
  (`{"ok":..,"data"|"error"}`), o que mantém a fronteira FFI mínima e permite
  testar a ponte no host, sem emulador. Panics são capturados na fronteira.
* O app Kotlin (`android/`) contém apenas interface, preferências,
  notificações e o agendador periódico (WorkManager).
* CI: `cargo-ndk` gera as `.so` para `arm64-v8a`, `armeabi-v7a`, `x86_64`
  em `jniLibs/`; Gradle gera os APKs; tags `v*` publicam uma release.

## Alternativas consideradas
* **UniFFI** — gera bindings tipados, mas adiciona um passo de geração de
  código e dependências no build; reavaliar se a API crescer.
* **Reescrever em Kotlin** — duplicaria o modelo de sotaque e a agenda.

## Consequências
* + Uma única implementação das regras, testada uma vez.
* + Mesmo binário de lógica no celular e na CLI.
* − Precisão dos agendamentos no Android limitada a 15 min (WorkManager).
* − Sem servidor neural configurado, o celular toca o esboço de prosódia.
