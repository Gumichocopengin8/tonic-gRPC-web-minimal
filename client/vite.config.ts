import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    // https://github.com/grpc/grpc-web/issues/1242#issuecomment-1815910705
    preserveSymlinks: true,
  },
});
