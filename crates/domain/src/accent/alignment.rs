//! Alinhamento (Levenshtein ponderado) entre a sequência canônica e a ouvida.
use crate::phonetics::Phoneme;

fn substitution_cost(a: &Phoneme, b: &Phoneme) -> f32 {
    if a == b {
        0.0
    } else if a.symbol().starts_with(b.symbol()) || b.symbol().starts_with(a.symbol()) {
        // t ~ tʃ, d ~ dʒ
        0.3
    } else if (a.is_vowel() || a.is_glide()) == (b.is_vowel() || b.is_glide()) {
        0.6
    } else {
        1.4
    }
}

const GAP: f32 = 1.0;

/// Para cada fonema canônico, devolve os fonemas observados correspondentes:
/// vazio = apagamento; mais de um = inserção agregada ao fonema anterior.
pub fn align(canonical: &[Phoneme], observed: &[Phoneme]) -> Vec<Vec<Phoneme>> {
    let (n, m) = (canonical.len(), observed.len());
    let mut dp = vec![vec![0.0f32; m + 1]; n + 1];
    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i as f32 * GAP;
    }
    for (j, cell) in dp[0].iter_mut().enumerate() {
        *cell = j as f32 * GAP;
    }
    for i in 1..=n {
        for j in 1..=m {
            let sub = dp[i - 1][j - 1] + substitution_cost(&canonical[i - 1], &observed[j - 1]);
            dp[i][j] = sub.min(dp[i - 1][j] + GAP).min(dp[i][j - 1] + GAP);
        }
    }

    let mut result: Vec<Vec<Phoneme>> = vec![Vec::new(); n];
    let mut leading: Vec<Phoneme> = Vec::new();
    let (mut i, mut j) = (n, m);
    // Inserções encontradas no backtrace ficam pendentes até achar o
    // fonema canônico anterior (ao qual se agregam).
    let mut pending: Vec<Phoneme> = Vec::new();
    while i > 0 || j > 0 {
        let eps = 1e-4;
        if i > 0
            && j > 0
            && (dp[i][j]
                - (dp[i - 1][j - 1] + substitution_cost(&canonical[i - 1], &observed[j - 1])))
            .abs()
                < eps
        {
            let mut cell = vec![observed[j - 1].clone()];
            cell.extend(pending.drain(..).rev());
            result[i - 1] = cell;
            i -= 1;
            j -= 1;
        } else if i > 0 && (dp[i][j] - (dp[i - 1][j] + GAP)).abs() < eps {
            let cell: Vec<Phoneme> = pending.drain(..).rev().collect();
            result[i - 1] = cell;
            i -= 1;
        } else {
            pending.push(observed[j - 1].clone());
            j -= 1;
        }
    }
    leading.extend(pending.drain(..).rev());
    if !leading.is_empty() && n > 0 {
        leading.append(&mut result[0]);
        result[0] = leading;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phonetics::parse_ipa;

    fn show(v: &[Vec<Phoneme>]) -> Vec<String> {
        v.iter()
            .map(|c| c.iter().map(Phoneme::symbol).collect::<String>())
            .collect()
    }

    #[test]
    fn aligns_substitutions() {
        let a = align(&parse_ipa("nojte"), &parse_ipa("nojtʃi"));
        assert_eq!(show(&a), ["n", "o", "j", "tʃ", "i"]);
    }

    #[test]
    fn aligns_deletion() {
        let a = align(&parse_ipa("falaɾ"), &parse_ipa("fala"));
        assert_eq!(show(&a), ["f", "a", "l", "a", ""]);
    }

    #[test]
    fn aligns_insertion() {
        let a = align(&parse_ipa("mas"), &parse_ipa("majs"));
        assert_eq!(show(&a), ["m", "aj", "s"]);
    }
}
