echo "Select an option:"
echo "1. Build frontend"
echo "2. Build backend"
echo "3. Build both"
cd frontend

read -p "Enter your choice (1/2/3): " choice

case $choice in
    1)
        echo "Building frontend..."
        npm run build
        ;;
    2)
        echo "Building backend..."
        npm run build:wasm
        ;;
    3)
        echo "Building both frontend and backend..."
        npm run build:wasm
        npm run build
        ;;
    *)
        echo "Invalid choice. Please select 1, 2, or 3."
        ;;
esac