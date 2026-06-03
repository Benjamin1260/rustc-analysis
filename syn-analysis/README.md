# Syn-Analysis

## Problem Statement
we want to be able to analyze Rust repositories for things like async/await occurences, specific method calls to asynchronous APIs and collect statistics on this. in order to do this, we need a way to "read" the files in essence. Grep/plaintext strugles because it does not fully understand the structure of code files, like comments, comment blocks... Syn also doesn't understand semantics so this analysis will not be fully accurate, but we see it as a fair baseline for now, sitting as a middle ground between grep, which does not understand Rust at all, and the Rust-Analyzer, which has full semantics understanding but has a steeper learning curve.

## Method
1. open file (what files should we include and exclude? can be decided later)
2. turn said file into AST (Abstract Syntax Tree) using Syn (no `proc_macro2` required here)
3. Parse/analyze said AST