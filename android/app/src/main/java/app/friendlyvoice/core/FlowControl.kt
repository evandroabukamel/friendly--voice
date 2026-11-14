package app.friendlyvoice.core

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

/**
 * Runs at most one job at a time: a launch while one is running is ignored.
 * Guards every server-touching action (create voice, play now, save) so
 * repeated taps never reach the back-end twice.
 */
class SingleFlight {
    private val _running = MutableStateFlow(false)

    /** True while a job runs; the UI disables its button with it. */
    val running: StateFlow<Boolean> = _running.asStateFlow()

    /** Starts [block] unless one is already running. Returns whether it started. */
    fun launch(scope: CoroutineScope, block: suspend () -> Unit): Boolean {
        if (!_running.compareAndSet(expect = false, update = true)) return false
        scope.launch {
            try {
                block()
            } finally {
                _running.value = false
            }
        }
        return true
    }
}

/**
 * Coalesces rapid ON/OFF toggles per alarm: waits [delayMillis] after the last
 * toggle, then sends the last state only if it differs from the persisted one.
 * Must be used from a single thread (the main thread / one test dispatcher).
 */
class ToggleCoalescer(
    private val scope: CoroutineScope,
    private val delayMillis: Long,
    /** Persists the state; returns true when the back-end accepted it. */
    private val send: suspend (id: String, enabled: Boolean) -> Boolean,
) {
    private val jobs = mutableMapOf<String, Job>()
    private val wanted = mutableMapOf<String, Boolean>()
    private val persistedStates = mutableMapOf<String, Boolean>()

    /** Records the state the back-end holds (after loading the list). */
    fun markPersisted(id: String, enabled: Boolean) {
        persistedStates[id] = enabled
    }

    fun persisted(id: String): Boolean? = persistedStates[id]

    /** The state the user asked for that is not sent yet, if any. */
    fun pending(id: String): Boolean? = wanted[id]

    fun request(id: String, enabled: Boolean) {
        jobs.remove(id)?.cancel()
        wanted[id] = enabled
        jobs[id] = scope.launch {
            delay(delayMillis)
            jobs.remove(id)
            wanted.remove(id)
            if (persistedStates[id] != enabled && send(id, enabled)) {
                persistedStates[id] = enabled
            }
        }
    }
}
