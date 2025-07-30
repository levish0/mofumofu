## Project Dependency Installation and Management
### 1. Install Only Essential Dependencies

Use this if you only want to install the **essential dependencies** required to run the project. These are the packages defined in the `[project].dependencies` section of your `pyproject.toml` file. This excludes development and testing tools (like **Ruff** or code formatters).

```bash
uv pip install .
```

This command sets up the lightest possible environment to get your project running.

### 2. Install All Dependencies (Including Development Dependencies)

Use this when you want to install **all project dependencies**, including those needed for development. This covers both the essential dependencies from `[project].dependencies` and the development and testing tools (like **Ruff**) defined under `[tool.uv].dev-dependencies` in your `pyproject.toml` file.

```bash
uv sync
```

The `uv sync` command synchronizes and installs all dependencies specified in `pyproject.toml` according to the `uv.lock` file. This method is useful for setting up a complete development environment.

### Which command should you use?

* **To simply run the project or set up a deployment environment:** Use `uv pip install .`
* **To develop, test, or perform code checks on the project:** Use `uv sync`

---

## Running the FastAPI Server

To start the FastAPI server locally and make it accessible on all network interfaces at port **7000**, use the following command:

```bash
uvicorn app.main:app --host 0.0.0.0 --port 7000
```

---

## Using Ruff

**Ruff** is an fast Python linter and code formatter. Once you've installed your development dependencies (using `uv sync`), you can use Ruff to check and format your code.

### Check for linting errors and formatting issues:

```bash
ruff check .
```

This command will analyze your entire project (current directory `.` ) for any linting errors or style violations according to your `pyproject.toml` configuration.

### Automatically fix fixable issues:

```bash
ruff check . --fix
```

This command will not only check for issues but also automatically fix any problems that Ruff knows how to correct, such as formatting inconsistencies or simple linting errors.

### Format your code:

```bash
ruff format .
```

Use this command to automatically reformat your code according to Ruff's default or configured style. This ensures consistent code style across your project.
