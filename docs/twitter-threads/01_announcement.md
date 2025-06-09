# Twitter Thread Draft: litesvm-testing Framework

## Tweet 1/12 - Hook + Team Context

```
🧵 Solana error handling is hard!
Testing on-chain programs is hard!

@gotokamai, we're obsessed with detecting, analyzing and fixing on-chain program errors. Testing is our proactive defense.

We just built something to transform both experiences. A complete educational testing framework for @solana programs.

Here's how: 👇
```

## Tweet 2/12 - Problem Setup (Lead with Error Hierarchy)

```
2/ The current pain:

❌ No clear guidance on error hierarchy
❌ `InstructionError::Custom(1)` - wtf does that mean?
❌ 8+ imports just to test basic functionality
❌ Build system complexity for each framework

Testing should teach you Solana, not fight you.
```

## Tweet 3/12 - Before/After Example

```
3/ Real example - testing insufficient funds:

Before: Manual error code casting, verbose imports, unclear error levels

After: Type-safe, self-documenting, educational

https://github.com/levicook/litesvm-testing/blob/main/crates/litesvm-testing/tests/test_system_error_insufficient_funds.rs
```

## Tweet 4/12 - Solution Showcase + Future Vision

```
4/ Three levels of error testing that match how Solana actually works:

🏗️ Transaction Level: `demand_transaction_error()`
📍 Instruction Level: `demand_instruction_error()`
⚙️ System Program Level: `demand_system_error(SystemError::InsufficientFunds)`

Next: Framework-specific helpers (SPL Token, Anchor errors, etc.) 🎯
```

## Tweet 5/12 - API Styles

```
5/ Choose your API style:

Traditional: `demand_logs_contain("Hello!", result)`
Modern: `result.demand_logs_contain("Hello!")`

Same power, different syntax. Your team's preference.

Plus "surgical precision" with instruction-index targeting for complex transactions.
```

## Tweet 6/12 - Educational Value

```
6/ Built-in educational progression:

📚 Good → Better → Best → Best+ examples
🔍 Real system program errors (not synthetic)
📖 Complete documentation for every function
🎓 Framework comparisons (Anchor vs Pinocchio)

Learn Solana testing patterns while you code.
```

## Tweet 7/12 - Build Integration

```
7/ Seamless build integration:

One line in build.rs → automatic program compilation → embedded in tests

✅ No external dependencies
✅ CI/CD friendly
✅ Changes trigger rebuilds automatically

[Image: Build system diagram or code snippet?]
```

## Tweet 8/12 - Technical Achievements

```
8/ The technical wins:

• 522-line comprehensive API with full docs
• Type-safe SystemError enum integration
• Complete error hierarchy matching Solana runtime
• Dual API paradigms for different preferences
• Real-world error scenarios, not toy examples
```

## Tweet 9/12 - Framework Support

```
9/ Framework support:

⚓ Anchor: Full IDL integration, complete build docs
🌲 Pinocchio: Lightweight compilation, minimal boilerplate
🔜 Steel: Planned support

Each framework gets first-class treatment with educational examples.
```

## Tweet 10/12 - Educational Mission

```
10/ This isn't just a testing library.

It's an educational resource that teaches:
• Solana's error hierarchy
• Testing best practices
• Framework trade-offs
• Build system integration

Learning while you build. 🎓
```

## Tweet 11/12 - Call to Action

```
11/ Try it now:

📦 `git clone https://github.com/levicook/litesvm-testing`
🔬 Check the examples/ directory
📚 Read the API progression tutorial
🚀 `cargo test --workspace --show-output`

Complete working examples for Anchor & Pinocchio included.
```

## Tweet 12/12 - Gratitude + CTA

```
12/ Special thanks to @LiteSVM for the incredible testing runtime that makes this possible.

If you're building on Solana, testing should be educational, not painful.

Star ⭐ if this helps your dev experience!

🔗 https://github.com/levicook/litesvm-testing
```

---

## Questions for Iteration:

1. **Tweet 3**: Link to code or ray.so image? Link is more accessible but image might be more engaging.

2. **Tweet 4**: How much detail on future framework-specific features? Could expand on "typesafe SPL token errors, anchor errors" etc.

3. **Team positioning**: Does tweet 1 capture @gotokamai's mission well? Should we mention specific error analysis work?

4. **Technical depth**: Right balance between technical credibility and accessibility?

5. **Threading**: Any tweets feel too long or should be split?
