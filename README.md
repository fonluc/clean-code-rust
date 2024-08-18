# Clean Rust Code  

![comic](assets/clean-code-comic.jpeg)

Translated from: [https://github.com/fonluc/codigo-limpo-rust](https://github.com/fonluc/codigo-limpo-rust)

This work aims to establish the first clean code guide in Rust, providing a solid foundation for best practices in the language. However, as with any initial work, it may contain code an grammatical errors. Therefore, it is crucial for other developers to get involved and contribute to improving this repository. This will not only enrich the content but also reflect the quality and commitment of Brazilian developers, whether they are specialists in backend, Golang, Rust, web3, or blockchain. Collaboration from everyone is essential to raise the standard of software development and promote excellence in our community.

## Preface: Why Write Clean Code?  
This document serves as a reference for the Rust community, aiming to help developers write cleaner code. Whether working on a personal project or as part of a larger team, writing clean code is an essential skill. Establishing good paradigms and consistent, accessible standards for writing clean code can help prevent developers from spending hours trying to understand their own work (or that of others).

*"We don’t read code, we decode it."* - Peter Seibel

As developers, we are sometimes tempted to write code in a way that is convenient at the moment, without considering best practices; this makes code reviews and testing more difficult. In a way, we are "encoding"—and by doing so, making it harder for others to decode our work. But we want our code to be usable, readable, and maintainable. And that requires coding the right way, not the easy way.

This document starts with a simple and brief introduction to the fundamentals of writing clean code. Later, we will discuss concrete examples of refactoring specific to Rust.

### [Table of Contents](#table-of-contents)
1. [Introduction to Clean Code](#introduction-to-clean-code)  
2. [Test-Driven Development](#test-driven-development)  
3. [Naming Conventions](#naming-conventions)  
4. [Comments](#comments)  
5. [Function Naming](#function-naming)  
6. [Variable Naming](#variable-naming)  
7. [Cleaning Functions](#cleaning-functions)  
8. [Function Size](#function-size)  
9. [Function Signatures](#function-signatures)  
10. [Variable Scope](#variable-scope)  
11. [Variable Declaration](#variable-declaration)  
12. [Clean Rust](#clean-rust)  
13. [Return Values](#return-values)  
14. [Returning Defined Errors](#returning-defined-errors)  
15. [Returning Dynamic Errors](#returning-dynamic-errors)  
16. [References in Rust](#references-in-rust)  
17. [Closures as Function Pointers](#closures-as-function-pointers)  
18. [Traits in Rust](#traits-in-rust)  
19. [The `dyn Any`](#the-dyn-any)  
20. [Summary](#summary)

### Introduction to Clean Code  
Clean code is the pragmatic concept of promoting readable and maintainable software. Clean code builds trust in the codebase and helps minimize the chances of careless bugs being introduced. It also helps developers maintain their agility, which typically declines as the codebase expands due to the increased risk of introducing bugs.

