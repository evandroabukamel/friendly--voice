package app.friendlyvoice.core

/** Finds the Settings language-picker entry for the stored app locale. Pure: tested on the JVM. */
object AppLanguage {

    /**
     * [storedTags] is AppCompat's comma-separated list ("" = system default);
     * [pickerTags] is values/arrays.xml app_language_tags, whose entry 0 is "".
     */
    fun indexOf(storedTags: String, pickerTags: List<String>): Int {
        val first = storedTags.split(',').first().trim().replace('_', '-')
        if (first.isEmpty()) return 0
        pickerTags.indexOfFirst { it.isNotEmpty() && it.equals(first, ignoreCase = true) }
            .takeIf { it >= 0 }?.let { return it }
        val language = first.substringBefore('-').lowercase()
        return pickerTags.indexOfFirst { it.isNotEmpty() && it.substringBefore('-').lowercase() == language }
            .takeIf { it >= 0 } ?: 0
    }
}
