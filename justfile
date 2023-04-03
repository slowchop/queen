watch:
    cargo watch --features bevy/dynamic_linking

dev-native:
    cargo run --features bevy/dynamic_linking

dev-web:
    trunk serve

release-itch:
    rm -fr dist
    trunk build --release

    rm -f release.zip
    cd dist && zip -r ../release.zip *
    butler push release.zip slowchop/the-queen:html

release-netlify:
    trunk build --release
    netlify deploy --prod --dir dist

release-build-windows:
    cargo build --release --target x86_64-pc-windows-msvc

