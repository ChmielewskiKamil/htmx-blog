dev-server:
	cargo watch -w src -w templates -w tailwind.config.js -w input.css -x run 

dev-tailwind:
	./tailwindcss -i input.css -o assets/output.css --watch=always
