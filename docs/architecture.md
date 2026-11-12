# Arquitetura

## Camadas (Clean Architecture)

```
┌─────────────────────────────────────────────────────────────┐
│ cli  (apresentação + composition root)                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ infrastructure  (adaptadores: WAV, DSP, G2P, JSON,    │  │
│  │                  HTTP, catálogo, saída, relógio)      │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │ application  (casos de uso + portas/traits)     │  │  │
│  │  │  ┌───────────────────────────────────────────┐  │  │  │
│  │  │  │ domain  (entidades, regras, sotaque)      │  │  │  │
│  │  │  └───────────────────────────────────────────┘  │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
          as dependências apontam sempre para dentro →
```

A **regra de dependência** é garantida pelo compilador: cada camada é um crate
e o `Cargo.toml` de `domain` não depende de nenhum outro crate do projeto;
`application` depende apenas de `domain`.

| Crate | Responsabilidade | Depende de |
|-------|------------------|-----------|
| `friendly-voice-domain` | Entidades, objetos de valor, serviços de domínio (sotaque, agenda) | `chrono`, `serde`, `thiserror` |
| `friendly-voice-application` | Casos de uso e **portas** (traits) | `domain` |
| `friendly-voice-infrastructure` | Implementações das portas | `domain`, `application`, `hound`, `serde_json`, (`ureq`) |
| `friendly-voice` (cli) | Interface, *wiring* das dependências | todos |

## Fluxo da clonagem de voz com sotaque

```
 WAV ──► WavDecoder ──► AudioClip ──► SampleQualityPolicy (domínio)
                              │
                              ├──► DspAcousticAnalyzer ──► pitch, pausas
 texto ──► PortugueseG2p ──► forma de citação  /lejte kẽte/
 fonemas ouvidos (IPA ou PhoneRecognizer)  ──► /lejtʃi kẽtʃi/
                              │
                              ▼
                     AccentAnalyzer (domínio)
       alinhamento + oportunidades × realizações por fenômeno
                              │
           AccentProfile (regras ordenadas + confiança)
                              │
 texto novo ─► G2P ─► AccentRenderer ─► SSML <phoneme ipa> ─► motor neural ─► WAV
```

O timbre (cor da voz) depende de um modelo neural, por isso fica atrás da porta
`VoiceCloningEngine`/`SpeechSynthesizer`. **O sotaque é modelado no domínio**,
de forma explicável e testável, e chega ao motor como pronúncia explícita
(IPA por palavra) — assim o sotaque é preservado mesmo que o motor neural
"puxe" para uma pronúncia padrão.

## Modelo de sotaque

Cada fenômeno é uma regra `alvo → substituição / contexto`:

| Aspecto pedido | Tipo | Exemplos de regras do catálogo |
|---|---|---|
| Jeito de pronunciar certas letras | `Segmental` | `t`→`tʃ` antes de /i/, `s`→`ʃ` em coda, `ɾ`→`h`/`ɻ` em coda, `x`→`h`/`r`, `ʎ`→`j`, vogais pretônicas abertas |
| Término de palavras | `WordEnding` | `e`→`i` e `o`→`u` átonos finais, `a`→`ɐ`, `e`→`ɨ`, queda do `r` final |
| Junção de palavras | `Juncture` | `s`+vogal→`z`, ligação do `r` (tepe), elisão de `a`, `i`→semivogal, degeminação `s`+`s` |

Contextos: posição (início, fim, coda, ataque, vogal final, pretônica),
fonema anterior/seguinte (exato, conjunto ou classe natural), tonicidade e
bloqueio diante de palavra iniciada por vogal.

**Aprendizado** (`AccentAnalyzer`):
1. Alinha (Levenshtein ponderado) a forma de citação com os fonemas ouvidos.
2. Percorre o catálogo **em ordem**; para cada regra conta oportunidades
   (contexto satisfeito) e realizações (a pessoa de fato produziu a
   substituição). Realizações são aplicadas à derivação, alimentando as regras
   seguintes ("noite": `e→i` cria contexto para `t→tʃ`).
3. Evidência mascarada por junção não penaliza regras de fim de palavra.
4. Adota regras com oportunidades ≥ 2 e taxa ≥ 50%; guarda a taxa como
   confiança (usada pela **intensidade** do sotaque).
5. Descobre substituições recorrentes fora do catálogo (`learned:v>b@WordInitial`).
6. Rotula o perfil com o preset regional mais próximo (Jaccard).

## Tom de voz

`VoiceTone` (domínio) = estilo (`ToneStyle`) + deslocamento de altura em
semitons. Cada estilo é um `ToneAdjustment` (semitons, fator de velocidade,
expressividade melódica, volume, voz soprosa) aplicado **sobre** a prosódia
natural da pessoa — a voz continua sendo a dela, só muda a "cor". O tema da
mensagem sugere o tom inicial (`ToneStyle::default_for`), que o usuário
pode sobrescrever em cada pedido ou agendamento.

## SOLID na prática

* **S** — um caso de uso por arquivo (`RegisterSpeaker`, `AddVoiceSample`,
  `BuildVoiceProfile`, `SpeakMessage`, ...); análise, renderização e
  alinhamento do sotaque em módulos separados.
* **O** — novos sotaques = novas regras no catálogo (dados), sem mudar o
  analisador; novos motores/armazenamentos = novos adaptadores.
* **L** — qualquer implementação de `SpeechSynthesizer` (sketch local, HTTP,
  fake de teste) é intercambiável.
* **I** — portas pequenas: `Transcriber` e `PhoneRecognizer` são opcionais e
  separadas; repositórios por agregado.
* **D** — casos de uso recebem `Arc<dyn Porta>`; só a composition root
  (`crates/cli/src/container.rs`) conhece classes concretas.
