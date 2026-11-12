# Product Backlog — Friendly Voice

**Visão do produto:** receber mensagens positivas e motivacionais faladas com a
voz (e o sotaque) de alguém querido — na hora em que eu pedir ou nos momentos
que eu definir.

**Persona principal:** *Evandro*, que quer ouvir a voz da avó dizendo "bom dia"
antes do trabalho e um "respira fundo" no meio da tarde.

Priorização: **MoSCoW** · Estimativa: **story points (Fibonacci)** · Histórias
no formato *Como… quero… para…*, com critérios de aceitação em Gherkin.

---

## Épico E1 — Pessoas e consentimento

### US-01 · Cadastrar a pessoa da voz — *Must* · 2 pts · ✅ Sprint 1
> Como usuário, quero cadastrar a pessoa cuja voz será usada, registrando o
> consentimento dela, para usar a voz de forma ética.

```gherkin
Cenário: cadastro sem consentimento
  Dado que informo apenas o nome "Vó"
  Quando tento cadastrar
  Então o cadastro é recusado com "é obrigatório registrar o consentimento"

Cenário: cadastro com consentimento
  Dado que informo o nome "Vó" e que "Vó" autorizou
  Quando cadastro
  Então recebo um id e a declaração de consentimento com data/hora
```

## Épico E2 — Clonagem de voz com sotaque

### US-02 · Enviar arquivos de áudio — *Must* · 5 pts · ✅ Sprint 2
> Como usuário, quero inserir áudios da pessoa para que a voz dela seja gerada.

```gherkin
Cenário: amostra válida
  Dado um WAV de 6s, 16 kHz, com transcrição "Leite quente de noite"
  Quando adiciono a amostra
  Então ela é armazenada com duração, pitch médio, taxa de pausas
  E com a pronúncia de citação /lejte kẽte de nojte/

Cenário: amostra ruim
  Dado um WAV com menos de 2s (ou saturado, silencioso, < 16 kHz)
  Quando adiciono a amostra
  Então ela é rejeitada explicando o motivo
```

### US-03 · Aprender o jeito de pronunciar letras/sons — *Must* · 8 pts · ✅ Sprint 2
> Como usuário, quero que a voz gerada pronuncie as letras como a pessoa
> pronuncia (ex.: "tia" → *tchia*, "festa" → *feshta*, "porta" → *pohta*).

```gherkin
Cenário: palatalização aprendida
  Dado amostras em que a pessoa diz /tʃi/ e /dʒi/ onde a escrita tem "ti" e "di"
  Quando gero a voz
  Então o perfil contém as regras t → tʃ e d → dʒ antes de /i/ com sua frequência
  E fenômenos que a pessoa não realiza (ex.: r retroflexo) não entram no perfil
```

### US-04 · Aprender o término e a junção de palavras — *Must* · 8 pts · ✅ Sprint 2
> Como usuário, quero que a voz termine as palavras e junte as palavras como a
> pessoa faz (ex.: "leite" → *leiti*, "falar" → *falá*, "os amigos" → *uzamigus*).

```gherkin
Cenário: término de palavras
  Dado amostras em que "e"/"o" átonos finais soam /i/ e /u/
  Então o perfil contém as regras de término correspondentes

Cenário: junção de palavras
  Dado amostras em que "s" final antes de vogal vira /z/
  Então o perfil contém a regra de junção sibilant-voicing
  E pontuação (pausa) bloqueia a junção

Cenário: regras que se alimentam
  Dado o perfil com "e final → i" e "t antes de i → tʃ"
  Quando pronuncio "noite"
  Então obtenho /nojtʃi/
```

### US-06 · Pré-visualizar a pronúncia — *Should* · 2 pts · ✅ Sprint 3
> Como usuário, quero ver como um texto será pronunciado com o sotaque da
> pessoa, para conferir antes de ouvir.

### US-09 · Intensidade do sotaque — *Could* · 1 pt · ✅ Sprint 3
> Como usuário, quero ajustar a intensidade (0 a 1) do sotaque aplicado.

### US-10 · Descobrir traços fora do catálogo — *Could* · 3 pts · ✅ Sprint 2
> Como usuário, quero que traços pessoais (ex.: "v" → "b") sejam descobertos
> mesmo que não sejam de um sotaque regional conhecido.

## Épico E3 — Mensagens

### US-05 · Pedir uma mensagem na hora — *Must* · 3 pts · ✅ Sprint 3
> Como usuário, quero pedir uma mensagem agora (por tema ou texto livre) e
> ouvi-la com a voz da pessoa.

```gherkin
Cenário: mensagem por tema personalizada
  Dado que a voz da "Vó" foi gerada e meu nome é "Evandro"
  Quando peço uma mensagem do tema "bom-dia"
  Então recebo um áudio WAV com uma mensagem que contém "Evandro"

Cenário: voz ainda não gerada
  Quando peço uma mensagem
  Então sou orientado a enviar amostras e gerar a voz
```

## Épico E4 — Agendamentos

### US-07 · Agendar mensagens com regras — *Must* · 5 pts · ✅ Sprint 3
> Como usuário, quero agendar mensagens com regras (diária, dias da semana,
> a cada N minutos dentro de uma janela, uma única vez) e horário de silêncio.

```gherkin
Cenário: intervalo com janela e silêncio
  Dado a regra "a cada 60 min entre 09:00-18:00" e silêncio "12:00-14:00"
  Quando calculo os disparos a partir das 10:00
  Então eles ocorrem às 11:00, 14:00, ..., 17:00 e depois 09:00 do dia seguinte

Cenário: regra impossível
  Dado a regra "todo dia 08:00" com silêncio "00:00-23:59"
  Então o agendamento é recusado ("nunca dispara")
```

### US-08 · Disparar agendamentos vencidos — *Must* · 3 pts · ✅ Sprint 3
> Como usuário, quero que as mensagens vencidas sejam faladas uma única vez,
> mesmo que o app tenha ficado desligado.

---

## Próximas histórias (refinamento futuro)

| Id | História | Prioridade | Pts |
|----|----------|-----------|-----|
| US-11 | Servidor de referência (Python) implementando `docs/engine-api.md` com XTTS + wav2vec2-phoneme | Should | 8 |
| US-12 | Alinhamento forçado com timestamps para medir duração por fonema (ritmo silábico vs. acentual) | Could | 8 |
| US-13 | Entonação por tipo de frase (pergunta, exclamação) aprendida das amostras | Could | 5 |
| US-14 | App mobile/desktop com notificação push consumindo os casos de uso | Should | 13 |
| US-15 | Revogar consentimento apaga amostras e voz da pessoa | Must | 3 |
| US-16 | Suporte a MP3/OGG/M4A na importação | Should | 3 |
| US-17 | Evitar repetir a mesma mensagem em sequência (histórico) | Could | 2 |
