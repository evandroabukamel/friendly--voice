package app.friendlyvoice.core

import org.junit.Assert.assertEquals
import org.junit.Test

class AppLanguageTest {

    private val tags = listOf("", "pt-BR", "pt-PT", "en-GB", "en-US", "nb", "zh-TW")

    @Test
    fun `no app locale selects System default`() = assertEquals(0, AppLanguage.indexOf("", tags))

    @Test
    fun `an exact tag is found regardless of case`() = assertEquals(2, AppLanguage.indexOf("pt-pt", tags))

    @Test
    fun `only the first of several stored locales counts`() = assertEquals(6, AppLanguage.indexOf("zh-TW,en-US", tags))

    @Test
    fun `a tag without the exact region falls back to the same language`() =
        assertEquals(3, AppLanguage.indexOf("en", tags))

    @Test
    fun `an unknown language selects System default`() = assertEquals(0, AppLanguage.indexOf("ko-KR", tags))
}
