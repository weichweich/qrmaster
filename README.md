# qrmaster

A simple tool for reading QR-Codes from images and creating QR-Codes from data.

It just puts a CLI in font of the [bardecoder](https://github.com/piderman314/bardecoder) and [qrcode-rust](https://github.com/kennytm/qrcode-rust) libraries.

## Read QR-Code from image

```
cargo run -- decode -i image_with_qr_code.png
```

## Write QR-Codes to image

```
echo -n "This is a test"| cargo run -- encode -o image_with_qr_code.png
```
