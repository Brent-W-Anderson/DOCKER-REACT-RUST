import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig( {
    plugins: [react()],
    server: {
        host: "0.0.0.0", // Bind to all interfaces
        port: 3000,      // Ensure it's listening on port 3000
    },
} )
