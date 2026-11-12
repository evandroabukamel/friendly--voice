# Friendly Voice 🗣️💛

Mensagens positivas e motivacionais **com a voz — e o sotaque — de alguém
querido**. Peça uma mensagem na hora ou agende com as suas próprias regras.

Escrito em **Rust**, seguindo **Clean Architecture** e **SOLID**, desenvolvido
de forma **ágil** (backlog, sprints, DoD — veja [`docs/agile`](docs/agile)).

## Destaques

* **Clonagem de voz a partir de arquivos de áudio**, com consentimento obrigatório.
* **Sotaque aprendido das gravações**, considerando:
  * o **jeito de pronunciar certas letras** — "tia" → *tchia*, "festa" → *feshta*,
    "porta" → *pohta* / *poɻta*, "rato" → *hato*, "mulher" → *muié*;
  * o **término das palavras** — "leite" → *leiti*, "gato" → *gatu*, "falar" → *falá*;
  * a **junção de palavras** — "os amigos" → *uzamigus*, "casa amarela" →
    *casamarela*, "mar azul" → *maɾazul*, "de outro" → *djoutro*.
* Relatório explicável: cada fenômeno com a frequência medida nas amostras e o
  sotaque regional mais próximo (Carioca, Paulistano, Mineiro, Nordestino,
  Gaúcho, Caipira, Português europeu) — além de traços pessoais descobertos.
* **Tom de voz por mensagem**: natural, carinhoso, animado, calmo, firme ou
  sussurrado, mais um ajuste fino de altura (±6 semitons). Cada tema já sugere
  um tom inicial (ex.: calma → calmo), que o usuário pode trocar.
* Mensagens por tema (motivação, confiança, gratidão, calma, celebração,
  bom-dia), personalizadas com o seu nome.
* Agendamentos: diário, dias da semana, a cada N minutos dentro de uma janela,
  uma única vez; horário de silêncio; disparo único se o app esteve desligado.

## Começando

```bash
cargo build --release
alias fv=./target/release/friendly-voice
export FRIENDLY_VOICE_ME="Evandro"       # seu nome nas mensagens
export FRIENDLY_VOICE_PLAYER="aplay"     # opcional: tocar o áudio (afplay no macOS)

# 1. Cadastre a pessoa (o consentimento é obrigatório)
fv pessoa adicionar --nome "Vó Lúcia" --relacao avó --consentimento-de "Lúcia"
#   Pessoa cadastrada: Vó Lúcia (id: spk-...)

# 2. Envie áudios .wav (≥ 2s, ≥ 16 kHz). Ao lado de cada áudio, opcionalmente:
#    audio.txt → o que foi dito      audio.ipa → o que foi ouvido (IPA)
fv amostra adicionar --pessoa spk-... --audio gravacao1.wav
fv amostra adicionar --pessoa spk-... --audio gravacao2.wav \
   --texto "Os amigos de verdade vão falar com você" \
   --fonemas "uz amiguʃ dʒi vehdadʒi vãw fala kũ vose"

# 3. Gere a voz (timbre + sotaque + prosódia)
fv voz gerar --pessoa spk-...

# 4. Confira a pronúncia e peça mensagens
fv pronuncia --pessoa spk-... "Você é forte, mas às vezes esquece."
#   Forma de citação: /vose ɛ foɾte mas as vezes eskese/
#   Com sotaque:      /vose ɛ fohtʃi maz aʃ veziz eʃkesi/
fv falar --pessoa spk-... --tema bom-dia
fv falar --pessoa spk-... --texto "Respira fundo, vai dar tudo certo." --intensidade 0.5
fv tons                                                     # lista os tons de voz
fv falar --pessoa spk-... --tema confianca --tom carinhoso --ajuste-tom -1

# 5. Agende
fv agenda criar --pessoa spk-... --diario 07:30 --tema bom-dia
fv agenda criar --pessoa spk-... --semanal seg,qua,sex@12:00 --tema confianca
fv agenda criar --pessoa spk-... --a-cada 120 --entre 09:00-18:00 --tema calma --silencio 12:00-13:00
fv agenda criar --pessoa spk-... --em "2026-12-24 20:00" --texto "Feliz Natal!" --tom animado
fv agenda criar --pessoa spk-... --diario 22:30 --tema gratidao --tom sussurrado
fv agenda listar
fv agenda executar --continuo      # deixe rodando (ou chame via cron sem --continuo)
```

Os áudios gerados e uma nota com o texto, a pronúncia e o SSML ficam em
`.friendly-voice/saida/`.

### Motor de voz neural

Sem configuração, o app usa um **motor local de esboço de prosódia**: gera um
áudio com o ritmo e a melodia da pessoa (útil para desenvolver e testar), mas
**não** reproduz o timbre. Para o timbre real, rode um servidor que implemente
o [contrato HTTP](docs/engine-api.md) (ex.: XTTS v2 + wav2vec2-phoneme) e:

```bash
cargo build --release --features http-engine
fv --motor-url http://localhost:8020 voz gerar --pessoa spk-...
```

Com o servidor, a transcrição e os fonemas das amostras são obtidos
automaticamente (não é preciso `.txt`/`.ipa`).

## Arquitetura

```
crates/
├── domain/          regras puras: Speaker, AudioClip, fonética, sotaque, agenda
├── application/     casos de uso + portas (traits)
├── infrastructure/  WAV, DSP, G2P pt-BR, JSON, motores de voz, catálogo, saída
└── cli/             interface + composition root
```

Detalhes em [`docs/architecture.md`](docs/architecture.md) e decisões em
[`docs/adr`](docs/adr).

## Desenvolvimento

```bash
cargo test --workspace --all-features    # unitários, casos de uso e aceitação
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all
```

## Uso responsável

Use apenas vozes de pessoas que autorizaram. Os dados ficam locais na pasta
`--dados`. O app é para mensagens pessoais e privadas.
