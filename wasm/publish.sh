./build.sh
read -p "Bump the version in js/package.json and press [ENTER] to continue..."
cd js
bun publish
cd ../