//! Curated catalog of positive messages in Portuguese.
use friendly_voice_application::ports::{MessageSource, PortError, PortResult};
use friendly_voice_domain::{
    message::{MessageCategory, MotivationalMessage},
    speaker::Speaker,
};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct CuratedMessageCatalog {
    state: AtomicU64,
}

impl CuratedMessageCatalog {
    pub fn with_seed(seed: u64) -> Self {
        Self {
            state: AtomicU64::new(seed | 1),
        }
    }
    pub fn new() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(42);
        Self::with_seed(nanos)
    }
    fn next(&self) -> u64 {
        let mut x = self.state.load(Ordering::Relaxed);
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state.store(x, Ordering::Relaxed);
        x
    }

    pub fn phrases(category: MessageCategory) -> &'static [&'static str] {
        use MessageCategory::*;
        match category {
            Motivation => &[
                "Ei {nome}, você é muito mais forte do que imagina. Vai com tudo hoje!",
                "Um passo de cada vez {nome}. Você já chegou longe e vai chegar mais longe ainda.",
                "Não desiste não {nome}! O que é difícil hoje vira história amanhã.",
                "Acredita em você {nome}, porque eu acredito. Sempre acreditei.",
                "Faz o seu melhor hoje {nome}. O seu melhor sempre é suficiente.",
            ],
            Confidence => &[
                "{nome}, você tem tudo o que precisa dentro de você. Confia!",
                "Lembra de quantas vezes você achou que não ia dar conta {nome}? Deu. E vai dar de novo.",
                "Levanta essa cabeça {nome}. Você é capaz, inteligente e muito querido.",
                "Você não precisa ser perfeito {nome}. Precisa só ser você.",
            ],
            Gratitude => &[
                "Que bom ter você na minha vida {nome}. Obrigado por existir.",
                "{nome} para um minutinho e repara em tudo de bom que você já construiu.",
                "Hoje, eu só queria te dizer: tenho muito orgulho de você, {nome}.",
            ],
            Calm => &[
                "Respira fundo {nome}. Vai ficar tudo bem. Uma coisa de cada vez.",
                "Calma {nome}. Você não precisa resolver tudo agora.",
                "Desaceléra um pouquinho {nome}. Cuidar de você também é importante.",
            ],
            Celebration => &[
                "Parabéns {nome}! Eu sabia que você ia conseguir!",
                "Olha só você {nome}! Comemora essa conquista, você merece!",
                "Que orgulho {nome}! Todo o seu esforço valeu a pena.",
            ],
            GoodMorning => &[
                "Bom dia {nome}! Hoje vai ser um dia lindo, aproveita!",
                "Bom dia meu amor! Levanta, {nome}! Que hoje é dia de coisa boa.",
                "Acorda {nome}! O dia está esperando por você. Vamo lá!",
            ],
        }
    }
}

impl Default for CuratedMessageCatalog {
    fn default() -> Self {
        Self::new()
    }
}

/// Replaces `{nome}`; without a name, removes the vocative naturally.
fn personalize(template: &str, listener: Option<&str>) -> String {
    match listener.map(str::trim).filter(|n| !n.is_empty()) {
        Some(name) => template.replace("{nome}", name),
        None => {
            let s = template
                .replace(", {nome},", ",")
                .replace(", {nome}", "")
                .replace("{nome}, ", "")
                .replace("{nome}", "");
            let mut chars = s.trim().chars();
            chars
                .next()
                .map(|c| c.to_uppercase().collect::<String>() + chars.as_str())
                .unwrap_or_default()
        }
    }
}

impl MessageSource for CuratedMessageCatalog {
    fn pick(
        &self,
        category: MessageCategory,
        _speaker: &Speaker,
        listener: Option<&str>,
    ) -> PortResult<MotivationalMessage> {
        let phrases = Self::phrases(category);
        let template = phrases[(self.next() % phrases.len() as u64) as usize];
        MotivationalMessage::new(&personalize(template, listener), Some(category))
            .map_err(PortError::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn personalizes() {
        assert_eq!(
            personalize("Bom dia, {nome}! Vai.", Some("Ana")),
            "Bom dia, Ana! Vai."
        );
        assert_eq!(personalize("Bom dia, {nome}! Vai.", None), "Bom dia! Vai.");
        assert_eq!(personalize("{nome}, você pode.", None), "Você pode.");
        assert_eq!(
            personalize("Ei, {nome}, você é forte.", None),
            "Ei, você é forte."
        );
        for c in MessageCategory::ALL {
            for p in CuratedMessageCatalog::phrases(c) {
                assert!(!personalize(p, None).contains("{nome}"));
            }
        }
    }
}
