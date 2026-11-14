package app.friendlyvoice.core

/**
 * Picks the voice language (one of the core's catalog codes) that best fits
 * the app's locale: exact tag, then the language prefix (es-MX → es), then
 * Brazilian Portuguese, matched the same way. Pure: tested on the JVM.
 */
object LanguageMatch {
    const val FALLBACK = "pt-BR"

    /** Same language under another code (Norwegian Bokmål/Nynorsk, legacy Java codes). */
    private val aliases = mapOf(
        "nb" to listOf("no"),
        "nn" to listOf("no"),
        "no" to listOf("nb"),
        "iw" to listOf("he"),
        "he" to listOf("iw"),
        "in" to listOf("id"),
        "id" to listOf("in"),
    )

    fun best(appTag: String, coreCodes: List<String>): String =
        match(appTag, coreCodes) ?: match(FALLBACK, coreCodes) ?: FALLBACK

    private fun normalize(tag: String) = tag.trim().replace('_', '-').lowercase()

    private fun language(tag: String) = normalize(tag).substringBefore('-')

    private fun match(tag: String, codes: List<String>): String? {
        val wanted = normalize(tag)
        codes.firstOrNull { normalize(it) == wanted }?.let { return it }
        val languages = listOf(language(tag)) + aliases[language(tag)].orEmpty()
        for (lang in languages) {
            val sameLanguage = codes.filter { language(it) == lang }
            sameLanguage.firstOrNull { normalize(it) == lang }?.let { return it }
            sameLanguage.firstOrNull()?.let { return it }
        }
        return null
    }
}
