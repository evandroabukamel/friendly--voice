package app.friendlyvoice.audio

import androidx.annotation.StringRes
import java.io.IOException

/** An audio problem whose user-facing text is a translated string resource. */
class AudioError(@StringRes val messageId: Int) : IOException()
