## Build
### Build frontend
```bash
docker build -t frontend -f Dockerfile.frontend .
```
### Run frontend
```bash
docker run -p 8000:8000 -ti frontend
```