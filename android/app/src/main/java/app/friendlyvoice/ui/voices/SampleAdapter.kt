package app.friendlyvoice.ui.voices

import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.DiffUtil
import androidx.recyclerview.widget.ListAdapter
import androidx.recyclerview.widget.RecyclerView
import app.friendlyvoice.R
import app.friendlyvoice.core.Sample
import app.friendlyvoice.databinding.MoleculeSampleRowBinding

/** Recordings of a voice (layout/molecule_sample_row.xml). */
class SampleAdapter : ListAdapter<Sample, SampleAdapter.Holder>(Diff) {

    class Holder(val binding: MoleculeSampleRowBinding) : RecyclerView.ViewHolder(binding.root)

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int) =
        Holder(MoleculeSampleRowBinding.inflate(LayoutInflater.from(parent.context), parent, false))

    override fun onBindViewHolder(holder: Holder, position: Int) {
        val item = getItem(position)
        val b = holder.binding
        val res = b.root.resources
        b.sampleTitle.text = res.getString(R.string.sample_title, position + 1)
        b.sampleDetail.text = res.getString(R.string.sample_duration, item.durationSecs)
    }

    private object Diff : DiffUtil.ItemCallback<Sample>() {
        override fun areItemsTheSame(a: Sample, b: Sample) = a.id == b.id
        override fun areContentsTheSame(a: Sample, b: Sample) = a == b
    }
}
