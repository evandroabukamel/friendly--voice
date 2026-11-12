//! # Friendly Voice — Infraestrutura
//!
//! Adaptadores que implementam as portas da camada de aplicação. Nenhuma
//! regra de negócio vive aqui: apenas tradução entre o mundo externo
//! (arquivos, áudio, HTTP, sistema) e o modelo do domínio.
pub mod audio;
pub mod delivery;
pub mod messages;
pub mod persistence;
pub mod phonetics;
pub mod system;
pub mod voice;
