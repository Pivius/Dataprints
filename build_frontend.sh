cd frontend
echo "Building frontend."
npm run build

if [ $? -eq 0 ]; then
    echo "Build finished successfully."
else
    echo "Build failed."
fi