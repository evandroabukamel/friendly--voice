//! # Friendly Voice — Domínio
//!
//! Camada mais interna da Clean Architecture. Contém apenas regras de negócio
//! puras: nenhuma dependência de I/O, banco, rede ou framework.
//!
//! * [`speaker`]   — pessoa cuja voz será usada (com consentimento obrigatório).
//! * [`audio`]     — clipe de áudio e política de qualidade das amostras.
//! * [`phonetics`] — fonemas, palavras e enunciados fonéticos (IPA).
//! * [`accent`]    — modelo de sotaque: pronúncia de letras, término de
//!   palavras e junção entre palavras; analisador e renderizador.
//! * [`voice`]     — amostras, perfil de voz, prosódia e roteiro de fala.
//! * [`message`]   — mensagens motivacionais.
//! * [`schedule`]  — regras de agendamento definidas pelo usuário.

pub mod accent;
pub mod audio;
pub mod error;
pub mod ids;
pub mod message;
pub mod phonetics;
pub mod schedule;
pub mod speaker;
pub mod voice;

pub use error::DomainError;
