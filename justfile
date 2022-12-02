export VERSION := `cargo metadata --no-deps -q --format-version=1 | grep -Eo '"version":"[0-9].[0-9].[0-9]"' | grep -Eo '[0-9].[0-9].[0-9]'`

build-linux:
    cargo build --release --target x86_64-unknown-linux-gnu

build-windows:
    cargo rustc --release --target x86_64-pc-windows-gnu -- -Clink-args="-Wl,--subsystem,windows"

[linux]
install: build-linux mk-desktop
    cp ./target/x86_64-unknown-linux-gnu/release/vidyalog ~/.local/bin/
    mv ./target/pack/vidyalog.desktop ~/.local/share/applications/
    @echo Installation complete

[linux]
remove:
    rm ~/.local/bin/vidyalog
    rm ~/.local/share/applications/vidyalog.desktop
    @echo Removal complete.
    @echo You may want to call purge to remove user data

[linux]
purge:
    #!/usr/bin/env bash
    if [ -f ~/.local/bin/vidyalog ]; then
        rm ~/.local/bin/vidyalog
    fi
    if [ -f ~/.local/share/applications/vidyalog.desktop ]; then
        rm ~/.local/share/applications.vidyalog.desktop
    fi
    if [ -d ~/.local/share/vidyalog ]; then
        rm -rf ~/.local/share/vidyalog
    fi
    if [ -d ~/.config/vidyalog ]; then
        rm -rf ~/.config/vidyalog
    fi
    if [ -d ~/.cache/vidyalog ]; then
        rm -rf ~/.cache/vidyalog
    fi
    echo Purge complete

pack-all: pack-zip pack-tar pack-deb
    @echo Packing everything completed

pack-zip: build-windows
    #!/usr/bin/env bash
    mkdir -p ./target/pack/Vidyalog
    cp ./target/x86_64-pc-windows-gnu/release/vidyalog.exe ./target/pack/Vidyalog/Vidyalog.exe
    cd target/pack/
    rm Vidyalog.zip
    zip -r Vidyalog Vidyalog
    rm -r Vidyalog/
    echo Packing Zip Complete

mk-desktop:
    #!/usr/bin/env bash
    mkdir -p ./target/pack
    cd ./target/pack
    echo '[Desktop Entry]' > vidyalog.desktop
    echo 'Type=Application' >> vidyalog.desktop
    echo 'Name=Vidyalog' >> vidyalog.desktop
    echo 'Comment=Manage and view Youtube playlists' >> vidyalog.desktop
    echo 'Exec=vidyalog' >> vidyalog.desktop
    echo 'Terminal=false' >> vidyalog.desktop
    echo 'Categories=Network' >> vidyalog.desktop

pack-tar: build-linux
    #!/usr/bin/env bash
    just mk-desktop
    mkdir -p ./target/pack/vidyalog
    cp ./target/x86_64-unknown-linux-gnu/release/vidyalog ./target/pack/vidyalog/
    mv ./target/pack/vidyalog.desktop ./target/pack/vidyalog/
    cd target/pack/vidyalog
    echo '#!/usr/bin/bash' >> install.sh
    echo 'if [ -z ${DESTDIR+unset} ]; then' >> install.sh
    echo '    echo Automatically determining installation path, set DESTDIR variable to determine where to put the executable file' >> install.sh
    echo '    echo Installing to .local/bin' >> install.sh
    echo '    cp ./vidyalog ~/.local/bin/' >> install.sh
    echo 'else' >> install.sh
    echo '    echo Installing to $DESTDIR' >> install.sh
    echo '    cp ./vidyalog $DESTDIR' >> install.sh
    echo 'fi' >> install.sh
    echo 'cp ./vidyalog.desktop ~/.local/share/applications' >> install.sh
    echo "echo Installation completed for local user" >> install.sh
    echo '#!/usr/bin/bash' >> remove.sh
    echo 'AB=$(which vidyalog)' >> remove.sh
    echo 'if [ $? -eq 0 ]; then' >> remove.sh
    echo '    echo Removed executable' >> remove.sh
    echo '    rm $AB' >> remove.sh
    echo 'fi' >> remove.sh
    echo 'if [ -f $HOME/.local/share/applications/vidyalog.desktop ]; then' >> remove.sh
    echo '    echo Removed desktop entry' >> remove.sh
    echo '    rm $HOME/.local/share/applications/vidyalog.desktop' >> remove.sh
    echo 'fi' >> remove.sh
    cp remove.sh purge.sh
    echo 'echo Removal complete, the data have been preserved, use purge.sh to remove everything.' >> remove.sh
    echo 'if [ -d $HOME/.local/share/vidyalog ]; then' >> purge.sh
    echo '    rm -rf $HOME/.local/share/vidyalog' >> purge.sh
    echo '    echo Removed data' >> purge.sh
    echo 'fi' >> purge.sh
    echo 'if [ -d $HOME/.config/vidyalog ]; then' >> purge.sh
    echo '    rm -rf $HOME/.config/vidyalog' >> purge.sh
    echo '    echo Removed config' >> purge.sh
    echo 'fi' >> purge.sh
    echo 'if [ -d $HOME/.cache/vidyalog ]; then' >> purge.sh
    echo '    rm -rf $HOME/.cache/vidyalog' >> purge.sh
    echo '    echo Purged the cache folder' >> purge.sh
    echo 'fi' >> purge.sh
    chmod 755 install.sh
    chmod 755 remove.sh
    chmod 755 purge.sh
    cd ..
    tar -caf ./vidyalog.tar.gz ./vidyalog
    rm -r vidyalog/
    echo Packing Tar Complete

pack-deb: build-linux
    #!/usr/bin/env bash
    VERSION_MAJOR=$(echo $VERSION | sed 's/.[0-9]$//')
    VERSION_MINOR=$(echo $VERSION | sed 's/[0-9].[0-9].//')
    TARGET_FOLDER=vidyalog_$VERSION_MAJOR-$VERSION_MINOR

    just mk-desktop
    mkdir -p ./target/pack/$TARGET_FOLDER/usr/bin
    mkdir -p ./target/pack/$TARGET_FOLDER/usr/share/applications

    cp ./target/x86_64-unknown-linux-gnu/release/vidyalog ./target/pack/$TARGET_FOLDER/usr/bin/
    mv ./target/pack/vidyalog.desktop ./target/pack/$TARGET_FOLDER/usr/share/applications/
    cd ./target/pack/$TARGET_FOLDER
    mkdir DEBIAN
    cd DEBIAN
    echo Package: vidyalog >> control
    echo Version: $VERSION_MAJOR-$VERSION_MINOR >> control
    echo Section: web >> control
    echo Priority: optional >> control
    echo Architecture: amd64 >> control
    echo "Maintainer: Purrie Brightstar <purriestarshine@gmail.com>" >> control
    echo "Homepage: https://github.com/purrie/Vidyalog" >> control
    echo Description: Video playlist manager >> control

    cp ../../../../LICENSE ./copyright
    if [ -f ../../changelog ]; then
        cp ../../changelog ./changelog
    else
        echo No changelog found in root/target/pack folder, skipping inclusion
    fi

    cd ../..
    dpkg-deb --build $TARGET_FOLDER
    rm -r $TARGET_FOLDER
    echo Packing Deb Complete

clear:
    rm -rf ./target
