# Friendev Jury Mode: The "Separation of Powers" in Code Security

In the realm of AI-assisted programming, we often say "trust, but verify." However, when the verifier is also an AI, how do we ensure it doesn't make mistakes? Friendev's answer is: **Don't just ask one AI, ask a "Jury".**

## What is Jury Mode?

**Jury Mode** is Friendev's highest-level security review mechanism. Unlike Shorekeeper mode (single AI review), Jury mode introduces a **consensus mechanism**.

When you enable this mode, for every sensitive operation (such as deleting files, modifying core logic), Friendev will not rely on the judgment of a single model. Instead, it will simultaneously convene **3 independent AI jurors**. They will review the same request in parallel and vote independently. The operation will only be executed if it receives a majority vote of **2/3 or more**.

## Why Do We Need a Jury?

Even the most advanced Large Language Models (LLMs) have the potential for "hallucinations" or errors in judgment. When dealing with extremely high-risk operations, a single model's misjudgment could lead to serious consequences.

Jury mode leverages statistical principles: **The probability of multiple independent models making the same mistake simultaneously is far lower than that of a single model.**

*   **Reducing False Positives**: If one model is overly conservative, the other two might correct it.
*   **Preventing False Negatives**: If one model overlooks a security risk, the other two might catch it.

## How It Works

1.  **Parallel Deliberation**: The system generates 3 independent conversation contexts and sends them to the AI service simultaneously. This is like distributing the case file to three judges who don't communicate with each other.
2.  **Independent Ruling**: Each "juror" receives strict instructions to evaluate based on security, compliance, and stability, and returns `True` (Approve) or `False` (Reject) along with detailed reasoning.
3.  **Majority Vote**: Friendev collects all votes.
    *   3 votes for: **Unanimous Approval**.
    *   2 votes for, 1 against: **Majority Approval** (but the dissenting opinion is recorded in the logs).
    *   1 or 0 votes for: **Vetoed**.
4.  **Transparent Disclosure**: The final output includes not just the result, but also the viewpoint of each juror. You can see why Juror #1 thought it was fine, while Juror #2 voted against it.

## How to Use

Add the `--jury` flag when starting Friendev:

```bash
friendev --jury
```