## Steps

1.  Clone the repository:
```bash
    git clone https://github.com/VeinteYNueve/simpleEncrypt.git
    ```

2. Change your current directory to the newly cloned folder:
    ```bash
    cd simpleEncrypt
    ```

3.  Prepare your input file
    - If you have a file you want to encrypt/decrypt, place a copy of it inside the `simpleEncrypt` directory you just navigated into.
    - When the program asks for the filename, you can just type the document's name instead of a full path.

4.  Run the project:
    
    ```bash
    cargo run
    ```

This should start the XOR encrypter application in your terminal.
* When prompted for the "name of the file to encrypt/decrypt," if you placed your file in the `simpleEncrypt` directory, you may just type its name. Otherwise, you'll need to provide the full path to the file.