# RUST API

[RUST API | Learning Rust API with Axum](http://localhost:8787)

## Development

1. **Clone atau Pull Repository**

   ```bash
   git clone git@gitlab.com:berkah-buana-solusindo/rust_api_task.git
   cd rust_api_task
   ```

2. **Konfigurasi Environment Variables**

   - **Linux/Mac**:

     ```bash
     cp .env.example .env
     ```

   - **Windows**:

     ```bash
     copy .env.example .env
     ```

3. **Build Docker Image** _(opsional jika belum pernah membangun image, sesuaikan dengan OS yang digunakan)_

   - **Linux/Mac**:
     ```bash
     DOCKER_BUILDKIT=1 docker build -t https://github.com/riz-it/rust_api_task.git .
     ```
   - **Windows**:
     ```bash
     docker build -t https://github.com/riz-it/rust_api_task.git .
     ```

4. **Jalankan Container**

   ```bash
   docker run --env-file .env --name rust_api_task -d -p 8787:8787 https://github.com/riz-it/rust_api_task.git
   ```

5. **Mulai Pengembangan**

   - Setelah container berjalan, akses aplikasi melalui [http://localhost:8787](http://localhost:8787)

## Cleaning up Docker

- **Hentikan dan Hapus Container tracer** _(sesuaikan dengan OS yang digunakan)_

  - **Linux/Mac**:
    ```bash
    docker stop rust_api_task || true && docker rm rust_api_task || true
    ```
  - **Windows**:
    ```bash
    docker stop rust_api_task
    docker rm rust_api_task
    ```

- **Hapus semua container**

  ```bash
  docker rm -vf $(docker ps -aq)
  ```

- **Menghapus Semua Container, Image, Jaringan, dan Volume**

  ```bash
  docker system prune -a --volumes -f
  ```
