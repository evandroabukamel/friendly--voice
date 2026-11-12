# Métodos nativos são resolvidos pelo nome (Java_app_friendlyvoice_NativeBridge_*).
-keep class app.friendlyvoice.NativeBridge { *; }
-keepclasseswithmembernames class * {
    native <methods>;
}
