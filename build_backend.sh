cd backend
read -p "Do you want to add the --release tag on build? (y/n): " choice

if [ "$choice" == "y" ]; then
    echo "Building backend with --release tag."
    cargo build --release
else
    echo "Building backend without --release tag."
    cargo build
fi

if [ $? -eq 0 ]; then
    echo "Build finished successfully."
else
    echo "Build failed."
fi