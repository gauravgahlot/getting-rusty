# lib-loading

An example of loading a dynamic library, and using the function it contains. This is achieved using the [libloading](https://docs.rs/libloading/latest/libloading/) crate.

### Directory structure

```bash
getting-rusty/lib-loading on main [$] via 🦀 v1.69.0
➜ tree . -d
.
├── executor        # Contains the executor crate
│   └── src
└── hello-world     # Contains the hello-world shared library 
    └── src
```

### Dependencies

- libloading (v0.8)

### Build and Run

The project root contains a `Makefile` that can be used to build and run the project.

```bash
# Build the project
make build

# Run the project
make run
```
