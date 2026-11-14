package app.friendlyvoice.ui.voices

import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.DiffUtil
import androidx.recyclerview.widget.ListAdapter
import androidx.recyclerview.widget.RecyclerView
import app.friendlyvoice.R
import app.friendlyvoice.core.Speaker
import app.friendlyvoice.databinding.MoleculeVoiceCardBinding
import kotlin.math.roundToInt

/** Voice cards (layout/molecule_voice_card.xml). Tap opens Import voice. */
class VoiceAdapter(
    private val onOpen: (Speaker) -> Unit,
) : ListAdapter<Speaker, VoiceAdapter.Holder>(Diff) {

    /** Language code → display name, from the catalog. */
    var languageNames: Map<String, String> = emptyMap()

    class Holder(val binding: MoleculeVoiceCardBinding) : RecyclerView.ViewHolder(binding.root)

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int) =
        Holder(MoleculeVoiceCardBinding.inflate(LayoutInflater.from(parent.context), parent, false))

    override fun onBindViewHolder(holder: Holder, position: Int) {
        val item = getItem(position)
        val b = holder.binding
        val res = b.root.resources
        b.voiceName.text = item.name
        b.voiceLanguage.text = languageNames[item.language] ?: item.language
        b.voiceStatus.setText(if (item.voiceReady) R.string.voice_status_ready else R.string.voice_status_not_created)
        val secs = item.totalSampleSecs.roundToInt()
        b.voiceSamples.text = res.getString(
            R.string.samples_count_and_secs,
            res.getQuantityString(R.plurals.samples_count, item.sampleCount, item.sampleCount),
            res.getQuantityString(R.plurals.seconds, secs, secs),
        )
        b.root.setOnClickListener { onOpen(item) }
    }

    private object Diff : DiffUtil.ItemCallback<Speaker>() {
        override fun areItemsTheSame(a: Speaker, b: Speaker) = a.id == b.id
        override fun areContentsTheSame(a: Speaker, b: Speaker) = a == b
    }
}
