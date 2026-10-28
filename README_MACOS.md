# Guía de Instalación y Gatekeeper en macOS para SoundToOSC

Esta guía explica cómo instalar y abrir **SoundToOSC** en macOS, resolviendo los avisos de seguridad de Gatekeeper, especialmente en **macOS 15 Sequoia** y versiones anteriores.

---

## 📥 Instalación Estándar

1. Descarga el archivo `SoundToOSC-macOS.dmg` desde los releases de GitHub.
2. Haz doble clic en el archivo `.dmg` para montarlo.
3. **Importante:** Arrastra el icono de **SoundToOSC.app** a la carpeta de acceso directo **Applications** (Aplicaciones).  
   *(No abras la aplicación directamente desde el interior del DMG).*
4. Expulsa la imagen de disco DMG.

---

## 🛡️ Cómo abrir SoundToOSC en macOS 15 Sequoia

Dado que SoundToOSC se distribuye con una firma ad-hoc (sin suscripción de pago al Apple Developer Program), macOS Sequoia bloqueará su ejecución la primera vez mostrando este aviso:

> *"SoundToOSC no se puede abrir porque Apple no puede comprobar si contiene software malicioso."*

### Pasos para autorizar la aplicación:
1. Haz doble clic en **SoundToOSC** dentro de `/Applications`. Cuando aparezca el mensaje de bloqueo, pulsa en **Aceptar** (o *Listo*).
2. Abre **Ajustes del Sistema** (*System Settings*) en tu Mac.
3. Entra en el apartado **Privacidad y seguridad** (*Privacy & Security*).
4. Baja hasta la sección **Seguridad**.
5. Verás el siguiente mensaje:  
   `Se ha bloqueado el uso de "SoundToOSC" porque no es de un desarrollador identificado.`
6. Haz clic en el botón **Abrir de todos modos** (*Open Anyway*).
7. Autentícate con tu Touch ID o contraseña de administrador.
8. En la ventana emergente final de confirmación, pulsa en **Abrir**.

---

## ⚡ Solución al error: "La aplicación está dañada y no se puede abrir"

Si macOS muestra el siguiente error:
> *"SoundToOSC.app está dañado y no se puede abrir. Deberías moverlo al basurero."*

Esto ocurre cuando el sistema Gatekeeper de macOS añade el atributo de cuarentena a los archivos descargados de la web y rechaza binarios que no están notarizados por Apple.

Puedes eliminar este bloqueo de cuarentena al instante ejecutando una sola línea en la **Terminal**:

```bash
xattr -cr /Applications/SoundToOSC.app
```

Una vez ejecutado este comando, podrás abrir SoundToOSC con total normalidad desde `/Applications` o Spotlight sin ninguna advertencia adicional.
