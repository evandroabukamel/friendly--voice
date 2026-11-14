package app.friendlyvoice.ui

import android.Manifest
import android.os.Build
import android.os.Bundle
import android.view.View
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.navigation.fragment.NavHostFragment
import androidx.navigation.ui.setupWithNavController
import app.friendlyvoice.R
import app.friendlyvoice.ScheduleWorker
import app.friendlyvoice.databinding.ActivityMainBinding

/** Hosts the three tabs (Alarms, Voices, Settings) and their detail screens. */
class MainActivity : AppCompatActivity() {

    private val notificationPermission =
        registerForActivityResult(ActivityResultContracts.RequestPermission()) { /* optional */ }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        val navHost = supportFragmentManager.findFragmentById(R.id.nav_host) as NavHostFragment
        val navController = navHost.navController
        binding.bottomNav.setupWithNavController(navController)
        // Detail screens (create alarm, import voice) use the whole screen.
        navController.addOnDestinationChangedListener { _, destination, _ ->
            binding.bottomNav.visibility = when (destination.id) {
                R.id.alarmsFragment, R.id.voicesFragment, R.id.settingsFragment -> View.VISIBLE
                else -> View.GONE
            }
        }

        if (savedInstanceState == null) {
            if (Build.VERSION.SDK_INT >= 33) {
                notificationPermission.launch(Manifest.permission.POST_NOTIFICATIONS)
            }
            ScheduleWorker.ensureScheduled(this)
        }
    }
}
