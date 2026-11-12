//! Native functions of `app.friendlyvoice.NativeBridge` (Kotlin).
//!
//! FFI boundary rules:
//! * Java `null` strings become `None`;
//! * every output is a JSON string (see [`crate::api`]);
//! * panics never cross into the JVM: they become `{"ok":false,...}`.
use crate::api;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jfloat, jstring};
use jni::JNIEnv;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn opt(env: &mut JNIEnv, s: &JString) -> Option<String> {
    if s.is_null() {
        return None;
    }
    env.get_string(s).ok().map(Into::into)
}

fn req(env: &mut JNIEnv, s: &JString) -> String {
    opt(env, s).unwrap_or_default()
}

fn guarded(env: &mut JNIEnv, f: impl FnOnce() -> String) -> jstring {
    let json = catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|_| {
        r#"{"ok":false,"error":"erro interno na biblioteca nativa"}"#.to_string()
    });
    env.new_string(json)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_configure<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    data_dir: JString<'l>,
    engine_url: JString<'l>,
) -> jstring {
    let (dir, url) = (req(&mut env, &data_dir), opt(&mut env, &engine_url));
    guarded(&mut env, || api::configure(dir, url))
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_catalog<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
) -> jstring {
    guarded(&mut env, api::catalog)
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_listSpeakers<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
) -> jstring {
    guarded(&mut env, api::list_speakers)
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_registerSpeaker<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    name: JString<'l>,
    relationship: JString<'l>,
    consent_by: JString<'l>,
) -> jstring {
    let name = req(&mut env, &name);
    let relationship = opt(&mut env, &relationship);
    let consent_by = opt(&mut env, &consent_by);
    guarded(&mut env, || {
        api::register_speaker(name, relationship, consent_by)
    })
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_addSample<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    speaker_id: JString<'l>,
    wav: JByteArray<'l>,
    transcript: JString<'l>,
    observed_ipa: JString<'l>,
) -> jstring {
    let speaker_id = req(&mut env, &speaker_id);
    let bytes = if wav.is_null() {
        Vec::new()
    } else {
        env.convert_byte_array(&wav).unwrap_or_default()
    };
    let transcript = opt(&mut env, &transcript);
    let observed_ipa = opt(&mut env, &observed_ipa);
    guarded(&mut env, || {
        api::add_sample(speaker_id, bytes, transcript, observed_ipa)
    })
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_buildVoice<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    speaker_id: JString<'l>,
) -> jstring {
    let speaker_id = req(&mut env, &speaker_id);
    guarded(&mut env, || api::build_voice(speaker_id))
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_speak<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    speaker_id: JString<'l>,
    category: JString<'l>,
    text: JString<'l>,
    tone: JString<'l>,
    pitch_shift: jfloat,
    listener: JString<'l>,
) -> jstring {
    let speaker_id = req(&mut env, &speaker_id);
    let category = opt(&mut env, &category);
    let text = opt(&mut env, &text);
    let tone = opt(&mut env, &tone);
    let listener = opt(&mut env, &listener);
    guarded(&mut env, || {
        api::speak(speaker_id, category, text, tone, pitch_shift, listener)
    })
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_createSchedule<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    request_json: JString<'l>,
) -> jstring {
    let request = req(&mut env, &request_json);
    guarded(&mut env, || api::create_schedule(request))
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_listSchedules<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
) -> jstring {
    guarded(&mut env, api::list_schedules)
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_removeSchedule<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    id: JString<'l>,
) -> jstring {
    let id = req(&mut env, &id);
    guarded(&mut env, || api::remove_schedule(id))
}

#[no_mangle]
pub extern "system" fn Java_app_friendlyvoice_NativeBridge_runDueSchedules<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    listener: JString<'l>,
) -> jstring {
    let listener = opt(&mut env, &listener);
    guarded(&mut env, || api::run_due_schedules(listener))
}
