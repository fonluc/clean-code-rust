# Clean Rust Code  

![comic](assets/good code bad code.jpeg)

## Preface: Why Write Clean Code?  
This document serves as a reference for the Rust community, aiming to help developers write cleaner code. Whether working on a personal project or as part of a larger team, writing clean code is an essential skill. Establishing good paradigms and consistent, accessible standards for writing clean code can help prevent developers from spending hours trying to understand their own work (or that of others).

*"We don’t read code, we decode it."* - Peter Seibel

As developers, we are sometimes tempted to write code in a way that is convenient at the moment, without considering best practices; this makes code reviews and testing more difficult. In a way, we are "encoding"—and by doing so, making it harder for others to decode our work. But we want our code to be usable, readable, and maintainable. And that requires coding the right way, not the easy way.

This document starts with a simple and brief introduction to the fundamentals of writing clean code. Later, we will discuss concrete examples of refactoring specific to Rust.

### Table of Contents 
1. Introduction to Clean Code  
2. Test-Driven Development  
3. Naming Conventions  
4. Comments  
5. Function Naming  
6. Variable Naming  
7. Cleaning Functions  
8. Function Size  
9. Function Signatures  
10. Variable Scope  
11. Variable Declaration  
12. Clean Rust  
13. Return Values  
14. Returning Defined Errors  
15. Returning Dynamic Errors  
16. References in Rust  
17. Closures as Function Pointers  
18. Traits in Rust  
19. The `dyn Any`  
20. Summary

### Introduction to Clean Code  
Clean code is the pragmatic concept of promoting readable and maintainable software. Clean code builds trust in the codebase and helps minimize the chances of careless bugs being introduced. It also helps developers maintain their agility, which typically declines as the codebase expands due to the increased risk of introducing bugs.