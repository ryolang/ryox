## Get Started with Ryo

Ready to experience productive, safe, and fast development? Follow these simple steps to write and run your first Ryo program.

**1. Installation**

*(Note: As Ryo is hypothetical, provide instructions based on your planned distribution method. Below is a common example using a package manager or download script).*

**Using `ryoinstall` (Recommended):**

The easiest way to get Ryo, including the compiler, standard library, and the `ryo` package manager included, is via our installation script.

```bash
# Linux / macOS / WSL
curl --proto '=https' --tlsv1.2 -sSf https://ryo-lang.org/install.sh | sh

# Windows (PowerShell)
# Invoke-RestMethod https://ryo-lang.org/install.ps1 | Invoke-Expression
```

Follow the on-screen instructions to add Ryo to your system's PATH. Verify the installation:

```bash
ryo --version
```

**(Alternative methods like downloading binaries, using system package managers (Homebrew, apt), or building from source would be listed here).**

**2. Your First Ryo Program: Hello, World!**

Let's create a classic "Hello, World!" application.

*   **Initialize the Project:** Use the `ryo` tool to create the basic project structure and manifest file (`ryo.toml`).
    ```bash
    ryo new hello_ryo
    ```

*   **Write the Code:** Open the `src/main.ryo` file created by `ryo new` and replace its contents with the following:

    ```ryo
    # src/main.ryo

    # The main entry point for the application
    fn main() {
        # Use print from the built-ins
        print("Hello, Ryo!")

        name = "Developer"
        # Use an f-string for formatted output
        print(f"Welcome, {name}!")
    }
    ```
    *Remember: Ryo uses **tabs only** for indentation!*

**3. Run Your Program**

Now, use `ryo` to compile and run your code:

```bash
ryo run
```

You should see the following output in your terminal:

```
Hello, Ryo!
Welcome, Developer!
```

**Congratulations!** You've just built and run your first Ryo application.

**4. Next Steps**

You're ready to explore more!

*   **Try the Ryo REPL:** Run `ryo` in your terminal for an interactive session.
*   **Explore the Syntax:** Check out the [Syntax Guide](link-to-syntax-guide.html) for details on variables, types, control flow, and more.
*   **Learn about Memory Safety:** Understand Ryo's unique "Ownership Lite" model in the [Memory Management Guide](link-to-memory-guide.html).
*   **Build Something:** Try modifying the "Hello, World!" example or start a small project using `ryo new my_project`.
*   **Standard Library:** Browse the [Standard Library Documentation](link-to-stdlib-docs.html) to see available modules like `net.http`, `encoding.json`, `os`, etc.

Welcome to the Ryo community! We're excited to see what you build.