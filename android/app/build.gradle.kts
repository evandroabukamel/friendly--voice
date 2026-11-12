plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

// Release signing via environment variables (CI/CD). Without them the
// release build is unsigned (app-release-unsigned.apk).
val keystorePath: String? = System.getenv("ANDROID_KEYSTORE_PATH")

android {
    namespace = "app.friendlyvoice"
    compileSdk = 34

    defaultConfig {
        applicationId = "app.friendlyvoice"
        minSdk = 26
        targetSdk = 34
        versionCode = (System.getenv("VERSION_CODE") ?: "1").toInt()
        versionName = System.getenv("VERSION_NAME") ?: "0.1.0"
        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64")
        }
    }

    signingConfigs {
        if (keystorePath != null) {
            create("release") {
                storeFile = file(keystorePath)
                storePassword = System.getenv("ANDROID_KEYSTORE_PASSWORD")
                keyAlias = System.getenv("ANDROID_KEY_ALIAS")
                keyPassword = System.getenv("ANDROID_KEY_PASSWORD")
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro",
            )
            if (keystorePath != null) {
                signingConfig = signingConfigs.getByName("release")
            }
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    lint {
        // The report is published as a CI artifact; it does not block the build.
        abortOnError = false
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.work:work-runtime-ktx:2.9.1")
}

// The Rust library must be built first (scripts/build-android.sh).
val checkNativeLibs by tasks.registering {
    val lib = file("src/main/jniLibs/arm64-v8a/libfriendly_voice_mobile.so")
    doLast {
        if (!lib.exists()) {
            throw GradleException(
                "Native library missing ($lib). Run first: scripts/build-android.sh --rust-only",
            )
        }
    }
}
tasks.named("preBuild") { dependsOn(checkNativeLibs) }
