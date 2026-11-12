# Definition of Done

Uma história só está **pronta** quando:

1. Todos os critérios de aceitação estão cobertos por testes automatizados
   (unitário no domínio, de caso de uso com fakes, ou de aceitação na CLI).
2. `cargo fmt --check`, `cargo clippy -D warnings` e `cargo test` passam no CI.
3. A regra de dependência da Clean Architecture é respeitada (domínio não
   depende de nada; aplicação só do domínio; infraestrutura implementa portas).
4. Regras de negócio novas vivem no domínio; I/O novo vive atrás de uma porta.
5. Mensagens ao usuário em português, claras e acionáveis.
6. Documentação (README/ADR/backlog) atualizada quando o comportamento muda.
7. Revisada por outra pessoa (PR) antes do merge.

## Definition of Ready
* História no formato *Como… quero… para…*, com critérios de aceitação.
* Estimada pelo time e pequena o bastante para caber em uma sprint.
* Dependências externas (ex.: servidor de voz) identificadas.
