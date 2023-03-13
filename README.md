![Redstone Dust](rsdust.webp)

# Redstoneware

```
⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠
!!!!!! ATTENTION !!!!!!

**The project is only for research purposes. DO NOT run it on your
actual system, or anyone else's system for that matter...**

I'm not responsible for whatever clownery happens in the Documents folder.
You have been warned.

⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠
```

## About

This was just an experiment to see if I can bypass Windows Defender AV.
~~Also, sort of a learning project, since I haven't worked with
anything from `futures` before in Rust.~~ It doesn't use `futures` rn
because it was a test project and I just wanted to get working code
before I improved anything. **Might add "streams" functionality later.**

If there's one thing I learned from this project, it's that **you should
probably turn on "Controlled Folder Access" on Windows, despite how
annoying the popups may be** for each new app. This app got a **1/67 on VirusTotal**,
(caught by Google) and the only real other vendor that detected it was VT's in-house
Jujubox behavioral analysis. Jujubox absolutely carried, so props to VirusTotal
for that, but most people don't even check the Behaviors tab, and they'd probably
trust the huge "1/67" widget. Windows Defender isn't really gonna help you against
these "new" threats, but securing your Documents folder with Windows's Ransomware
Protection (the setting is called Controlled Folder Access) definitely helps prevent
script kiddies ~~(like me <3)~~ from writing simple stuff like this.

So, remember, everyone...

>Stay safe, stay online, and please, attempt to stay private!
>
>\- Someone, probably a Chief Security Fanatic.

## Manual

Redstoneware locks up all the files
<sub>~~(unless it encounters something huge, in which it'll probably crash or
something after trying to load it into memory kek)~~</sub>
in the user's Documents folder via
the XChaCha20 algorithm. It writes the decryption key to `keys.txt` in the
target folder root. Then, when decrypting, it checks for a `keys.txt`, and
reads the key from there to decrypt the files.

When starting the program, you're prompted for the mode you want to run it in.
If your input begins with `E`, it encrypts. If your input begins with `D`, it
decrypts. Otherwise, it assumes you want to quit without running the payload.
This check is case insensitive, so feel free to write `e` or `d` instead.

## Notes

- Major thanks to the people who are working on `orion`. That crate is a huge
lifesaver for cryptography stuff. Go check it out if you're gonna need a
crypto crate in Rust.
