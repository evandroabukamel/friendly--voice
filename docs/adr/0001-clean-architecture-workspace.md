# ADR 0001 — Clean Architecture com um crate por camada

**Status:** aceito · Sprint 0

## Contexto
Precisamos seguir Clean Architecture e SOLID, e trocar motores de voz e
armazenamento sem reescrever regras de negócio.

## Decisão
Workspace Cargo com `domain`, `application`, `infrastructure` e `cli`. As
dependências entre crates tornam a regra de dependência **verificada pelo
compilador**. Injeção de dependência manual via `Arc<dyn Trait>` em uma única
composition root.

## Consequências
* + Violações de camada viram erro de compilação.
* + Casos de uso testados com fakes, sem I/O.
* − Algum *boilerplate* de construtores.
