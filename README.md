<div align=center>
  <a href="https://github.com/sada-L/gvozd/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/sada-L/gvozd.svg?style=for-the-badge"/>
  </a>
  <a href="https://github.com/sada-L/gvozd/network/members">
    <img src="https://img.shields.io/github/forks/sada-L/gvozd.svg?style=for-the-badge"/>
  </a>
  <a href="https://github.com/sada-L/gvozd/stargazers">
    <img src="https://img.shields.io/github/stars/sada-L/gvozd.svg?style=for-the-badge"/>
  </a>
  <a href="https://github.com/sada-L/gvozd/issues">
    <img src="https://img.shields.io/github/issues/sada-L/gvozd.svg?style=for-the-badge"/>
  </a>
  <a href="https://github.com/sada-L/gvozd/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/sada-L/gvozd.svg?style=for-the-badge"/>
  </a>
</div>

<div align="center">
  <a href="https://github.com/sada-L/gvozd">
    <img width="200" height="200" alt="Image" src="https://github.com/user-attachments/assets/de9d01d0-ae0c-4b09-93c7-3c994951ffb0" />
  </a>
<h3 align="center">Gvozd</h3>

  <p align="center">
    A backend service for organizing and storing user credentials.
    <br />
    <a href="https://github.com/sada-L/gvozd">Docs</a>
    |
    <a href="https://github.com/sada-L/gvozd/issues/new?labels=bug&template=bug-report.md">Report Bug</a>
    |
    <a href="https://github.com/sada-L/gvozd/issues/new?labels=enhancement&template=feature-request.md">Request Feature</a>
  </p>

</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#built-with">Built With</a></li>
    <li><a href="#features">Features</a></li>
    <li><a href="#requirements">Requirements</a></li>
    <li><a href="#getting-started">Getting Started</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

### Built With

* [![Rust][Rust-lang]][Rust-url]
* [![Actix][Actix-badge]][Actix-url]
* [![PostgreSQL][Postgres-badge]][Postgres-url]
* [![Docker][Docker-badge]][Docker-url]

## Features
- Blazingly fast server with **actix-web**
- Integrated with **PostgreSQL** using **sqlx**
- Secure JWT authentication ready for extension

---

## Requirements

- **Rust >= 1.87.0**
- **Docker >= 20.10**
- **Docker Compose >= 1.29**
- (Optional) **PostgreSQL >= 13** if running locally outside Docker

---

## Getting Started

<details><summary>Try it with Docker</summary>

1. Clone the repo
   ```sh
   git clone https://github.com/sada-L/gvozd.git
   cd gvozd
   ```

2. Copy `.env.example` to `.env` and set required environment variables.

3. Build and start services (development)
   ```sh
   docker-compose up --build
   ```

### This will:

- Start the Web API
- Start a PostgreSQL container
- Expose the API at `http://localhost:8080`

### HealthCheck:
   ```sh
    curl -X POST http://localhost:8080/api/auth/signup \
      -H "Content-Type: application/json" \
      -d '{ "email": "test@example.com", "password": "SecurePassword123!", "username": "Test User" }'
   ```

</details>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<!-- CONTACT -->
## Contact

Sada - fxdv493@gmail.com

Project Link: [https://github.com/sada-L/gvozd](https://github.com/sada-L/gvozd)

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Rust Lang Book](https://doc.rust-lang.org/book/)
* [Actix Web Docs](https://actix.rs/docs/)
* [sqlx Docs](https://docs.rs/sqlx/latest/sqlx/)

<!-- MARKDOWN LINKS & IMAGES -->
[Rust-lang]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org
[Actix-badge]: https://img.shields.io/badge/Actix-04b6e6?style=for-the-badge
[Actix-url]: https://actix.rs/
[Postgres-badge]: https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white
[Postgres-url]: https://www.postgresql.org/
[Docker-badge]: https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white
[Docker-url]: https://www.docker.com/


