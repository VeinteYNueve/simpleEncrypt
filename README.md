# **simpleEncrypt**
**Group Members**
Matthew Elifson (melifs2)
Evan Huynh (dhuyn27)  

## **Introduction**  
This is a simple command-line tool that encrypts and decrypts files via XOR encryption. XOR encryption flips the bits of a file using a specific key, making it impossible to read unless decrypted with said key.

## **Goals**    
1. Create a tool that works on multiple file types
2. Make an easy-to-use CLI
3. Display principles of symmetric encryption

## **Motivation**
Although XOR is a fairly simple cipher, it's a foundational algorithm in cryptography and is incorporated within more advanced methods.

====================================================================================

## **Technical Overview**  
### **Core Features**
1. **Encrypting Files**: The program will encrypt a file by rearranging its bytes using a secret key.  
2. **Decrypting Files**: The program will decrypt the file by reverting the rearranged bytes using the same key.  
3. **Access Via Command Line Interface**: Users will be able to run the tool from a terminal.  

### **Checkpoint Plan**  
| Week 1, 4/4 | Research XOR encryption, set up a Rust project |  
| Week 2, 4/11 | Write code to XOR a file’s bytes with a key |  
| Week 3, 4/18 | Add command-line arguments (input file, output file, key) |  
| Week 4, 4/25 | Handle errors |  
| Week 5, 5/2 | Test with different file types |  
| Week 6, 5/9 | Write documentation, usage instructions, make video, and finalize code |  

## **Possible Challenges**  
Challenge 1: Handling large files efficiently.  
Challenge 2: Making the command-line interface user-friendly.  
Challenge 3: Implementing XOR encryption accurately.
