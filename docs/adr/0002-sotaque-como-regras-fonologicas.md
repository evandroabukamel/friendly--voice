# ADR 0002 — Sotaque como regras fonológicas ordenadas no domínio

**Status:** aceito · Sprint 1

## Contexto
A feature precisa considerar o sotaque: pronúncia de letras, término e junção
de palavras. Motores neurais de clonagem copiam bem o timbre, mas tendem a
"normalizar" a pronúncia para o padrão do idioma, e não são explicáveis.

## Decisão
Modelar o sotaque no **domínio** como regras `alvo → substituição / contexto`
aplicadas em ordem (como na fonologia gerativa), em três tipos: segmental,
término de palavra e junção. As regras são aprendidas comparando a forma de
citação (G2P) com os fonemas ouvidos e entregues ao motor como IPA por palavra
(SSML `<phoneme>`).

## Consequências
* + Explicável: o usuário vê cada fenômeno com a frequência medida.
* + Controlável: intensidade do sotaque, edição de regras.
* + Testável sem áudio.
* − Depende de fonemas observados (anotação manual ou reconhecedor fonético).
* − Fenômenos prosódicos finos (duração por fonema, entonação) ficam para
  histórias futuras (US-12, US-13).
