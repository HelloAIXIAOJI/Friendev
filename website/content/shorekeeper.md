# Friendev Shorekeeper: Intelligent Security Check for Your AI Programming Assistant

In the era of AI-assisted programming, we often face a dilemma: should we manually review every line of AI-generated code to ensure security, or should we approve all changes to maintain speed? Friendev's Shorekeeper mode is the intelligent solution designed to address this very problem.

## What is Shorekeeper?

Shorekeeper is an advanced security review mechanism built into Friendev. Its core concept is "AI reviewing AI."

When you enable this mode, Friendev no longer relies on human users to mechanically approve every file modification or command execution request. Instead, it assigns a dedicated AI role—the Shorekeeper—to evaluate the risk of each operation in real-time. It acts like an tireless code audit expert, constantly safeguarding your codebase security.

## How Does It Work?

The Shorekeeper workflow is fully automated and transparent:

- **Interception and Analysis**: When Friendev is about to execute sensitive operations (such as rewriting source code, deleting files, or executing shell commands), Shorekeeper immediately intercepts the request.
- **Context Evaluation**: The system packages the operation type, target file, and specific code change preview (Diff) to send to the review model.
- **Risk Assessment**:
  - Shorekeeper conducts deep analysis of the changes based on preset security criteria (prioritizing security, data integrity, compliance, and stability).
    - If the change is safe (e.g., adding logs, fixing obvious bugs, creating new modules), it automatically approves and the process continues.
    - If the change poses risks (e.g., deleting core logic, hardcoding sensitive information, destructive configurations), it firmly rejects and provides specific rejection reasons.

## Core Advantages

1. **Intelligent Risk Identification**

Unlike simple rule matching, Shorekeeper has the ability to understand code semantics. It not only understands what the code "writes," but also what it "means." This means it can identify operations that are syntactically correct but logically dangerous.

2. **Zero-Friction Development Experience**

In most conventional development scenarios, Shorekeeper works silently in the background. You no longer need to interrupt your thought process due to frequent "Are you sure you want to write?" prompts. It only intervenes when truly necessary vigilance is required.

3. **Independent Model Configuration**

To ensure the objectivity and efficiency of the review, Friendev allows you to specify a dedicated AI model for Shorekeeper.

---
**Main Model**: Responsible for writing code, focusing on creativity and logic.

**Shorekeeper Model**: Responsible for reviewing code, focusing on security and rigor. You can configure a model with stronger logical reasoning capabilities specifically for oversight, or use a faster model to speed up the approval process.

## How to Enable

Simply add the --shorekeeper parameter when starting Friendev to activate this mode:
```
friendev --shorekeeper
```

If you want to specify a dedicated model for Shorekeeper, you can use the following command during your session:
```
/model sk
```