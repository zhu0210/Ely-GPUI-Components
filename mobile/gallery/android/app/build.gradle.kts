// The Ely gallery's Android app: gpui-mobile's example host, renamed.
//
// The Rust library is built separately into app/src/main/jniLibs/<abi>/:
//   cd mobile/gallery
//   cargo ndk -t arm64-v8a -P 26 -o android/app/src/main/jniLibs build --release
//   cd android && ./gradlew assembleDebug
//
// The Java classes stay in `dev.gpui.mobile`: gpui-mobile's JNI looks them up by that name.

plugins {
    id("com.android.application")
}

android {
    namespace = "com.elygpui.gallery"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.elygpui.gallery"
        minSdk = 26          // Vulkan 1.0 is mandatory from API 26+
        targetSdk = 34
        versionCode = 1
        versionName = "1.0.0"

        // Tell NativeActivity which .so to load.
        // This must match the cdylib / example output name.
        ndk {
            abiFilters += listOf("arm64-v8a", "x86_64")
        }

        // Forward the library name to the manifest via a placeholder.
        manifestPlaceholders["nativeLibraryName"] = "ely_gallery_mobile"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {
            isDebuggable = true
            isJniDebuggable = true
        }
    }

    // We do NOT use CMake / ndk-build — the native library is compiled
    // externally via cargo-ndk and placed directly into jniLibs.
    //
    // Disable the built-in native build system so Gradle doesn't look for
    // a CMakeLists.txt or Android.mk.
    externalNativeBuild {
        // Intentionally left empty.
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    // Tell Gradle where the pre-built .so files live.
    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }

    packaging {
        // Prevent stripping of the Rust library — cargo already strips in
        // release mode and stripping again can break backtraces.
        jniLibs {
            keepDebugSymbols += listOf(
                "*/arm64-v8a/libely_gallery_mobile.so",
                "*/armeabi-v7a/libely_gallery_mobile.so",
                "*/x86_64/libely_gallery_mobile.so",
                "*/x86/libely_gallery_mobile.so"
            )
        }
    }

    // Lint configuration — relaxed for an example project.
    lint {
        abortOnError = false
        checkReleaseBuilds = false
    }
}

dependencies {
    // AndroidX core for NotificationCompat (used by GpuiNotifications)
    implementation("androidx.core:core:1.12.0")
    // AndroidX SplashScreen compat (used by GpuiActivity to hold splash until native init)
    implementation("androidx.core:core-splashscreen:1.0.1")
    // AndroidX Biometric for BiometricPrompt (used by GpuiAuthActivity)
    implementation("androidx.biometric:biometric:1.1.0")
    // AndroidX Media for MediaSessionCompat (used by GpuiMediaSession for system controls)
    implementation("androidx.media:media:1.7.1")
}
