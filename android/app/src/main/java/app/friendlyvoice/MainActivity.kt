package app.friendlyvoice

import android.Manifest
import android.app.Activity
import android.content.Intent
import android.graphics.Typeface
import android.media.MediaPlayer
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.text.InputType
import android.view.View
import android.widget.AdapterView
import android.widget.ArrayAdapter
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.SeekBar
import android.widget.Spinner
import android.widget.TextView
import org.json.JSONObject

/**
 * Single screen: person → samples → voice → message (with tone) → schedule.
 * Every business rule lives in the Rust core; this is UI only (user-facing text in Portuguese).
 */
class MainActivity : Activity() {

    private lateinit var prefs: Prefs
    private lateinit var status: TextView
    private lateinit var speakerSpinner: Spinner
    private lateinit var categorySpinner: Spinner
    private lateinit var toneSpinner: Spinner
    private lateinit var toneHint: TextView
    private lateinit var pitchLabel: TextView
    private lateinit var pitchBar: SeekBar
    private lateinit var schedulesView: TextView

    private var speakerIds: List<String> = emptyList()
    private var categories: List<String> = emptyList()
    private var defaultTones: Map<String, String> = emptyMap()
    private var toneSlugs: List<String?> = listOf(null)
    private var pickedWav: ByteArray? = null
    private var player: MediaPlayer? = null

