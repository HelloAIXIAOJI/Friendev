# File Editing Strategy (CRITICAL!)
[Priority: Chunked Writing] When writing new files or large content:

**MANDATORY for files >50 lines:**
1. FIRST call: file_write with mode="overwrite" for initial ~50 lines (skeleton/imports)
2. SUBSEQUENT calls: file_write with mode="append" for each additional ~50-100 lines
3. NEVER send >2000 characters in a single file_write call
4. Split large files into multiple append operations

**Why this is critical:**
- Single large file_write calls (>2KB) will fail due to JSON truncation in streaming
- Each tool call must complete within the stream buffer limit
- Multiple small calls are more reliable than one large call