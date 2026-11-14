package app.friendlyvoice

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.w3c.dom.Element
import java.io.File
import javax.xml.parsers.DocumentBuilderFactory

/**
 * Translation parity, checked on the JVM by reading each res/values-xx/strings.xml:
 * every supported locale resolves every translatable key of the default
 * (English) file, adds no unknown keys, keeps the same format placeholders,
 * and gives each plural the quantities its language needs.
 *
 * A regional folder may hold only its differences: its keys are merged with
 * its language folder (values-pt for values-pt-rPT) or, for English, with the
 * default values/ folder, the same fallback Android applies at runtime.
 */
class LocaleResourcesTest {

    private class Strings(val strings: Map<String, String>, val plurals: Map<String, Map<String, String>>) {
        val keys: Set<String> get() = strings.keys.map { "string:$it" }.toSet() + plurals.keys.map { "plurals:$it" }
    }

    /** Supported locale folder → (fallback folder, plural rule language). */
    private val locales = mapOf(
        "values-pt-rBR" to ("values-pt" to "pt"),
        "values-pt-rPT" to ("values-pt" to "pt"),
        "values-en-rGB" to ("values" to "en"),
        "values-en-rUS" to ("values" to "en"),
        "values-en-rNZ" to ("values" to "en"),
        "values-en-rAU" to ("values" to "en"),
        "values-es-rES" to ("values-es" to "es"),
        "values-es-rMX" to ("values-es" to "es"),
        "values-es-rAR" to ("values-es" to "es"),
        "values-de" to (null to "de"),
        "values-fr" to (null to "fr"),
        "values-it" to (null to "it"),
        "values-zh-rCN" to (null to "zh"),
        "values-zh-rTW" to (null to "zh"),
        "values-ja" to (null to "ja"),
        "values-ru" to (null to "ru"),
        "values-uk" to (null to "uk"),
        "values-tr" to (null to "tr"),
        "values-ar" to (null to "ar"),
        "values-fi" to (null to "fi"),
        "values-nb" to (null to "nb"),
        "values-no" to (null to "no"),
        "values-pt" to (null to "pt"),
        "values-es" to (null to "es"),
    )

    /** CLDR plural categories each language needs for integers. */
    private fun requiredQuantities(language: String): Set<String> = when (language) {
        "ar" -> setOf("zero", "one", "two", "few", "many", "other")
        "ru", "uk" -> setOf("one", "few", "many", "other")
        "ja", "zh" -> setOf("other")
        else -> setOf("one", "other")
    }

    private val res: File = listOf(File("src/main/res"), File("app/src/main/res")).first { it.isDirectory }

    private fun load(folder: String): Strings {
        val file = File(res, "$folder/strings.xml")
        if (!file.isFile) return Strings(emptyMap(), emptyMap())
        val root = DocumentBuilderFactory.newInstance().newDocumentBuilder().parse(file).documentElement
        val strings = mutableMapOf<String, String>()
        val plurals = mutableMapOf<String, Map<String, String>>()
        val children = root.childNodes
        for (i in 0 until children.length) {
            val e = children.item(i) as? Element ?: continue
            if (e.getAttribute("translatable") == "false") continue
            when (e.tagName) {
                "string" -> strings[e.getAttribute("name")] = e.textContent
                "plurals" -> {
                    val items = e.getElementsByTagName("item")
                    plurals[e.getAttribute("name")] = (0 until items.length)
                        .map { items.item(it) as Element }
                        .associate { it.getAttribute("quantity") to it.textContent }
                }
            }
        }
        return Strings(strings, plurals)
    }

    private val default = load("values")

    private val placeholder = Regex("""%(\d+\$)?[-#+ 0,(]*\d*(\.\d+)?[a-zA-Z]""")
    private fun placeholders(text: String) = placeholder.findAll(text).map { it.value }.toSet()

    @Test
    fun `every supported locale has its folder`() {
        for (folder in locales.keys) {
            assertTrue("missing $folder/strings.xml", File(res, "$folder/strings.xml").isFile)
        }
    }

    @Test
    fun `every locale resolves exactly the default keys`() {
        for ((folder, rule) in locales) {
            val own = load(folder)
            val unknown = own.keys - default.keys
            assertTrue("$folder has keys that the default file lacks (or marks untranslatable): $unknown", unknown.isEmpty())
            val merged = own.keys + (rule.first?.let { load(it).keys } ?: emptySet())
            assertEquals("$folder is missing keys", emptySet<String>(), default.keys - merged)
        }
    }

    @Test
    fun `translations keep the same placeholders`() {
        for (folder in locales.keys) {
            val own = load(folder)
            for ((name, text) in own.strings) {
                assertEquals("$folder/$name", placeholders(default.strings.getValue(name)), placeholders(text))
            }
            for ((name, items) in own.plurals) {
                val allowed = placeholders(default.plurals.getValue(name).getValue("other"))
                assertEquals("$folder/$name other", allowed, placeholders(items["other"].orEmpty()))
                for ((quantity, text) in items) {
                    assertTrue("$folder/$name $quantity uses unknown placeholders", allowed.containsAll(placeholders(text)))
                }
            }
        }
    }

    @Test
    fun `plurals have the quantities their language needs`() {
        for ((folder, rule) in locales) {
            val own = load(folder)
            val fallback = rule.first?.let { load(it) }
            for (name in default.plurals.keys) {
                val items = own.plurals[name] ?: fallback?.plurals?.get(name) ?: continue
                val missing = requiredQuantities(rule.second) - items.keys
                assertTrue("$folder/$name lacks $missing", missing.isEmpty())
            }
        }
    }
}