    private val maxShift = 6

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        prefs = Prefs(this)
        setContentView(buildUi())
        if (Build.VERSION.SDK_INT >= 33) {
            requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), 1)
        }
        configureAndLoad()
        ScheduleWorker.ensureScheduled(this)
    }

    override fun onDestroy() {
        player?.release()
        super.onDestroy()
    }

    // ---------------------------------------------------------------- UI

    private lateinit var root: LinearLayout

    private fun buildUi(): View {
        root = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(dp(16), dp(16), dp(16), dp(32))
        }
        status = TextView(this).apply { setPadding(0, 0, 0, dp(8)) }
        root.addView(status)

        section("Configurações")
        val listener = edit("Seu nome (para personalizar as mensagens)", prefs.listener)
        val engine = edit("URL do servidor de voz neural (opcional)", prefs.engineUrl, InputType.TYPE_TEXT_VARIATION_URI)
        button("Salvar configurações") {
            prefs.listener = listener.text.toString()
            prefs.engineUrl = engine.text.toString()
            configureAndLoad()
        }

        section("1. Pessoa da voz")
        val name = edit("Nome (ex.: Vó Lúcia)")
        val relationship = edit("Relação (ex.: avó)")
        val consent = edit("Quem autorizou o uso da voz (obrigatório)")
        button("Cadastrar pessoa") {
            callNative({ NativeBridge.registerSpeaker(name.text.toString(), relationship.text.toString(), consent.text.toString()) }) {
                show("Pessoa cadastrada: ${it.obj().getString("name")}")
                loadSpeakers()
            }
        }
        speakerSpinner = Spinner(this)
        root.addView(label("Pessoa selecionada"))
        root.addView(speakerSpinner)

        section("2. Amostras de áudio")
        button("Escolher arquivo .wav ou .flac") {
            val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
                addCategory(Intent.CATEGORY_OPENABLE)
                type = "audio/*"
            }
            startActivityForResult(intent, PICK_WAV)
        }
        val transcript = edit("O que é dito no áudio")
        val ipa = edit("Fonemas ouvidos em IPA (opcional, ex.: lejtʃi kẽtʃi)")
        button("Adicionar amostra") {
            val wav = pickedWav ?: return@button show("Escolha um arquivo .wav ou .flac primeiro.")
            val id = selectedSpeaker() ?: return@button
            callNative({ NativeBridge.addSample(id, wav, transcript.text.toString(), ipa.text.toString()) }) {
                val s = it.obj()
                show("Amostra adicionada (%.1fs): /%s/".format(s.getDouble("duration_secs"), s.getString("citation_ipa")))
                pickedWav = null
            }
        }
        button("Gerar voz (timbre + sotaque)") {
            val id = selectedSpeaker() ?: return@button
            callNative({ NativeBridge.buildVoice(id) }) { show(describeVoice(it.obj())) }
        }

        section("3. Mensagem")
        categorySpinner = Spinner(this)
        root.addView(label("Tema"))
        root.addView(categorySpinner)
        val text = edit("Ou um texto seu (opcional)")
        toneSpinner = Spinner(this)
        root.addView(label("Tom de voz"))
        root.addView(toneSpinner)
        toneHint = label("")
        root.addView(toneHint)
        pitchLabel = label("")
        root.addView(pitchLabel)
        pitchBar = SeekBar(this).apply {
            max = maxShift * 2
            progress = maxShift
            setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
                override fun onProgressChanged(bar: SeekBar, value: Int, fromUser: Boolean) = updatePitchLabel()
                override fun onStartTrackingTouch(bar: SeekBar) {}
                override fun onStopTrackingTouch(bar: SeekBar) {}
            })
        }
        root.addView(pitchBar)
        updatePitchLabel()
        categorySpinner.onItemSelectedListener = object : AdapterView.OnItemSelectedListener {
            override fun onItemSelected(p: AdapterView<*>?, v: View?, pos: Int, id: Long) = updateToneHint()
            override fun onNothingSelected(p: AdapterView<*>?) {}
        }
        button("Falar agora") {
            val id = selectedSpeaker() ?: return@button
            callNative({
                NativeBridge.speak(id, selectedCategory(), text.text.toString(), selectedTone(), pitchShift(), prefs.listener)
            }) {
                val m = it.obj()
                show("💬 ${m.getString("text")}\nTom: ${m.getString("tone")}\n/${m.getString("ipa")}/")
                play(m.getString("audio_path"))
            }
        }

        section("4. Agendar (usa o tema, texto e tom acima)")
        val kinds = listOf("diario", "semanal", "intervalo", "uma-vez")
        val kind = Spinner(this).apply {
            adapter = ArrayAdapter(this@MainActivity, android.R.layout.simple_spinner_dropdown_item, kinds)
        }
        root.addView(kind)
        val at = edit("Horário: HH:MM (ou AAAA-MM-DD HH:MM para uma vez)")
        val days = edit("Dias (semanal): seg,qua,sex")
        val every = edit("A cada quantos minutos (intervalo)", null, InputType.TYPE_CLASS_NUMBER)
        val within = edit("Somente entre (intervalo): 09:00-18:00")
        val quiet = edit("Não perturbe: 22:00-07:00")
        button("Agendar") {
            val id = selectedSpeaker() ?: return@button
            val request = JSONObject().apply {
                put("speaker_id", id)
                put("kind", kinds[kind.selectedItemPosition])
                put("at", at.text.toString())
                put("days", days.text.toString())
                every.text.toString().toIntOrNull()?.let { put("every_minutes", it) }
                put("within", within.text.toString())
                put("quiet", quiet.text.toString())
                put("category", selectedCategory())
                put("text", text.text.toString())
                selectedTone()?.let { put("tone", it) }
                put("pitch_shift", pitchShift().toDouble())
            }
            callNative({ NativeBridge.createSchedule(request.toString()) }) {
                show("Agendado! Próxima: ${it.obj().optString("next_due", "—")}")
                loadSchedules()
            }
        }
        schedulesView = TextView(this).apply { setTextIsSelectable(true) }
        root.addView(schedulesView)
        val removeId = edit("Id do agendamento para remover")
        button("Remover agendamento") {
            callNative({ NativeBridge.removeSchedule(removeId.text.toString().trim()) }) {
                show("Removido.")
                loadSchedules()
            }
        }

        return ScrollView(this).apply { addView(root) }
    }

    private fun dp(v: Int): Int = (v * resources.displayMetrics.density).toInt()

    private fun section(title: String) {
        root.addView(TextView(this).apply {
            text = title
            textSize = 18f
            setTypeface(typeface, Typeface.BOLD)
            setPadding(0, dp(20), 0, dp(4))
        })
    }

    private fun label(text: String) = TextView(this).apply { this.text = text }

    private fun edit(hint: String, value: String? = null, type: Int = InputType.TYPE_CLASS_TEXT): EditText {
        val e = EditText(this).apply {
            this.hint = hint
            inputType = type
            setText(value ?: "")
        }
        root.addView(e)
        return e
    }

    private fun button(text: String, onClick: () -> Unit) {
        root.addView(Button(this).apply {
            this.text = text
            setOnClickListener { onClick() }
        })
    }

    private fun show(message: String) {
        status.text = message
    }

    // ------------------------------------------------------------ data

    /** Native calls do I/O and DSP: they run off the UI thread. */
    private fun callNative(call: () -> String, onOk: (NativeResult) -> Unit) {
        show("Processando…")
        Thread {
            val result = try {
                NativeResult(call())
            } catch (e: Exception) {
                NativeResult(JSONObject().put("ok", false).put("error", e.message ?: "erro").toString())
            }
            runOnUiThread { if (result.ok) onOk(result) else show("⚠️ ${result.error}") }
        }.start()
    }

    private fun configureAndLoad() {
        callNative({ NativeBridge.configure(filesDir.absolutePath, prefs.engineUrl) }) {
            show("Motor de voz: ${it.obj().getString("engine")}")
            loadCatalog()
            loadSpeakers()
            loadSchedules()
        }
    }

    private fun loadCatalog() {
        val c = NativeResult(NativeBridge.catalog()).obj()
        val cats = c.getJSONArray("categories")
        categories = (0 until cats.length()).map { cats.getJSONObject(it).getString("slug") }
        defaultTones = (0 until cats.length()).associate {
            cats.getJSONObject(it).getString("slug") to cats.getJSONObject(it).getString("default_tone")
        }
        val tones = c.getJSONArray("tones")
        val toneObjs = (0 until tones.length()).map { tones.getJSONObject(it) }
        toneSlugs = listOf<String?>(null) + toneObjs.map { it.getString("slug") }
        val toneLabels = listOf("Sugerido pelo tema") + toneObjs.map { "${it.getString("slug")} — ${it.getString("description")}" }
        categorySpinner.adapter = ArrayAdapter(this, android.R.layout.simple_spinner_dropdown_item, categories)
        toneSpinner.adapter = ArrayAdapter(this, android.R.layout.simple_spinner_dropdown_item, toneLabels)
        updateToneHint()
    }

    private fun loadSpeakers() {
        callNative({ NativeBridge.listSpeakers() }) {
            val arr = it.arr()
            val items = (0 until arr.length()).map { i -> arr.getJSONObject(i) }
            speakerIds = items.map { s -> s.getString("id") }
            val labels = items.map { s ->
                val voice = if (s.optBoolean("voice_ready")) "voz pronta · ${s.optString("accent")}" else "voz não gerada"
                "${s.getString("name")} ($voice)"
            }
            speakerSpinner.adapter = ArrayAdapter(this, android.R.layout.simple_spinner_dropdown_item, labels)
        }
    }

    private fun loadSchedules() {
        callNative({ NativeBridge.listSchedules() }) {
            val arr = it.arr()
            schedulesView.text = if (arr.length() == 0) {
                "Nenhum agendamento."
            } else {
                (0 until arr.length()).joinToString("\n") { i ->
                    val s = arr.getJSONObject(i)
                    "• ${s.getString("id")} — tom: ${s.optString("tone_label")} — próxima: ${s.optString("next_due", "—")}"
                }
            }
        }
    }

    private fun selectedSpeaker(): String? {
        val pos = speakerSpinner.selectedItemPosition
        if (pos < 0 || pos >= speakerIds.size) {
            show("Cadastre e selecione uma pessoa primeiro.")
            return null
        }
        return speakerIds[pos]
    }

    private fun selectedCategory(): String? = categories.getOrNull(categorySpinner.selectedItemPosition)

    private fun selectedTone(): String? = toneSlugs.getOrNull(toneSpinner.selectedItemPosition)

    private fun pitchShift(): Float = (pitchBar.progress - maxShift).toFloat()

    private fun updatePitchLabel() {
        val v = pitchBar.progress - maxShift
        pitchLabel.text = when {
            v > 0 -> "Ajuste de tom: +$v semitons (mais aguda)"
            v < 0 -> "Ajuste de tom: $v semitons (mais grave)"
            else -> "Ajuste de tom: 0 (altura natural da pessoa)"
        }
    }

    private fun updateToneHint() {
        val suggested = selectedCategory()?.let { defaultTones[it] } ?: "natural"
        toneHint.text = "Tom inicial sugerido para este tema: $suggested"
    }

    private fun describeVoice(v: JSONObject): String {
        val sb = StringBuilder("Voz gerada! Sotaque: ${v.getString("accent")}\n")
        val ev = v.getJSONArray("evidence")
        for (i in 0 until ev.length()) {
            val e = ev.getJSONObject(i)
            val mark = if (e.getBoolean("adopted")) "✔" else "·"
            sb.append("$mark ${e.getInt("realizations")}/${e.getInt("opportunities")} ${e.getString("description")}\n")
        }
        return sb.toString()
    }

    private fun play(path: String) {
        player?.release()
        player = MediaPlayer().apply {
            setDataSource(path)
            prepare()
            start()
        }
    }

    @Deprecated("Plain Activity API (no AndroidX)")
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        val uri: Uri = data?.data ?: return
        if (requestCode == PICK_WAV && resultCode == RESULT_OK) {
            pickedWav = contentResolver.openInputStream(uri)?.use { it.readBytes() }
            show("Arquivo escolhido (${(pickedWav?.size ?: 0) / 1024} KB). Preencha o texto e adicione.")
        }
    }

    companion object {
        private const val PICK_WAV = 42
    }
}
