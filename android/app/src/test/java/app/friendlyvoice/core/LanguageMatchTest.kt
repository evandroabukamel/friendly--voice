package app.friendlyvoice.core

import org.junit.Assert.assertEquals
import org.junit.Test

class LanguageMatchTest {

    // The codes the core's catalog() lists today (crates/domain/src/language.rs).
    private val core = listOf(
        "ar", "da", "de", "el", "en", "es", "fi", "fr", "he", "hi", "it", "ja",
        "ko", "ms", "nl", "no", "pl", "pt", "ru", "sv", "sw", "tr", "zh",
    )

    private fun best(tag: String, codes: List<String> = core) = LanguageMatch.best(tag, codes)

    @Test
    fun `an exact tag wins over the language prefix`() =
        assertEquals("pt-BR", best("pt-BR", listOf("pt", "pt-BR", "pt-PT")))

    @Test
    fun `tags are compared without case and with underscores`() =
        assertEquals("pt-PT", best("pt_pt", listOf("pt-BR", "pt-PT")))

    @Test
    fun `european portuguese falls back to the portuguese code`() = assertEquals("pt", best("pt-PT"))

    @Test
    fun `new zealand english uses english`() = assertEquals("en", best("en-NZ"))

    @Test
    fun `mexican spanish uses spanish`() = assertEquals("es", best("es-MX"))

    @Test
    fun `traditional chinese uses chinese`() = assertEquals("zh", best("zh-TW"))

    @Test
    fun `norwegian bokmal and the legacy no tag both reach norwegian`() {
        assertEquals("no", best("nb"))
        assertEquals("no", best("nb-NO"))
        assertEquals("no", best("no"))
        assertEquals("nb", best("no", listOf("en", "nb")))
    }

    @Test
    fun `arabic uses arabic`() = assertEquals("ar", best("ar-EG"))

    @Test
    fun `the plain language code is preferred among regional ones`() =
        assertEquals("es", best("es-AR", listOf("es-ES", "es", "es-MX")))

    @Test
    fun `an unsupported language falls back to brazilian portuguese`() {
        assertEquals("pt", best("uk"))
        assertEquals("pt-BR", best("uk", listOf("en", "pt-BR")))
        assertEquals("pt-BR", best("uk", emptyList()))
    }
}
