package app.friendlyvoice.core

import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.CoroutineExceptionHandler
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.advanceTimeBy
import kotlinx.coroutines.test.advanceUntilIdle
import kotlinx.coroutines.test.runCurrent
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

@OptIn(ExperimentalCoroutinesApi::class)
class FlowControlTest {

    // ------------------------------------------------------------ SingleFlight

    @Test
    fun `a second launch while the first runs is ignored`() = runTest {
        val flight = SingleFlight()
        val release = CompletableDeferred<Unit>()
        var runs = 0

        assertTrue(flight.launch(this) { runs++; release.await() })
        runCurrent()
        assertTrue(flight.running.value)
        assertFalse(flight.launch(this) { runs++ })
        assertFalse(flight.launch(this) { runs++ })

        release.complete(Unit)
        advanceUntilIdle()
        assertEquals(1, runs)
        assertFalse(flight.running.value)
    }

    @Test
    fun `a new launch is accepted once the previous one finished`() = runTest {
        val flight = SingleFlight()
        var runs = 0

        flight.launch(this) { runs++ }
        advanceUntilIdle()
        flight.launch(this) { runs++ }
        advanceUntilIdle()

        assertEquals(2, runs)
    }

    @Test
    fun `the guard is released when the work fails`() = runTest {
        val flight = SingleFlight()
        val failing = CoroutineScope(
            StandardTestDispatcher(testScheduler) + SupervisorJob() + CoroutineExceptionHandler { _, _ -> },
        )
        flight.launch(failing) { error("server down") }
        advanceUntilIdle()
        assertFalse(flight.running.value)
    }

    // --------------------------------------------------------- ToggleCoalescer

    private class Recorder {
        val sent = mutableListOf<Pair<String, Boolean>>()
        var accept = true
        suspend fun send(id: String, enabled: Boolean): Boolean {
            sent += id to enabled
            return accept
        }
    }

    @Test
    fun `rapid toggles send only the last state once`() = runTest {
        val r = Recorder()
        val c = ToggleCoalescer(this, 400) { id, on -> r.send(id, on) }
        c.markPersisted("a", false)

        c.request("a", true)
        advanceTimeBy(100)
        c.request("a", false)
        advanceTimeBy(100)
        c.request("a", true)
        advanceTimeBy(399)
        assertEquals(emptyList<Pair<String, Boolean>>(), r.sent)

        advanceUntilIdle()
        assertEquals(listOf("a" to true), r.sent)
    }

    @Test
    fun `toggling back to the persisted state sends nothing`() = runTest {
        val r = Recorder()
        val c = ToggleCoalescer(this, 400) { id, on -> r.send(id, on) }
        c.markPersisted("a", true)

        c.request("a", false)
        c.request("a", true)
        advanceUntilIdle()

        assertEquals(emptyList<Pair<String, Boolean>>(), r.sent)
    }

    @Test
    fun `alarms are debounced independently`() = runTest {
        val r = Recorder()
        val c = ToggleCoalescer(this, 400) { id, on -> r.send(id, on) }
        c.markPersisted("a", false)
        c.markPersisted("b", true)

        c.request("a", true)
        c.request("b", false)
        advanceUntilIdle()

        assertEquals(setOf("a" to true, "b" to false), r.sent.toSet())
    }

    @Test
    fun `a successful send becomes the persisted state`() = runTest {
        val r = Recorder()
        val c = ToggleCoalescer(this, 400) { id, on -> r.send(id, on) }
        c.markPersisted("a", false)

        c.request("a", true)
        advanceUntilIdle()
        c.request("a", true)
        advanceUntilIdle()

        assertEquals(listOf("a" to true), r.sent)
    }

    @Test
    fun `a failed send keeps the old persisted state`() = runTest {
        val r = Recorder().apply { accept = false }
        val c = ToggleCoalescer(this, 400) { id, on -> r.send(id, on) }
        c.markPersisted("a", false)

        c.request("a", true)
        advanceUntilIdle()
        assertEquals(false, c.persisted("a"))
    }

    @Test
    fun `the pending state is visible until it is sent`() = runTest {
        val r = Recorder()
        val c = ToggleCoalescer(this, 400) { id, on -> r.send(id, on) }
        c.markPersisted("a", false)

        c.request("a", true)
        assertEquals(true, c.pending("a"))
        advanceUntilIdle()
        assertEquals(null, c.pending("a"))
    }
}
